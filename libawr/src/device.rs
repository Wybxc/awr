//! 解析和生成 `device.json`。

use std::{backtrace::Backtrace, string::FromUtf8Error};

use hex::FromHexError;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use ricq::{device::OSVersion, Device};
use serde_json::{Map, Value};
use thiserror::Error;

box_error_impl!(DeviceError, DeviceErrorImpl, "设备信息文件错误。");

/// 设备信息文件错误。
#[derive(Error, Debug)]
enum DeviceErrorImpl {
    /// Json 解析错误。
    #[error("Json 解析错误")]
    JsonParseError {
        /// 原始错误。
        source: serde_json::Error,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// Json 序列化错误。
    #[error("Json 序列化错误")]
    JsonDumpError {
        /// 原始错误。
        source: serde_json::Error,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// Json 格式错误。
    #[error("Json 格式错误")]
    JsonSchemaError {
        /// 错误信息。
        message: String,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// 设备信息版本不支持。
    #[error("设备信息版本不支持: {version}")]
    UnsupportedVersion {
        /// 版本号。
        version: i64,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// UTF-8 解码错误。
    #[error("UTF-8 解码错误")]
    FromUtf8Error {
        /// 原始错误。
        #[from]
        source: FromUtf8Error,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// 十六进制解码错误。
    #[error("十六进制解码错误")]
    FromHexError {
        /// 原始错误。
        #[from]
        source: FromHexError,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// 整数溢出。
    #[error("整数溢出")]
    TryFromIntError {
        /// 原始错误。
        #[from]
        source: std::num::TryFromIntError,
        /// 错误堆栈。
        backtrace: Backtrace,
    },
}

impl DeviceErrorImpl {
    fn from_parse_err(err: serde_json::Error) -> Self {
        Self::JsonParseError {
            source: err,
            backtrace: Backtrace::capture(),
        }
    }

    fn from_dump_err(err: serde_json::Error) -> Self {
        Self::JsonDumpError {
            source: err,
            backtrace: Backtrace::capture(),
        }
    }
}

macro_rules! schema_err {
    ($msg:literal $(,)?) => {
        DeviceErrorImpl::JsonSchemaError {
            message: String::from($msg),
            backtrace: Backtrace::capture(),
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        DeviceErrorImpl::JsonSchemaError {
            message: format!($fmt, $($arg)*),
            backtrace: Backtrace::capture(),
        }
    };
}

type Result<T> = std::result::Result<T, DeviceError>;

macro_rules! parse_batch {
    ($version:ty, $json:ident, $fallback:ident, $($key:expr => $name:ident,)*) => {
        Device {
            $($name: <$version>::parse($json, $key, || $fallback.$name.clone())?,)*
        }
    };
}

macro_rules! parse {
    ($version:ty, $json:ident, $fallback:ident) => {
        parse_batch!($version, $json, $fallback,
            "display" => display,
            "product" => product,
            "device" => device,
            "board" => board,
            "model" => model,
            "fingerprint" => finger_print,
            "bootId" => boot_id,
            "procVersion" => proc_version,
            "imei" => imei,
            "brand" => brand,
            "bootloader" => bootloader,
            "baseBand" => base_band,
            "version" => version,
            "simInfo" => sim_info,
            "osType" => os_type,
            "macAddress" => mac_address,
            "ipAddress" => ip_address,
            "wifiBSSID" => wifi_bssid,
            "wifiSSID" => wifi_ssid,
            "imsiMd5" => imsi_md5,
            "androidId" => android_id,
            "apn" => apn,
            "vendorName" => vendor_name,
            "vendorOsName" => vendor_os_name,
        )
    }
}

/// 从 `device.json` 中读取设备信息。
///
/// `device.json` 采用 **mirai 的格式**，与 ricq 的直接定义不兼容。
///
/// # Arguments
/// - `json` - `device.json` 的内容。
/// - `fallback` - 某一项不存在时的默认值。
pub(crate) fn from_json(json: &str, fallback: &Device) -> Result<Device> {
    let json: Value = serde_json::from_str(json).map_err(DeviceErrorImpl::from_parse_err)?;
    let json = json
        .as_object()
        .ok_or_else(|| schema_err!("根对象不是 `Object`"))?;
    // 查看版本
    let version = json
        .get("deviceInfoVersion")
        .map(|v| v.as_i64().unwrap_or(-1))
        .unwrap_or(1);
    match version {
        1 => {
            // 版本1：字符串全部使用 UTF-8 字节数组表示，MD5 使用字节数组表示
            Ok(parse!(V1, json, fallback))
        }
        2 => {
            // 版本2：字符串直接储存，MD5 使用十六进制表示
            let json = json
                .get("data")
                .and_then(|v| v.as_object())
                .ok_or_else(|| schema_err!("未找到 `data` 字段"))?;
            Ok(parse!(V2, json, fallback))
        }
        _ => Err(DeviceErrorImpl::UnsupportedVersion {
            version,
            backtrace: Backtrace::capture(),
        }
        .into()),
    }
}

/// 以 QQ 号为种子生成随机的设备信息。
pub(crate) fn random_from_uin(uin: i64) -> Device {
    let mut seed = ChaCha8Rng::seed_from_u64(uin as u64);
    Device::random_with_rng(&mut seed)
}

macro_rules! dump_batch {
    ($json:ident, $device:ident, $($key:expr => $name:ident,)*) => {
        $($json.insert($key.to_string(), V2::dump(&$device.$name));)*
    };
}

macro_rules! dump {
    ($json:ident, $device:ident) => {
        dump_batch!($json, $device,
            "display" => display,
            "product" => product,
            "device" => device,
            "board" => board,
            "model" => model,
            "fingerprint" => finger_print,
            "bootId" => boot_id,
            "procVersion" => proc_version,
            "imei" => imei,
            "brand" => brand,
            "bootloader" => bootloader,
            "baseBand" => base_band,
            "version" => version,
            "simInfo" => sim_info,
            "osType" => os_type,
            "macAddress" => mac_address,
            "ipAddress" => ip_address,
            "wifiBSSID" => wifi_bssid,
            "wifiSSID" => wifi_ssid,
            "imsiMd5" => imsi_md5,
            "androidId" => android_id,
            "apn" => apn,
            "vendorName" => vendor_name,
            "vendorOsName" => vendor_os_name,
        )
    }
}

/// 将设备信息写入 `device.json`。
pub(crate) fn to_json(device: &Device) -> Result<String> {
    let mut json = Map::new();
    json.insert("deviceInfoVersion".into(), Value::Number(2.into()));
    json.insert("data".into(), {
        let mut json = Map::new();
        dump!(json, device);
        json.into()
    });
    Ok(serde_json::to_string_pretty(&json).map_err(DeviceErrorImpl::from_dump_err)?)
}

trait Parse<T> {
    fn parse(json: &Map<String, Value>, key: &str, fallback: impl FnOnce() -> T) -> Result<T>;
}

struct V1;

impl Parse<String> for V1 {
    fn parse(
        json: &Map<String, Value>,
        key: &str,
        fallback: impl FnOnce() -> String,
    ) -> Result<String> {
        json.get(key)
            .map(|v| -> Result<String> {
                if let Some(s) = v.as_str() {
                    return Ok(s.to_string());
                }
                let bytes = v
                    .as_array()
                    .ok_or_else(|| schema_err!("`{}` 格式错误", key))?
                    .iter()
                    .map(|b| b.as_i64())
                    .collect::<Option<Vec<i64>>>()
                    .ok_or_else(|| schema_err!("`{}` 格式错误", key))?
                    .iter()
                    .map(|b| b.to_le_bytes()[0])
                    .collect::<Vec<u8>>();
                Ok(String::from_utf8(bytes)?)
            })
            .unwrap_or_else(|| Ok(fallback()))
    }
}

impl Parse<Vec<u8>> for V1 {
    fn parse(
        json: &Map<String, Value>,
        key: &str,
        fallback: impl FnOnce() -> Vec<u8>,
    ) -> Result<Vec<u8>> {
        json.get(key)
            .map(|v| -> Result<Vec<u8>> {
                let bytes = v
                    .as_array()
                    .ok_or_else(|| schema_err!("`{}` 格式错误", key))?
                    .iter()
                    .map(|b| b.as_i64())
                    .collect::<Option<Vec<i64>>>()
                    .ok_or_else(|| schema_err!("`{}` 格式错误", key))?
                    .iter()
                    .map(|b| b.to_le_bytes()[0])
                    .collect::<Vec<u8>>();
                Ok(bytes)
            })
            .unwrap_or_else(|| Ok(fallback()))
    }
}

impl Parse<u32> for V1 {
    fn parse(json: &Map<String, Value>, key: &str, fallback: impl FnOnce() -> u32) -> Result<u32> {
        json.get(key)
            .map(|v| -> Result<u32> {
                let value = v
                    .as_i64()
                    .ok_or_else(|| schema_err!("`{}` 格式错误", key))?;
                Ok(value as u32)
            })
            .unwrap_or_else(|| Ok(fallback()))
    }
}

impl Parse<OSVersion> for V1 {
    fn parse(
        json: &Map<String, Value>,
        key: &str,
        fallback: impl FnOnce() -> OSVersion,
    ) -> Result<OSVersion> {
        let version = json
            .get(key)
            .and_then(|v| v.as_object())
            .ok_or_else(|| schema_err!("`{}` 格式错误", key))?;
        let fallback = fallback();
        let incremental = V1::parse(version, "incremental", || fallback.incremental)?;
        let release = V1::parse(version, "release", || fallback.release)?;
        let codename = V1::parse(version, "codename", || fallback.codename)?;
        let sdk = V1::parse(version, "sdk", || fallback.sdk)?;
        Ok(OSVersion {
            incremental,
            release,
            codename,
            sdk,
        })
    }
}

struct V2;

impl Parse<String> for V2 {
    fn parse(
        json: &Map<String, Value>,
        key: &str,
        fallback: impl FnOnce() -> String,
    ) -> Result<String> {
        json.get(key)
            .map(|v| -> Result<String> {
                Ok(v.as_str()
                    .ok_or_else(|| schema_err!("`{}` 格式错误", key))?
                    .to_string())
            })
            .unwrap_or_else(|| Ok(fallback()))
    }
}

impl Parse<Vec<u8>> for V2 {
    fn parse(
        json: &Map<String, Value>,
        key: &str,
        fallback: impl FnOnce() -> Vec<u8>,
    ) -> Result<Vec<u8>> {
        json.get(key)
            .map(|v| -> Result<Vec<u8>> {
                let hex = v
                    .as_str()
                    .ok_or_else(|| schema_err!("`{}` 格式错误", key))?;
                Ok(hex::decode(hex)?)
            })
            .unwrap_or_else(|| Ok(fallback()))
    }
}

impl Parse<u32> for V2 {
    fn parse(json: &Map<String, Value>, key: &str, fallback: impl FnOnce() -> u32) -> Result<u32> {
        json.get(key)
            .map(|v| -> Result<u32> {
                let value = v
                    .as_i64()
                    .ok_or_else(|| schema_err!("`{}` 格式错误", key))?;
                Ok(value.try_into()?)
            })
            .unwrap_or_else(|| Ok(fallback()))
    }
}

impl Parse<OSVersion> for V2 {
    fn parse(
        json: &Map<String, Value>,
        key: &str,
        fallback: impl FnOnce() -> OSVersion,
    ) -> Result<OSVersion> {
        let version = json
            .get(key)
            .and_then(|v| v.as_object())
            .ok_or_else(|| schema_err!("`{}` 格式错误", key))?;
        let fallback = fallback();
        let incremental = V2::parse(version, "incremental", || fallback.incremental)?;
        let release = V2::parse(version, "release", || fallback.release)?;
        let codename = V2::parse(version, "codename", || fallback.codename)?;
        let sdk = V2::parse(version, "sdk", || fallback.sdk)?;
        Ok(OSVersion {
            incremental,
            release,
            codename,
            sdk,
        })
    }
}

trait Dump<T> {
    fn dump(value: &T) -> Value;
}

impl Dump<String> for V2 {
    fn dump(value: &String) -> Value {
        value.to_string().into()
    }
}

impl Dump<Vec<u8>> for V2 {
    fn dump(value: &Vec<u8>) -> Value {
        hex::encode(value).into()
    }
}

impl Dump<u32> for V2 {
    fn dump(value: &u32) -> Value {
        (*value as u64).into()
    }
}

impl Dump<OSVersion> for V2 {
    fn dump(value: &OSVersion) -> Value {
        let mut map = Map::new();
        map.insert("incremental".to_string(), V2::dump(&value.incremental));
        map.insert("release".to_string(), V2::dump(&value.release));
        map.insert("codename".to_string(), V2::dump(&value.codename));
        map.insert("sdk".to_string(), V2::dump(&value.sdk));
        map.into()
    }
}
