//! 基于 [`ricq`] 包装，供 Python 使用的 QQ 无头客户端。
//!
//! 更多信息请参考 [`login`] 和 [`client`] 模块。
//!
//! # Examples
//! ```python
//! import awr
//! import asyncio
//!
//! async def main():
//!     ## 登录账号
//!     client = await awr.Dynamic().login(12345678, "./bots")
//!     ## 读取群列表
//!     print(await client.get_group_list())
//!     ## 保持连接
//!     await client.alive()
//!     
//! try:
//!     asyncio.run(main())
//! except KeyboardInterrupt:
//!     import sys
//!     sys.exit(0)
//! ```
//!
//! [`ricq`]: https://docs.rs/ricq/latest/ricq/

#![feature(try_blocks)]

use pyo3::prelude::*;
use pyo3_built::pyo3_built;

use tracing::info;

#[macro_use]
mod utils;

pub mod client;
// mod device;
pub mod login;
mod loguru;
// pub mod message;

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
#[doc(hidden)]
pub fn init(module: &PyModule) -> PyResult<()> {
    // 设置日志输出
    loguru::init(module)?;

    // 打印版本信息
    info!("{}", LOGO);
    Ok(())
}

/// 构建信息。
#[allow(dead_code)]
pub mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[pymodule]
#[pyo3(name = "_awr")]
#[doc(hidden)]
pub fn awr(py: Python, m: &PyModule) -> PyResult<()> {
    // 初始化
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__build__", pyo3_built!(py, build))?;
    m.add_function(wrap_pyfunction!(loguru::getframe, m)?)?;
    // 登录
    m.add_function(wrap_pyfunction!(login::login, m)?)?;
    m.add_function(wrap_pyfunction!(login::login_with_password, m)?)?;
    m.add_function(wrap_pyfunction!(login::login_with_password_md5, m)?)?;
    m.add_function(wrap_pyfunction!(login::login_with_qrcode, m)?)?;
    // 客户端
    m.add_class::<client::Client>()?;
    // // 消息元素
    // m.add_class::<message::elements::At>()?;
    // m.add_class::<message::elements::Face>()?;
    // // 消息内容
    // m.add_class::<message::content::MessageContent>()?;
    Ok(())
}
