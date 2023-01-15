//! 好友列表。

use std::{backtrace::Backtrace, collections::HashMap, ops::Deref, sync::Arc};

use async_trait::async_trait;
use ricq::RQError;
use ricq_core::command::friendlist::FriendListResponse;
use thiserror::Error;

use crate::meta::cache::Cacheable;
use crate::{
    client::{friend::Friend, friend_group::FriendGroup, Client},
    meta::selector::{Selector, SingleSelector},
};

box_error_impl!(
    FetchFriendListError,
    FetchFriendListErrorImpl,
    "获取好友列表错误。"
);

/// 获取好友列表错误。
#[derive(Error, Debug)]
#[error("获取好友列表错误")]
struct FetchFriendListErrorImpl {
    #[from]
    source: RQError,
    backtrace: Backtrace,
}

/// 好友列表。
///
/// # Python
/// ```python
/// class FriendList:
///     @property
///     def total_count(self) -> int: ...
///     @property
///     def online_count(self) -> int: ...
/// ```
#[derive(Debug, Clone)]
pub struct FriendList {
    selector: FriendListSelector,
    friends: HashMap<i64, Arc<Friend>>,
    /// 好友分组信息。
    friend_groups: HashMap<u8, Arc<FriendGroup>>,
    /// 好友数量。
    pub total_count: i16,
    /// 在线好友数量。
    pub online_count: i16,
}

impl FriendList {
    pub(crate) fn new(client: Arc<Client>, origin: FriendListResponse) -> Self {
        let friends = origin
            .friends
            .into_iter()
            .map(|info| (info.uin, Arc::new(Friend::new(&client, info))))
            .collect();
        let friend_groups = origin
            .friend_groups
            .into_iter()
            .map(|(id, info)| (id, Arc::new(FriendGroup::new(&client, info))))
            .collect();
        Self {
            selector: FriendListSelector::new(client),
            friends,
            friend_groups,
            total_count: origin.total_count,
            online_count: origin.online_friend_count,
        }
    }

    /// 获取好友信息。
    ///
    /// # Python
    /// ```python
    /// def friends(self) -> dict[int, Friend]: ...
    /// ```
    pub fn friends(&self) -> &HashMap<i64, Arc<Friend>> {
        &self.friends
    }

    /// 获取所有好友分组信息。
    ///
    /// # Python
    /// ```python
    /// def friend_groups(self) -> dict[int, FriendGroup]: ...
    /// ```
    pub fn friend_groups(&self) -> &HashMap<u8, Arc<FriendGroup>> {
        &self.friend_groups
    }
}

impl Deref for FriendList {
    type Target = FriendListSelector;
    fn deref(&self) -> &Self::Target {
        &self.selector
    }
}

#[async_trait]
impl Cacheable for FriendList {
    type Error = FetchFriendListError;
    /// 请求获取好友列表。
    async fn fetch_uncached(client: &Arc<Client>) -> Result<Self, Self::Error> {
        let origin = client.inner.get_friend_list().await?;
        Ok(Self::new(client.clone(), origin))
    }
}

/// 好友列表选择器。
///
/// # Python
/// ```python
/// class FriendListSelector:
///     ...
/// ```
#[derive(Debug, Clone)]
pub struct FriendListSelector {
    client: Arc<Client>,
}

impl FriendListSelector {
    pub(crate) fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Selector for FriendListSelector {
    type Target = Arc<FriendList>;
    type Error = FetchFriendListError;

    async fn flush(&self) -> &Self {
        self.client.friend_list.make_dirty().await;
        self
    }

    fn as_client(&self) -> &Arc<Client> {
        &self.client
    }
}

#[async_trait]
impl SingleSelector for FriendListSelector {
    async fn fetch(&self) -> Result<Self::Target, Self::Error> {
        Ok(self.client.friend_list.get(&self.client).await?)
    }
}
