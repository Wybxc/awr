//! 好友分组

use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use ricq::{structs::FriendGroupInfo, RQError};
use thiserror::Error;

use crate::{
    client::{friend_list::FetchFriendListError, Client},
    meta::selector::{OptionSelector, Selector},
};

box_error_impl!(
    FetchFriendGroupError,
    FetchFriendGroupErrorImpl,
    "获取好友分组错误。"
);

/// 获取好友分组错误。
#[derive(Error, Debug)]
enum FetchFriendGroupErrorImpl {
    /// 获取好友列表失败。
    #[error("获取好友列表失败")]
    FetchFriendListError(#[from] FetchFriendListError),
}

/// 好友分组。
///
/// # Python
/// ```python
/// class FriendGroup:
///     @property
///     def id(self) -> int: ...
///     @property
///     def name(self) -> str: ...
///     @property
///     def friend_count(self) -> int: ...
///     @property
///     def online_count(self) -> int: ...
///     @property
///     def seq_id(self) -> int: ...
/// ```
#[derive(Debug, Clone)]
pub struct FriendGroup {
    selector: FriendGroupSelector,
    /// 好友分组编号。
    pub id: u8,
    /// 好友分组名称。
    pub name: String,
    /// 好友分组好友数。
    pub friend_count: i32,
    /// 在线好友数。
    pub online_count: i32,
    /// 好友分组排序。
    pub seq_id: u8,
}

impl FriendGroup {
    pub(crate) fn new(client: &Arc<Client>, info: FriendGroupInfo) -> Self {
        Self {
            selector: client.friend_group(info.group_id),
            id: info.group_id,
            name: info.group_name,
            friend_count: info.friend_count,
            online_count: info.online_friend_count,
            seq_id: info.seq_id,
        }
    }
}

impl Deref for FriendGroup {
    type Target = FriendGroupSelector;

    fn deref(&self) -> &Self::Target {
        &self.selector
    }
}

/// 好友分组选择器。
///
/// # Python
/// ```python
/// class FriendGroupSelector:
///     @property
///     def id(self) -> int: ...
/// ```
#[derive(Debug, Clone)]
pub struct FriendGroupSelector {
    client: Arc<Client>,
    /// 好友分组编号。
    pub id: u8,
}

impl FriendGroupSelector {
    pub(crate) fn new(client: Arc<Client>, id: u8) -> Self {
        Self { client, id }
    }

    /// 删除好友分组。
    ///
    /// 此方法会使好友列表缓存失效。
    ///
    /// # Python
    /// ```python
    /// async def delete(self) -> None: ...
    /// ```
    pub async fn delete(&self) -> Result<(), RQError> {
        self.client.inner.friend_list_del_group(self.id).await?;
        self.client.friend_list.make_dirty().await;
        Ok(())
    }

    /// 重命名好友分组。
    ///
    /// 此方法会使好友列表缓存失效。
    ///
    /// # Python
    /// ```python
    /// async def rename(self, new_name: str) -> None: ...
    /// ```
    pub async fn rename(&self, new_name: String) -> Result<(), RQError> {
        self.client
            .inner
            .friend_list_rename_group(self.id, new_name)
            .await?;
        self.client.friend_list.make_dirty().await;
        Ok(())
    }
}

#[async_trait]
impl Selector for FriendGroupSelector {
    type Target = Arc<FriendGroup>;
    type Error = FetchFriendGroupError;

    async fn flush(&self) -> &Self {
        self.client.friend_list.make_dirty().await;
        self
    }

    fn as_client(&self) -> &Arc<Client> {
        &self.client
    }
}

#[async_trait]
impl OptionSelector for FriendGroupSelector {
    async fn fetch(&self) -> Result<Option<Self::Target>, Self::Error> {
        Ok(self
            .client
            .get_friend_list()
            .await?
            .friend_groups()
            .get(&self.id)
            .cloned())
    }
}
