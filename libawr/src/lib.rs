//! 基于 [ricq](https://docs.rs/ricq/latest/ricq/) 的 QQ 机器人框架，提供一致、易用、简洁的 API。
//!
//! 更多信息，请参考 [`login`] 和 [`Client`]。
//!
//! # Examples
//!
//! ## Rust
//!
//! ```rust,no_run
//! use libawr::{login, msg, Protocol};
//! use anyhow::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let (client, alive) = login!(12345678, password="******", protocol=Protocol::IPad).await?;
//!     client.friend(23456789).send(msg!("Hello, world!")).await?;
//!     alive.auto_reconnect().await?;
//!     unreachable!()
//! }
//! ```
//!
//! ## Python
//! ```python
//! import asyncio
//! import awr
//!
//! async def main():
//!     client, alive = await awr.login(12345678, password="******", protocol=awr.Protocol.IPad)
//!     await client.friend(23456789).send("Hello, world!")
//!     await alive.auto_reconnect()
//!
//! asyncio.run(main())
//! ```
//!
//! [`login`]: mod@crate::login
//! [`Client`]: crate::client::Client
#![feature(error_generic_member_access)]
#![feature(provide_any)]
#![deny(missing_docs)]

#[macro_use]
pub(crate) mod utils;

pub mod client;
pub mod consts;
pub mod device;
pub mod login;
pub mod message;
pub mod meta;

pub use client::Client;
pub use login::{login_with_password, login_with_password_md5, login_with_qrcode};
pub use ricq::Protocol;
