//! 登录。
//!
//! awr 目前支持三种登录方式：
//! - 密码登录：[`login_with_password`]
//! - 密码 MD5 登录：[`login_with_password_md5`]
//! - 扫码登录：[`login_with_qrcode`]
//!
//! 此外，awr 还提供了 [`login`] 宏/方法，以统一不同登录方式的参数。
//!
//! 登录方法接受 QQ 号、密码、协议、配置文件目录等参数，返回一个 [`Client`] 和一个 [`AliveHandle`]。
//! 登录完成后，账户的必要信息将会保存在 `配置文件目录/QQ号/` 文件夹下，以便下次登录时使用。
//!
//! | 参数 | 说明 |
//! | --- | --- |
//! | `uin` | QQ 号 |
//! | `password` | 密码 |
//! | `password_md5` | 密码 MD5 |
//! | `show_qrcode` | 扫码登录时的回调函数 |
//! | `protocol` | 协议 |
//! | `data_folder` | 配置文件目录 |
//!
//! [`Client`] 用于发送消息、获取好友列表等操作，[`AliveHandle`] 用于保持连接与断线重连。
//!
//! 部分登录方式可以指定使用的协议。可用的协议包括：
//!
//! | 协议 | 说明 |
//! | --- | --- |
//! | [`Protocol::IPad`] | iPad 协议 |
//! | [`Protocol::AndroidPhone`] | Android 手机协议 |
//! | [`Protocol::AndroidWatch`] | Android 手表协议 |
//! | [`Protocol::MacOS`] | MacOS 客户端协议 |
//! | [`Protocol::QiDian`] | 企点协议 |
//!
//! # Examples
//!
//! ## Rust
//! ```rust
//! use libawr::{login, Protocol};
//!
//! # async fn _main() -> Result<(), Box<dyn std::error::Error>> {
//! // 密码登录
//! let (client, alive) = login!(12345678, password="xxxxxx", protocol=Protocol::IPad).await?;
//! // 扫码登录（手表协议）
//! let (client, alive) = login!(12345678, show_qrcode=|_| { unimplemented!() }).await?;
//! // 指定配置文件目录
//! let (client, alive) = login!(
//!     12345678,
//!     password = "xxxxxx",
//!     protocol = Protocol::IPad,
//!     data_folder = "./bots"
//! ).await?;
//!
//! // 断线重连
//! alive.auto_reconnect().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Python
//! ```python
//! ## 密码登录
//! client, alive = await awr.login(12345678, password="xxxxxx", protocol=awr.Protocol.IPAD)
//! ## 扫码登录（手表协议）
//! client, alive = await awr.login(12345678, show_qrcode=lambda _: None)
//! ## 指定配置文件目录
//! client, alive = await awr.login(
//!     12345678,
//!     password="xxxxxx",
//!     protocol=awr.Protocol.IPAD,
//!     data_folder="./bots"
//! )
//!
//! ## 断线重连
//! await alive.auto_reconnect()
//! ```
//!
//! [`login`]: crate::login!

use std::error::Error;
use std::future::Future;
use std::{
    backtrace::Backtrace,
    path::{Path, PathBuf},
    sync::Arc,
};

use bytes::Bytes;
use futures_util::StreamExt;
use ricq::{
    client::{Connector, DefaultConnector, NetworkStatus, Token},
    ext::{common::after_login, reconnect::fast_login},
    handler::DefaultHandler,
    version::get_version,
    Device, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginSuccess,
};
use thiserror::Error;
use tokio::task::JoinHandle;
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::{client::Client, utils::retry};

box_error_impl!(LoginError, LoginErrorImpl, "登录错误。");

/// 协议。
///
/// | 协议 | 说明 |
/// | --- | --- |
/// | [`Protocol::IPad`] | iPad 协议 |
/// | [`Protocol::AndroidPhone`] | Android 手机协议 |
/// | [`Protocol::AndroidWatch`] | Android 手表协议 |
/// | [`Protocol::MacOS`] | MacOS 客户端协议 |
/// | [`Protocol::QiDian`] | 企点协议 |
///
/// # Python
/// ```python
/// class Protocol(Enum):
///     IPAD = enum.auto()
///     ANDROID_PHONE = enum.auto()
///     ANDROID_WATCH = enum.auto()
///     MAC_OS = enum.auto()
///     QI_DIAN = enum.auto()
/// ```
pub use ricq::Protocol;

/// 登录错误。
#[derive(Error, Debug)]
enum LoginErrorImpl {
    /// `device.json` 错误。
    #[error("`device.json` 错误")]
    DeviceError(#[from] crate::device::DeviceError),

    /// 无效的 token 文件。
    #[error("无效的 token 文件")]
    InvalidToken {
        /// 原始错误。
        #[from]
        source: serde_json::Error,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// IO 错误。
    #[error("IO 错误")]
    IOError {
        /// 原始错误。
        #[from]
        source: std::io::Error,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// 服务器返回意外响应。
    #[error("服务器返回意外响应: {response:?}")]
    RemoteException {
        /// 响应信息。
        response: LoginResponse,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// 登录失败。
    #[error("登录失败")]
    RQError {
        /// 原始错误。
        #[from]
        source: ricq::RQError,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// 需要设备锁验证。
    #[error("需要设备锁验证 ({message}), 请前往链接进行验证: {url}")]
    DeviceLocked {
        /// 错误信息。
        message: String,
        /// 验证链接。
        url: String,
    },

    /// 账号被冻结。
    #[error("账号被冻结")]
    AccountFrozen,

    /// 短信请求过于频繁。
    #[error("短信请求过于频繁")]
    TooManySMSRequest,

    /// 二维码已取消。
    #[error("二维码已取消")]
    QrCodeCancelled,

    /// 连接断开。
    #[error("连接断开")]
    ConnectionClosed {
        /// 原始错误。
        #[from]
        source: tokio::task::JoinError,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// 重连终止。
    #[error("重连终止")]
    ReconnectAborted {
        /// 错误消息。
        message: String,
        /// 错误堆栈。
        backtrace: Backtrace,
    },

    /// 其他错误。
    #[error("其他错误")]
    Other {
        /// 原始错误。
        #[from]
        source: Box<(dyn Error + Sync + Send)>,
        /// 错误堆栈。
        backtrace: Backtrace,
    },
}

type Result<T> = std::result::Result<T, LoginError>;

/// 登录保持。
///
/// awr 在登录完成后，并不会主动阻塞程序，而是需要用户手动开始保持连接。
/// 这给了用户更多的控制权，比如用户可以将登录保持纳入自己的异步任务实现中。
///
/// [`auto_reconnect`] 方法返回一个 Future，理论上此 Future 会无限期等待，除非有错误发生。
/// 等待期间，awr 会自动进行断线重连。
///
/// 如果需要更细粒度的控制，可以使用 [`alive`] 和 [`reconnect`] 方法，分别用于保持连接和重连。
/// [`auto_reconnect`] 方法实际上是不断循环调用 [`alive`] 和 [`reconnect`] 方法。
///
/// # Examples
///
/// ## Rust
/// ```rust
/// use libawr::{login, Protocol};
///
/// # async fn _main() -> Result<(), Box<dyn std::error::Error>> {
/// let (client, mut alive) = login!(12345678, password="xxxxxx", protocol=Protocol::IPad).await?;
///
/// loop {
///     alive.alive().await?;
///     println!("连接断开，正在重连...");
///     alive.reconnect().await?;
///     println!("重连成功");
/// }
/// # Ok(())
/// # }
/// ```
///
/// ## Python
/// ```python
/// client, alive = awr.login(12345678, password="xxxxxx", protocol=awr.Protocol.IPad)
///
/// while True:
///     await alive.alive()
///     print("连接断开，正在重连...")
///     await alive.reconnect()
///     print("重连成功")
/// ```
///
/// # Python
/// ```python
/// class AliveHandle:
///     ...
/// ```
///
/// [`auto_reconnect`]: AliveHandle::auto_reconnect
/// [`alive`]: AliveHandle::alive
/// [`reconnect`]: AliveHandle::reconnect
pub struct AliveHandle {
    client: Arc<ricq::Client>,
    account_data_folder: PathBuf,
    alive: Option<JoinHandle<()>>,
}

impl AliveHandle {
    pub(crate) fn new(
        client: Arc<ricq::Client>,
        account_data_folder: PathBuf,
        alive: JoinHandle<()>,
    ) -> Self {
        Self {
            client,
            account_data_folder,
            alive: Some(alive),
        }
    }

    /// 等待，直到连接断开。
    ///
    /// # Python
    /// ```python
    /// async def alive(self): ...
    /// ```
    ///
    /// # Note
    /// 此方法的 Python 绑定带有借用检查，同一时间只能有一个调用。
    /// 重复调用会引发 `RuntimeError`。
    pub async fn alive(&mut self) -> Result<()> {
        if let Some(alive) = self.alive.take() {
            alive.await?;
        }
        Ok(())
    }

    /// 断线重连。
    ///
    /// # Safety
    /// 此方法不会检查连接是否已经断开，如果连接未断开，调用此方法会导致不可预知的行为。
    ///
    /// 建议只在 [`alive`] 方法返回后调用此方法。
    ///
    /// # Python
    /// ```python
    /// async def reconnect(self): ...
    /// ```
    ///
    /// # Note
    /// 此方法的 Python 绑定带有借用检查，同一时间只能有一个调用。
    /// 重复调用会引发 `RuntimeError`。
    ///
    /// [`alive`]: AliveHandle::alive
    pub async fn reconnect(&mut self) -> Result<()> {
        if self.alive.is_none() {
            // 断线重连
            let handle = reconnect(&self.client, &self.account_data_folder).await?;
            self.alive = Some(handle);
        }
        Ok(())
    }

    /// 开始自动断线重连。
    ///
    /// 此方法相当于无限循环调用 [`alive`] 和 [`reconnect`] 方法。
    ///
    /// # Python
    /// ```python
    /// async def auto_reconnect(self): ...
    /// ```
    ///
    /// # Note
    /// 此方法的 Python 绑定带有借用检查，并且消耗所有权。
    /// 调用此方法后，对此对象的后续使用会引发 `RuntimeError`。
    ///
    /// [`alive`]: AliveHandle::alive
    /// [`reconnect`]: AliveHandle::reconnect
    pub async fn auto_reconnect(mut self) -> Result<()> {
        loop {
            self.alive().await?;
            self.reconnect().await?;
        }
    }
}

async fn login_impl<Fut>(
    uin: i64,
    protocol: Protocol,
    data_folder: impl AsRef<Path>,
    login_with_credential: impl FnOnce(Arc<ricq::Client>) -> Fut,
) -> Result<(Arc<Client>, AliveHandle)>
where
    Fut: Future<Output = Result<()>>,
{
    // 创建数据文件夹
    let account_data_folder = data_folder.as_ref().join(uin.to_string());
    tokio::fs::create_dir_all(&account_data_folder).await?;

    let device = load_device_json(uin, &account_data_folder).await?;
    let (client, alive) = prepare_client(device, protocol).await?;

    // 尝试 token 登录
    if !try_token_login(&client, &account_data_folder).await? {
        login_with_credential(client.clone()).await?;
    }

    // 注册客户端，启动心跳。
    after_login(&client).await;
    save_token(&client, &account_data_folder).await?;

    let alive = AliveHandle::new(client.clone(), account_data_folder, alive);
    let client = Arc::new(Client::new(client).await);
    Ok((client, alive))
}

/// 使用密码登录。
///
/// # Python
/// ```python
/// async def login_with_password(
///     uin: int,
///     password: str,
///     protocol: Protocol,
///     data_folder: str = "./bots",
/// ) -> Tuple[Client, AliveHandle]: ...
/// ```
pub async fn login_with_password(
    uin: i64,
    password: &str,
    protocol: Protocol,
    data_folder: impl AsRef<Path>,
) -> Result<(Arc<Client>, AliveHandle)> {
    login_impl(uin, protocol, data_folder, move |client| async move {
        let resp = client.password_login(uin, password).await?;
        handle_password_login_resp(&client, resp).await?;
        Ok(())
    })
    .await
}

/// 使用密码 MD5 登录。
///
/// # Python
/// ```python
/// async def login_with_password_md5(
///     uin: int,
///     password_md5: bytes,
///     protocol: Protocol,
///     data_folder: str = "./bots",
/// ) -> Tuple[Client, AliveHandle]: ...
pub async fn login_with_password_md5(
    uin: i64,
    password_md5: &[u8],
    protocol: Protocol,
    data_folder: impl AsRef<Path>,
) -> Result<(Arc<Client>, AliveHandle)> {
    login_impl(uin, protocol, data_folder, move |client| async move {
        let resp = client.password_md5_login(uin, password_md5).await?;
        handle_password_login_resp(&client, resp).await?;
        Ok(())
    })
    .await
}

/// 使用二维码登录。
///
/// 二维码图片会通过 `show_qrcode` 回调函数传递给调用者。
/// 调用者需要自行实现二维码图片的显示。
///
/// # Examples
///
/// ## Rust
/// 下面是一个使用 [image](https://crates.io/crates/image)、[rqrr](https://crates.io/crates/rqrr)
/// 和 [qrcode]( https://crates.io/crates/qrcode) 库在控制台打印二维码的例子。
///
/// ```rust
/// use anyhow::{bail, Result};
/// use libawr::{login, Protocol};
///
/// /// 将二维码图片转换为文本。
/// fn qrcode_text(qrcode: &[u8]) -> Result<String> {
///     let qrcode = image::load_from_memory(qrcode)?.to_luma8();
///     let mut qrcode = rqrr::PreparedImage::prepare(qrcode);
///     let grids = qrcode.detect_grids();
///     if grids.len() != 1 {
///        bail!("无法识别二维码");
///     }
///     let (_, content) = grids[0].decode()?;
///     let qrcode = qrcode::QrCode::new(content)?;
///     let qrcode = qrcode.render::<qrcode::render::unicode::Dense1x2>().build();
///     Ok(qrcode)
/// }
///
/// # async fn _main() -> Result<(), Box<dyn std::error::Error>> {
/// let (client, alive) = login!(
///     12345678,
///     show_qrcode = |img| {
///         println!("{}", qrcode_text(&img)?);
///         Ok(())
///     }
/// ).await?;
///
/// alive.auto_reconnect().await?;
/// # Ok(())
/// # }
/// ```
///
/// ## Python
/// 下面是一个使用 [Pillow](https://pypi.org/project/Pillow/) 显示二维码图片的例子。
///
/// ```python
/// from io import BytesIO
/// from PIL import Image
///
/// client, alive = await awr.login(
///     12345678,
///     show_qrcode = lambda img: Image.open(BytesIO(img)).show(),
/// )
/// ```
///
/// # Python
/// ```python
/// async def login_with_qrcode(
///     uin: int,
///     show_qrcode: Callable[[bytes], None],
///     data_folder: str = "./bots",
/// ) -> Tuple[Client, AliveHandle]: ...
/// ```
pub async fn login_with_qrcode(
    uin: i64,
    show_qrcode: impl FnMut(Bytes) -> std::result::Result<(), Box<dyn Error + Send + Sync>>,
    data_folder: impl AsRef<Path>,
) -> Result<(Arc<Client>, AliveHandle)> {
    login_impl(
        uin,
        Protocol::AndroidWatch,
        data_folder,
        move |client| async move {
            qrcode_login(&client, uin, show_qrcode).await?;
            Ok(())
        },
    )
    .await
}

/// 登录。
///
/// 在 Rust 中，使用宏模拟了函数重载和默认参数。
///
/// 更多信息请参考 [`login`] 模块。
///
/// # Examples
///
/// ## Rust
/// ```rust
/// use libawr::{login, Protocol};
///
/// # async fn _main() -> Result<(), Box<dyn std::error::Error>> {
/// // 密码登录
/// let (client, alive) = login!(12345678, password="xxxxxx", protocol=Protocol::IPad).await?;
/// // 密码 MD5 登录
/// let (client, alive) = login!(
///     12345678,
///     password_md5 = &hex::decode("bed09fdb1471ef51")?,
///     protocol = Protocol::IPad
/// ).await?;
/// // 扫码登录（手表协议）
/// let (client, alive) = login!(12345678, show_qrcode=|_| { unimplemented!() }).await?;
/// // 指定配置文件目录
/// let (client, alive) = login!(
///     12345678,
///     password = "xxxxxx",
///     protocol = Protocol::IPad,
///     data_folder = "./bots"
/// ).await?;
/// # Ok(())
/// # }
/// ```
///
/// ## Python
/// ```python
/// ## 密码登录
/// client, alive = await awr.login(12345678, password="xxxxxx", protocol=awr.Protocol.IPAD)
/// ## 密码 MD5 登录
/// client, alive = await awr.login(
///     12345678,
///     password_md5 = bytes.fromhex("bed09fdb1471ef51"),
///     protocol = awr.Protocol.IPAD
/// )
/// ## 扫码登录（手表协议）
/// client, alive = await awr.login(12345678, show_qrcode=lambda _: None)
/// ## 指定配置文件目录
/// client, alive = await awr.login(
///     12345678,
///     password = "xxxxxx",
///     protocol = awr.Protocol.IPAD,
///     data_folder = "./bots"
/// )
/// ```
///
/// # Python
/// ```python
/// @overload
/// async def login(
///     uin: int,
///     *,
///     password: str,
///     protocol: Protocol,
///     data_folder = "./bots"
/// ) -> Tuple[Client, AliveHandle]: ...
/// @overload
/// async def login(
///     uin: int,
///     *,
///     password_md5: str,
///     protocol: Protocol,
///     data_folder = "./bots"
/// ) -> Tuple[Client, AliveHandle]: ...
/// @overload
/// async def login(
///     uin: int,
///     *,
///     show_qrcode: Callable[[bytes], None],
///     data_folder = "./bots"
/// ) -> Tuple[Client, AliveHandle]: ...
/// ```
///
/// [`login`]: mod@crate::login
#[macro_export]
macro_rules! login {
    ($uin: expr, password = $password: expr, protocol = $protocol: expr, data_folder = $data_folder: expr $(,)?) => {
        $crate::login::login_with_password($uin, $password, $protocol, $data_folder)
    };
    ($uin: expr, password = $password: expr, protocol = $protocol: expr $(,)?) => {
        $crate::login::login_with_password(
            $uin,
            $password,
            $protocol,
            ::std::path::Path::new("./bots"),
        )
    };

    ($uin: expr, password_md5 = $password_md5: expr, protocol = $protocol: expr, data_folder = $data_folder: expr $(,)?) => {
        $crate::login::login_with_password_md5($uin, $password_md5, $protocol, $data_folder)
    };
    ($uin: expr, password_md5 = $password_md5: expr, protocol = $protocol: expr $(,)?) => {
        $crate::login::login_with_password_md5(
            $uin,
            $password_md5,
            $protocol,
            ::std::path::Path::new("./bots"),
        )
    };

    ($uin: expr, show_qrcode = $show_qrcode: expr, data_folder = $data_folder: expr $(,)?) => {
        $crate::login::login_with_qrcode($uin, $show_qrcode, $data_folder)
    };
    ($uin: expr, show_qrcode = $show_qrcode: expr $(,)?) => {
        $crate::login::login_with_qrcode($uin, $show_qrcode, ::std::path::Path::new("./bots"))
    };
}

/// 加载 `device.json`。
async fn load_device_json(uin: i64, data_folder: impl AsRef<Path>) -> Result<Device> {
    use crate::device;

    // 获取 `device.json` 的路径
    let device_json = data_folder.as_ref().join("device.json");

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
) -> tokio::io::Result<(Arc<ricq::Client>, JoinHandle<()>)> {
    let client = Arc::new(ricq::Client::new(
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
async fn try_token_login(
    client: &ricq::Client,
    account_data_folder: impl AsRef<Path>,
) -> Result<bool> {
    let token_path = account_data_folder.as_ref().join("token.json");

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
            Err(LoginErrorImpl::RemoteException {
                response: login_resp,
                backtrace: Backtrace::capture(),
            }
            .into())
        }
        Err(_) => {
            tracing::info!("token 登录失败，将删除 token");
            tokio::fs::remove_file(token_path).await?;
            Ok(false)
        }
    }
}

/// 保存 Token，用于断线重连。
async fn save_token(client: &ricq::Client, account_data_folder: impl AsRef<Path>) -> Result<()> {
    let token = client.gen_token().await;
    let token = serde_json::to_string(&token)?;
    let token_path = account_data_folder.as_ref().join("token.json");
    tokio::fs::write(token_path, token).await?;
    Ok(())
}

/// 密码登录。
async fn handle_password_login_resp(client: &ricq::Client, mut resp: LoginResponse) -> Result<()> {
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
                verify_url,
                message,
                ..
            }) => {
                return Err(LoginErrorImpl::DeviceLocked {
                    message: message.unwrap_or_default(),
                    url: verify_url.unwrap_or_default(),
                }
                .into());
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
            LoginResponse::AccountFrozen => return Err(LoginErrorImpl::AccountFrozen.into()),
            LoginResponse::TooManySMSRequest => {
                return Err(LoginErrorImpl::TooManySMSRequest.into())
            }
            unknown => {
                return Err(LoginErrorImpl::RemoteException {
                    response: unknown,
                    backtrace: Backtrace::capture(),
                }
                .into())
            }
        }
    }

    Ok(())
}

/// 二维码登录。
pub async fn qrcode_login(
    client: &ricq::Client,
    uin: i64,
    mut show_qrcode: impl FnMut(Bytes) -> std::result::Result<(), Box<dyn Error + Send + Sync>>,
) -> Result<()> {
    use std::time::Duration;

    use ricq::{QRCodeConfirmed, QRCodeImageFetch, QRCodeState};

    tracing::info!("使用二维码登录，uin={}", uin);

    let mut resp = client.fetch_qrcode().await?;

    let mut image_sig = bytes::Bytes::new();
    loop {
        match resp {
            QRCodeState::ImageFetch(QRCodeImageFetch {
                image_data,
                ref sig,
            }) => {
                show_qrcode(image_data)?;
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
                    image_data,
                    ref sig,
                }) = client.fetch_qrcode().await.expect("failed to fetch qrcode")
                {
                    show_qrcode(image_data)?;
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
                return Err(LoginErrorImpl::RemoteException {
                    response: login_resp,
                    backtrace: Backtrace::capture(),
                }
                .into());
            }
            QRCodeState::Canceled => return Err(LoginErrorImpl::QrCodeCancelled.into()),
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
        resp = client.query_qrcode_result(&image_sig).await?;
    }

    Ok(())
}

/// 断线重连。
pub(crate) async fn reconnect(
    client: &Arc<ricq::Client>,
    account_data_folder: &Path,
) -> Result<JoinHandle<()>> {
    retry(
        10,
        || async {
            // 如果不是网络原因掉线，不重连（服务端强制下线/被踢下线/用户手动停止）
            if client.get_status() != (NetworkStatus::NetworkOffline as u8) {
                return Ok(Err(LoginErrorImpl::ReconnectAborted {
                    message: "客户端因非网络原因下线，不再重连".to_string(),
                    backtrace: Backtrace::capture(),
                }
                .into()));
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
            let token_path = account_data_folder.join("token.json");
            if !token_path.exists() {
                return Ok(Err(LoginErrorImpl::ReconnectAborted {
                    message: "重连失败：无法找到上次登录的 token".to_string(),
                    backtrace: Backtrace::capture(),
                }
                .into()));
            }
            let token = tokio::fs::read_to_string(token_path).await?;
            let token = match serde_json::from_str(&token) {
                Ok(token) => token,
                Err(err) => {
                    return Ok(Err(LoginErrorImpl::ReconnectAborted {
                        message: format!("重连失败：无法解析上次登录的 token: {err}"),
                        backtrace: Backtrace::capture(),
                    }
                    .into()));
                }
            };
            fast_login(client, &ricq::ext::reconnect::Credential::Token(token))
                .await
                .map_err(|e| {
                    client.stop(NetworkStatus::NetworkOffline);
                    e
                })?;

            after_login(client).await;

            tracing::info!("客户端重连成功");
            Ok(Ok(alive))
        },
        |e: LoginError, c| async move {
            tracing::error!("客户端重连失败，原因：{}，剩余尝试 {} 次", e, c);
            if let Some(backtrace) = (&e as &dyn Error).request_ref::<Backtrace>() {
                tracing::debug!("backtrace: {}", backtrace);
            }
        },
    )
    .await?
}
