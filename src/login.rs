use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use bytes::Bytes;
use pyo3::prelude::*;

use anyhow::{anyhow, bail, Result};
use futures_util::StreamExt;
use ricq::{
    client::{Connector, DefaultConnector, NetworkStatus, Token},
    ext::{
        common::after_login,
        reconnect::{fast_login, Credential},
    },
    handler::DefaultHandler,
    version::get_version,
    Client, Device, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginSuccess,
    LoginUnknownStatus, Protocol, QRCodeConfirmed, QRCodeImageFetch, QRCodeState,
};
use tokio::{task::JoinHandle, time::sleep};
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::utils::{py_future, retry};

/// 登录方式。
#[pyclass(subclass)]
pub struct LoginMethod {
    protocol: Protocol,
}

#[pymethods]
impl LoginMethod {
    /// 构造登录方式。
    ///
    /// # Arguments
    /// - `protocol` - 客户端协议。
    #[new]
    #[args(protocol = "\"ipad\".to_string()")]
    fn new(mut protocol: String) -> PyResult<Self> {
        protocol.make_ascii_lowercase();
        let protocol = match protocol.as_str() {
            "ipad" => Protocol::IPad,
            "android" | "android_phone" => Protocol::AndroidPhone,
            "watch" | "android_watch" => Protocol::AndroidWatch,
            "mac" | "macos" => Protocol::MacOS,
            "qidian" => Protocol::QiDian,
            _ => Err(anyhow!("不支持的协议"))?,
        };
        Ok(Self { protocol })
    }

    /// 登录到指定的账号。
    ///
    /// # Arguments
    /// - `uin` - 用户的 QQ 号。
    /// - `data_folder` - 数据目录。
    fn login<'py>(&self, uin: i64, data_folder: PathBuf) -> PyResult<&'py PyAny> {
        let _ = (uin, data_folder);
        Err(anyhow!("未实现"))?
    }
}

/// 密码登录。
#[pyclass(extends=LoginMethod)]
pub struct Password {
    password: String,
    md5: bool,
}

#[pymethods]
impl Password {
    /// 构造密码登录方式。
    ///
    /// # Arguments
    /// - `password` - 密码。
    /// - `protocol` - 客户端协议。
    /// - `md5` - 是否用密码的 MD5 代替密码。
    #[new]
    #[args(protocol = "\"ipad\".to_string()", md5 = "false")]
    fn new(password: String, protocol: String, md5: bool) -> PyResult<PyClassInitializer<Self>> {
        Ok(PyClassInitializer::from(LoginMethod::new(protocol)?)
            .add_subclass(Self { password, md5 }))
    }

    /// 登录到指定的账号。
    ///
    /// # Arguments
    /// - `uin` - 用户的 QQ 号。
    /// - `data_folder` - 数据目录。
    fn login<'py>(
        self_: PyRef<'py, Self>,
        py: Python<'py>,
        uin: i64,
        mut data_folder: PathBuf,
    ) -> PyResult<&'py PyAny> {
        let protocol = self_.as_ref().protocol.clone();
        let password = self_.password.clone();
        let md5 = self_.md5;
        py_future(py, async move {
            data_folder.push(uin.to_string());
            tokio::fs::create_dir_all(&data_folder).await?;

            let device = load_device_json(uin, data_folder.clone()).await?;
            let (client, alive) = prepare_client(device, protocol).await?;

            if !try_token_login(&client, data_folder.clone()).await? {
                password_login(&client, uin, password, md5).await?;
            }

            // 注册客户端，启动心跳。
            after_login(&client).await;
            save_token(&client, data_folder.clone()).await?;
            Ok(crate::client::Client::new(client, alive, data_folder).await)
        })
    }
}

/// 二维码登录（仅支持手表协议）。
#[pyclass(extends=LoginMethod)]
pub struct QrCode {}

#[pymethods]
impl QrCode {
    /// 构造二维码登录方式。
    ///
    /// # Arguments
    /// - `protocol` - 客户端协议。
    #[new]
    fn new() -> PyResult<PyClassInitializer<Self>> {
        Ok(PyClassInitializer::from(LoginMethod::new("watch".to_string())?).add_subclass(Self {}))
    }

    /// 登录到指定的账号。
    ///
    /// # Arguments
    /// - `uin` - 用户的 QQ 号。
    /// - `data_folder` - 数据目录。
    fn login<'py>(
        self_: PyRef<'py, Self>,
        py: Python<'py>,
        uin: i64,
        mut data_folder: PathBuf,
    ) -> PyResult<&'py PyAny> {
        let protocol = self_.as_ref().protocol.clone();
        py_future(py, async move {
            data_folder.push(uin.to_string());
            tokio::fs::create_dir_all(&data_folder).await?;

            let device = load_device_json(uin, data_folder.clone()).await?;
            let (client, alive) = prepare_client(device, protocol).await?;

            if !try_token_login(&client, data_folder.clone()).await? {
                qrcode_login(&client, uin).await?;
            }

            // 注册客户端，启动心跳。
            after_login(&client).await;
            save_token(&client, data_folder.clone()).await?;
            Ok(crate::client::Client::new(client, alive, data_folder).await)
        })
    }
}

/// 运行时选择登录方式。
#[pyclass(extends=LoginMethod)]
pub struct Dynamic {
    protocol_override: bool,
}

#[pymethods]
impl Dynamic {
    /// 构造动态登录方式。
    ///
    /// # Arguments
    /// - `protocol` - 客户端协议（可选）。
    #[new]
    #[args(protocol = "None")]
    fn new(protocol: Option<String>) -> PyResult<PyClassInitializer<Self>> {
        Ok(if let Some(protocol) = protocol {
            PyClassInitializer::from(LoginMethod::new(protocol)?).add_subclass(Self {
                protocol_override: false,
            })
        } else {
            PyClassInitializer::from(LoginMethod::new("ipad".to_string())?).add_subclass(Self {
                protocol_override: true,
            })
        })
    }

    /// 登录到指定的账号。
    ///
    /// # Arguments
    /// - `uin` - 用户的 QQ 号。
    /// - `data_folder` - 数据目录。
    fn login<'py>(
        self_: PyRef<'py, Self>,
        py: Python<'py>,
        uin: i64,
        mut data_folder: PathBuf,
    ) -> PyResult<&'py PyAny> {
        use requestty::Question;
        let protocol_override = self_.protocol_override;
        let protocol = self_.as_ref().protocol.clone();

        py_future(py, async move {
            data_folder.push(uin.to_string());
            tokio::fs::create_dir_all(&data_folder).await?;

            // 询问登录方式
            let login_method = {
                let login_method = Question::select("login_method")
                    .message(format!("请选择账号 {} 的登录方式：", uin))
                    .choice("密码登录")
                    .choice("二维码登录")
                    .build();
                requestty::prompt_one(login_method)
                    .map_err(anyhow::Error::new)?
                    .as_list_item()
                    .unwrap()
                    .index
            };

            // 询问协议
            let protocol = if login_method != 1 {
                if protocol_override {
                    let protocol = Question::select("protocol")
                        .message("请选择客户端协议：")
                        .choice("IPad")
                        .choice("Android Phone")
                        .choice("Android Watch")
                        .choice("MacOS")
                        .choice("企点")
                        .default(0)
                        .build();
                    let protocol = requestty::prompt_one(protocol)
                        .map_err(anyhow::Error::new)?
                        .as_list_item()
                        .unwrap()
                        .index;
                    match protocol {
                        0 => Protocol::IPad,
                        1 => Protocol::AndroidPhone,
                        2 => Protocol::AndroidWatch,
                        3 => Protocol::MacOS,
                        4 => Protocol::QiDian,
                        _ => unreachable!(),
                    }
                } else {
                    protocol
                }
            } else {
                // 二维码仅支持手表协议
                Protocol::AndroidWatch
            };

            let device = load_device_json(uin, data_folder.clone()).await?;
            let (client, alive) = prepare_client(device, protocol.clone()).await?;

            if !try_token_login(&client, data_folder.clone()).await? {
                match login_method {
                    0 => {
                        // 密码登录
                        let password = {
                            let password = Question::password("password")
                                .message("请输入密码：")
                                .build();
                            let password =
                                requestty::prompt_one(password).map_err(anyhow::Error::new)?;
                            password.as_string().unwrap().to_string()
                        };

                        password_login(&client, uin, password, false).await?;
                    }
                    1 => {
                        // 二维码登录
                        qrcode_login(&client, uin).await?;
                    }
                    _ => Err(anyhow!("尚未实现的登录方式"))?,
                }
            }

            // 注册客户端，启动心跳。
            after_login(&client).await;
            save_token(&client, data_folder.clone()).await?;
            Ok(crate::client::Client::new(client, alive, data_folder).await)
        })
    }
}

/// 加载 `device.json`。
async fn load_device_json(uin: i64, mut data_folder: PathBuf) -> Result<Device> {
    use crate::device;

    // 获取 `device.json` 的路径
    let device_json = {
        data_folder.push("device.json");
        data_folder
    };

    // 解析设备信息
    let device = if device_json.exists() {
        // 尝试读取已有的 `device.json`
        let json = tokio::fs::read_to_string(device_json).await?;
        device::from_json(&json, &device::random_from_uin(uin))?
    } else {
        // 否则，生成一个新的 `device.json` 并保存到文件中
        let device = device::random_from_uin(uin);
        let json = device::to_json(&device)?;
        tokio::fs::write(device_json, json).await?;
        device
    };

    Ok(device)
}

/// 创建客户端，准备登录。
async fn prepare_client(
    device: Device,
    protocol: Protocol,
) -> Result<(Arc<Client>, JoinHandle<()>)> {
    let client = Arc::new(Client::new(
        device,
        get_version(protocol),
        DefaultHandler, // TODO: 处理事件
    ));
    let alive = tokio::spawn({
        let client = client.clone();
        // 连接最快的服务器
        let stream = DefaultConnector.connect(&client).await?;
        async move { client.start(stream).await }
    });

    tokio::task::yield_now().await; // 等一下，确保连上了
    Ok((client, alive))
}

/// 尝试使用 token 登录。
async fn try_token_login(client: &Client, mut data_folder: PathBuf) -> Result<bool> {
    let token_path = {
        data_folder.push("token.json");
        data_folder
    };
    if !token_path.exists() {
        return Ok(false);
    }
    tracing::info!("发现上一次登录的 token，尝试使用 token 登录");
    let token = tokio::fs::read_to_string(&token_path).await?;
    let token: Token = serde_json::from_str(&token)?;
    match client.token_login(token).await {
        Ok(login_resp) => {
            if let LoginResponse::Success(LoginSuccess {
                ref account_info, ..
            }) = login_resp
            {
                tracing::info!("登录成功: {:?}", account_info);
                return Ok(true);
            }
            bail!("登录失败，原因未知：{:?}", login_resp)
        }
        Err(_) => {
            tracing::info!("token 登录失败，将删除 token");
            tokio::fs::remove_file(token_path).await?;
            Ok(false)
        }
    }
}

/// 在控制台打印二维码。
fn print_qrcode(qrcode: &Bytes) -> Result<String> {
    let qrcode = image::load_from_memory(qrcode)?.to_luma8();
    let mut qrcode = rqrr::PreparedImage::prepare(qrcode);
    let grids = qrcode.detect_grids();
    if grids.len() != 1 {
        return Err(anyhow!("无法识别二维码"));
    }
    let (_, content) = grids[0].decode()?;
    let qrcode = qrcode::QrCode::new(content)?;
    let qrcode = qrcode.render::<qrcode::render::unicode::Dense1x2>().build();
    Ok(qrcode)
}

/// 保存 Token，用于断线重连。
async fn save_token(client: &Client, mut data_folder: PathBuf) -> Result<()> {
    let token = client.gen_token().await;
    let token = serde_json::to_string(&token)?;
    let token_path = {
        data_folder.push("token.json");
        data_folder
    };
    tokio::fs::write(token_path, token).await?;
    Ok(())
}

/// 密码登录。
async fn password_login(client: &Client, uin: i64, password: String, md5: bool) -> Result<()> {
    tracing::info!("使用密码登录，uin={}", uin);

    let mut resp = if !md5 {
        client.password_login(uin, &password).await?
    } else {
        client
            .password_md5_login(uin, &hex::decode(password)?)
            .await?
    };

    loop {
        match resp {
            LoginResponse::Success(LoginSuccess {
                ref account_info, ..
            }) => {
                tracing::info!("登录成功: {:?}", account_info);
                break;
            }
            LoginResponse::DeviceLocked(LoginDeviceLocked {
                // ref sms_phone,
                ref verify_url,
                ref message,
                ..
            }) => {
                tracing::info!("设备锁: {}", message.as_deref().unwrap_or(""));
                tracing::info!("验证 url: {}", verify_url.as_deref().unwrap_or(""));
                bail!("手机打开 url，处理完成后重启程序")
                //也可以走短信验证
                // resp = client.request_sms().await.expect("failed to request sms");
            }
            LoginResponse::NeedCaptcha(LoginNeedCaptcha { ref verify_url, .. }) => {
                tracing::info!("滑块 url: {}", verify_url.as_deref().unwrap_or("")); // TODO: 接入 TxCaptchaHelper
                tracing::info!("请输入 ticket:");
                let mut reader = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
                let ticket = reader.next().await.transpose().unwrap().unwrap();
                resp = client.submit_ticket(&ticket).await?;
            }
            LoginResponse::DeviceLockLogin { .. } => {
                resp = client.device_lock_login().await?;
            }
            LoginResponse::AccountFrozen => bail!("账号被冻结"),
            LoginResponse::TooManySMSRequest => bail!("短信请求过于频繁"),
            LoginResponse::UnknownStatus(LoginUnknownStatus {
                ref status,
                ref tlv_map,
                ref message,
            }) => {
                bail!("登陆失败，原因未知：{}, {}, {:?}", status, message, tlv_map);
            }
        }
    }

    Ok(())
}

/// 二维码登录。
async fn qrcode_login(client: &Client, uin: i64) -> Result<()> {
    tracing::info!("使用二维码登录，uin={}", uin);

    let mut resp = client.fetch_qrcode().await?;

    let mut image_sig = Bytes::new();
    loop {
        match resp {
            QRCodeState::ImageFetch(QRCodeImageFetch {
                ref image_data,
                ref sig,
            }) => {
                let qr = print_qrcode(image_data)?;
                tracing::info!("请扫描二维码: \n{}", qr);
                image_sig = sig.clone();
            }
            QRCodeState::WaitingForScan => {
                tracing::debug!("等待二维码扫描")
            }
            QRCodeState::WaitingForConfirm => {
                tracing::debug!("二维码已扫描，等待确认")
            }
            QRCodeState::Timeout => {
                tracing::info!("二维码已超时，重新获取");
                if let QRCodeState::ImageFetch(QRCodeImageFetch {
                    ref image_data,
                    ref sig,
                }) = client.fetch_qrcode().await.expect("failed to fetch qrcode")
                {
                    let qr = print_qrcode(image_data)?;
                    tracing::info!("请扫描二维码: \n{}", qr);
                    image_sig = sig.clone();
                }
            }
            QRCodeState::Confirmed(QRCodeConfirmed {
                ref tmp_pwd,
                ref tmp_no_pic_sig,
                ref tgt_qr,
                ..
            }) => {
                tracing::info!("二维码已确认");
                let mut login_resp = client.qrcode_login(tmp_pwd, tmp_no_pic_sig, tgt_qr).await?;
                if let LoginResponse::DeviceLockLogin { .. } = login_resp {
                    login_resp = client.device_lock_login().await?;
                }
                if let LoginResponse::Success(LoginSuccess {
                    ref account_info, ..
                }) = login_resp
                {
                    tracing::info!("登录成功: {:?}", account_info);
                    let real_uin = client.uin().await;
                    if real_uin != uin {
                        tracing::warn!("预期登录账号 {}，但实际登陆账号为 {}", uin, real_uin);
                    }
                    break;
                }
                bail!("登录失败，原因未知：{:?}", login_resp)
            }
            QRCodeState::Canceled => {
                bail!("二维码已取消")
            }
        }
        sleep(Duration::from_secs(5)).await;
        resp = client.query_qrcode_result(&image_sig).await?;
    }

    Ok(())
}

pub async fn reconnect(client: &Arc<Client>, data_folder: &Path) -> Result<Option<JoinHandle<()>>> {
    retry(
        10,
        || async {
            // 如果不是网络原因掉线，不重连（服务端强制下线/被踢下线/用户手动停止）
            if client.get_status() != (NetworkStatus::NetworkOffline as u8) {
                tracing::warn!("客户端因非网络原因下线，不再重连");
                return Ok(None);
            }
            client.stop(NetworkStatus::NetworkOffline);

            tracing::error!("客户端连接中断，将在 10 秒后重连");
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;

            let alive = tokio::spawn({
                let client = client.clone();
                // 连接最快的服务器
                let stream = DefaultConnector.connect(&client).await?;
                async move { client.start(stream).await }
            });
            tokio::task::yield_now().await; // 等一下，确保连上了

            // 启动接收后，再发送登录请求，否则报错 NetworkError
            let token_path = data_folder.join("token.json");
            if !token_path.exists() {
                tracing::error!("重连失败：未找到上次登录的 token");
                return Ok(None);
            }
            let token = tokio::fs::read_to_string(token_path).await?;
            let token = match serde_json::from_str(&token) {
                Ok(token) => token,
                Err(err) => {
                    tracing::error!("重连失败：无法解析上次登录的 token，{}", err);
                    return Ok(None);
                }
            };
            fast_login(client, &Credential::Token(token))
                .await
                .map_err(|e| {
                    client.stop(NetworkStatus::NetworkOffline);
                    e
                })?;

            after_login(client).await;

            tracing::info!("客户端重连成功");
            Ok(Some(alive))
        },
        |e, c| async move {
            let backtrace = e.backtrace().map(|b| b.to_string()).unwrap_or_default();
            tracing::error!("客户端重连失败，原因：{}，剩余尝试 {} 次", e, c);
            tracing::debug!("backtrace: {}", backtrace);
        },
    )
    .await
}
