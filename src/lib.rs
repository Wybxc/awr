use pyo3::prelude::*;

use tracing::info;

mod client;
mod device;
mod login;
mod loguru;
mod utils;

const LOGO: &str = r#"
 █████╗ ██╗    ██╗██████╗ 
██╔══██╗██║    ██║██╔══██╗
███████║██║ █╗ ██║██████╔╝
██╔══██║██║███╗██║██╔══██╗
██║  ██║╚███╔███╔╝██║  ██║
╚═╝  ╚═╝ ╚══╝╚══╝ ╚═╝  ╚═╝
"#;

/// 初始化 AWR 环境：
/// - 设置日志输出。
/// - 打印版本信息。
#[pyfunction]
fn init(module: &PyModule) -> PyResult<()> {
    // 设置日志输出
    loguru::init(module)?;

    // 打印版本信息
    info!("{}", LOGO);
    Ok(())
}

/// Avilla with Ricq.
#[pymodule]
fn awr(_py: Python, m: &PyModule) -> PyResult<()> {
    // 初始化
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add("__version__",  env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(loguru::getframe, m)?)?;
    // 登录方式
    m.add_class::<login::LoginMethod>()?;
    m.add_class::<login::Password>()?;
    m.add_class::<login::QrCode>()?;
    m.add_class::<login::Dynamic>()?;
    // 客户端
    m.add_class::<client::Client>()?;
    m.add_class::<client::AccountInfo>()?;
    m.add_class::<client::FriendInfo>()?;
    m.add_class::<client::FriendGroupInfo>()?;
    m.add_class::<client::FriendList>()?;
    Ok(())
}
