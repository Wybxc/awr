//! 远程对象选择器。
//!
//! 选择器是 awr 用于操作远程对象的核心机制。
//!
//! 在 awr 中，好友信息、群信息等抽象为远程对象，而 [`Friend`] [`Group`] 等对象是远程对象的本地副本。
//! 选择器是指向远程对象的指针，可以用于下载本地副本，或者在不下载远程对象的情况下对远程对象进行操作。
//!
//! 以好友为例，下图展示了 awr 中的好友对象的选择器机制。
//!
//! ```mermaid
//! graph LR
//! Client -->|"friend(...)"| FriendSelector   
//! Client -->|"async get_friend(...)"| Friend
//! FriendSelector -->|"async fetch(...)"| Friend    
//! Friend -.->|方法委托| FriendSelector
//! FriendSelector -.->|"API 调用"| S([服务器])
//! S -.->|数据| Friend
//! ```
//!
//! [`Friend`]: crate::client::friend::Friend
//! [`Group`]: crate::client::group::Group

use std::sync::Arc;
use std::{collections::HashMap, hash::Hash};

use crate::Client;
use async_trait::async_trait;

/// 远程对象选择器。
///
/// # Python
/// ```python
/// class Selector(Protocol, Generic[Target]): ...
/// ```
#[async_trait]
pub trait Selector: Clone + Sync + Send {
    /// 选择的对象类型。
    type Target;
    /// 错误类型。
    type Error;

    /// 刷新缓存。
    ///
    /// # Python
    /// ```python
    /// async def flush(self) -> Self:
    /// ```
    async fn flush(&self) -> &Self;

    /// 获取客户端引用。
    ///
    /// # Python
    /// ```python
    /// def as_client(self) -> Client: ...
    /// ```
    fn as_client(&self) -> &Arc<Client>;

    /// 获取选择器。
    ///
    /// # Python
    /// ```python
    /// def as_selector(self) -> Selector: ...
    /// ```
    fn as_selector(&self) -> &Self {
        self
    }
}

/// 单对象选择器。
///
/// # Python
/// ```python
/// class SingleSelector(Selector[Target]): ...
/// ```
#[async_trait]
pub trait SingleSelector: Selector {
    /// 获取远程对象。
    ///
    /// # Python
    /// ```python
    /// async def fetch(self) -> Target | None:
    /// ```
    async fn fetch(&self) -> Result<Self::Target, Self::Error>;

    /// 刷新缓存并获取远程对象。
    ///
    /// # Python
    /// ```python
    /// async def flush_and_fetch(self) -> Target | None:
    /// ```
    async fn flush_and_fetch(&self) -> Result<Self::Target, Self::Error> {
        self.flush().await.fetch().await
    }
}

/// 可空对象选择器。
///
/// # Python
/// ```python
/// class OptionSelector(Selector[Target]): ...
/// ```
#[async_trait]
pub trait OptionSelector: Selector {
    /// 获取远程对象。
    ///
    /// # Python
    /// ```python
    /// async def fetch(self) -> Target | None:
    /// ```
    async fn fetch(&self) -> Result<Option<Self::Target>, Self::Error>;

    /// 刷新缓存并获取远程对象。
    ///
    /// # Python
    /// ```python
    /// async def flush_and_fetch(self) -> Target | None:
    /// ```
    async fn flush_and_fetch(&self) -> Result<Option<Self::Target>, Self::Error> {
        self.flush().await.fetch().await
    }
}

/// 多个远程对象选择器。
///
/// # Python
/// ```python
/// class MultiSelector(Protocol, Generic[Key, Target]): ...
/// ```
#[async_trait]
pub trait MultiSelector: Selector {
    /// 远程对象键类型。
    type Key: Clone + Hash + Eq;

    /// 获取远程对象。
    ///
    /// # Python
    /// ```python
    /// async def fetch(self) -> dict[Key, Target]:
    /// ```
    async fn fetch(&self) -> Result<HashMap<Self::Key, Self::Target>, Self::Error>;

    /// 刷新缓存并获取远程对象。
    ///
    /// # Python
    /// ```python
    /// async def flush_and_fetch(self) -> dict[Key, Target]:
    /// ```
    async fn flush_and_fetch(&self) -> Result<HashMap<Self::Key, Self::Target>, Self::Error> {
        self.flush().await;
        self.fetch().await
    }
}
