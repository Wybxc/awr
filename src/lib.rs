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

#![deny(missing_docs)]
#![allow(clippy::borrow_deref_ref)] // TODO: Remove this line when pyo3-asyncio is updated to 0.17

use pyo3::prelude::*;

use tracing::info;

pub mod client;
mod device;
pub mod login;
mod loguru;
// pub mod selector;
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
#[doc(hidden)]
pub fn init(module: &PyModule) -> PyResult<()> {
    // 设置日志输出
    loguru::init(module)?;

    // 打印版本信息
    info!("{}", LOGO);
    Ok(())
}

/// Avilla with Ricq.
#[pymodule]
#[doc(hidden)]
pub fn awr(_py: Python, m: &PyModule) -> PyResult<()> {
    // 初始化
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(loguru::getframe, m)?)?;
    // 登录方式
    m.add_class::<login::LoginMethod>()?;
    m.add_class::<login::Password>()?;
    m.add_class::<login::QrCode>()?;
    m.add_class::<login::Dynamic>()?;
    // 客户端
    m.add_class::<client::Client>()?;
    Ok(())
}
