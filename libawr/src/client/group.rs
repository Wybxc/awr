//! 群。

use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use crate::client::group_member::GroupMemberSelector;
use crate::client::group_member_list::GroupMemberListSelector;
use crate::meta::cache::{AllCacheable, BatchCacheable, MapCacheable};
use crate::meta::selector::{MultiSelector, OptionSelector, Selector};
use crate::Client;
use async_trait::async_trait;
use ricq::structs::GroupInfo;
use ricq::RQError;
use thiserror::Error;

box_error_impl!(
    FetchGroupInfoError,
    FetchGroupInfoErrorImpl,
    "获取群信息失败。"
);

/// 获取群信息失败。
#[derive(Error, Debug)]
enum FetchGroupInfoErrorImpl {
    /// 获取群列表失败。
    #[error("获取群列表失败")]
    FetchGroupListError {
        #[from]
        /// 原始错误。
        source: RQError,
        /// 错误堆栈。
        backtrace: Backtrace,
    },
}

/// 群聊。
///
/// # Python
/// ```python
/// class Group:
///     @property
///     def uin(self) -> int: ...
///     @property
///     def code(self) -> int: ...
///     @property
///     def name(self) -> str: ...
///     @property
///     def memo(self) -> str: ...
///     @property
///     def owner_uin(self) -> int: ...
///     @property
///     def group_create_time(self) -> int: ...
///     @property
///     def group_level(self) -> int: ...
///     @property
///     def member_count(self) -> int: ...
///     @property
///     def max_member_count(self) -> int: ...
///     @property
///     def shut_up_timestamp(self) -> int: ...
///     @property
///     def my_shut_up_timestamp(self) -> int: ...
///     @property
///     def last_msg_seq(self) -> int | None: ...
/// ```
#[derive(Debug, Clone)]
pub struct Group {
    selector: GroupSelector,
    /// uin。
    ///
    /// 含义可参考：[#181](https://github.com/Mrs4s/MiraiGo/issues/181)。
    pub uin: i64,
    /// 群号。
    pub code: i64,
    /// 群名称。
    pub name: String,
    /// 入群公告。
    pub memo: String,
    /// 群主 QQ 号。
    pub owner_uin: i64,
    /// 群创建时间。
    pub group_create_time: u32,
    /// 群等级。
    pub group_level: u32,
    /// 群成员数。
    pub member_count: u16,
    /// 最大群成员数。
    pub max_member_count: u16,
    /// 全群禁言时间。
    pub shut_up_timestamp: i64,
    /// 自己被禁言时间。
    pub my_shut_up_timestamp: i64,
    /// 最后一条消息的 seq。
    pub last_msg_seq: Option<i64>,
}

impl Group {
    pub(crate) fn new(client: &Arc<Client>, info: GroupInfo) -> Self {
        Self {
            selector: client.group(info.code),
            uin: info.uin,
            code: info.code,
            name: info.name,
            memo: info.memo,
            owner_uin: info.owner_uin,
            group_create_time: info.group_create_time,
            group_level: info.group_level,
            member_count: info.member_count,
            max_member_count: info.max_member_count,
            shut_up_timestamp: info.shut_up_timestamp,
            my_shut_up_timestamp: info.my_shut_up_timestamp,
            last_msg_seq: Some(info.last_msg_seq),
        }
    }

    #[allow(dead_code)] // TODO: remove this
    pub(crate) fn new_without_last_seq(client: &Arc<Client>, info: GroupInfo) -> Self {
        Self {
            selector: client.group(info.code),
            uin: info.uin,
            code: info.code,
            name: info.name,
            memo: info.memo,
            owner_uin: info.owner_uin,
            group_create_time: info.group_create_time,
            group_level: info.group_level,
            member_count: info.member_count,
            max_member_count: info.max_member_count,
            shut_up_timestamp: info.shut_up_timestamp,
            my_shut_up_timestamp: info.my_shut_up_timestamp,
            last_msg_seq: None,
        }
    }
}

impl Deref for Group {
    type Target = GroupSelector;

    fn deref(&self) -> &Self::Target {
        &self.selector
    }
}

#[async_trait]
impl MapCacheable for Group {
    type Key = i64;
    type Error = FetchGroupInfoError;

    async fn fetch_uncached(client: &Arc<Client>, code: &i64) -> Result<Option<Self>, Self::Error> {
        if let Some(group_info) = client.inner.get_group_info(*code).await? {
            Ok(Some(Group::new(client, group_info)))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl BatchCacheable for Group {
    async fn fetch_uncached_batch(
        client: &Arc<Client>,
        codes: &[i64],
    ) -> Result<Vec<(Self::Key, Self)>, Self::Error> {
        let group_infos = client.inner.get_group_infos(codes.to_vec()).await?;
        Ok(group_infos
            .into_iter()
            .map(|info| (info.code, Group::new(client, info)))
            .collect())
    }
}

#[async_trait]
impl AllCacheable for Group {
    async fn fetch_uncached_all(
        client: &Arc<Client>,
    ) -> Result<Vec<(Self::Key, Self)>, Self::Error> {
        let group_infos = client.inner.get_group_list().await?;
        Ok(group_infos
            .into_iter()
            .map(|info| (info.code, Group::new(client, info)))
            .collect())
    }
}

/// 群聊选择器。
///
/// # Python
/// ```python
/// class GroupSelector:
///     @property
///     def code(self) -> int: ...
/// ```
#[derive(Debug, Clone)]
pub struct GroupSelector {
    client: Arc<Client>,
    /// 群号。
    pub code: i64,
}

impl GroupSelector {
    pub(crate) fn new(client: Arc<Client>, code: i64) -> Self {
        Self { client, code }
    }

    /// 获取群成员列表选择器。
    ///
    /// # Python
    /// ```python
    /// def member_list(self) -> GroupMemberListSelector: ...
    /// ```
    pub fn member_list(&self) -> GroupMemberListSelector {
        GroupMemberListSelector::new(self.client.clone(), self.code)
    }

    /// 构造群成员选择器。
    ///
    /// # Python
    /// ```python
    /// def member(self, uin: int) -> GroupMemberSelector: ...
    /// ```
    pub fn member(&self, uin: i64) -> GroupMemberSelector {
        GroupMemberSelector::new(self.client.clone(), self.code, uin)
    }
}

#[async_trait]
impl Selector for GroupSelector {
    type Target = Arc<Group>;
    type Error = FetchGroupInfoError;

    async fn flush(&self) -> &Self {
        self.client.groups.make_dirty(&self.code).await;
        self
    }

    fn as_client(&self) -> &Arc<Client> {
        &self.client
    }
}

#[async_trait]
impl OptionSelector for GroupSelector {
    async fn fetch(&self) -> Result<Option<Self::Target>, Self::Error> {
        self.client.groups.get(&self.client, &self.code).await
    }
}

/// 多个群聊选择器。
///
/// # Python
/// ```python
/// class MultiGroupSelector:
///     ...
/// ```
#[derive(Debug, Clone)]
pub struct MultiGroupSelector {
    client: Arc<Client>,
    codes: Vec<i64>,
}

impl MultiGroupSelector {
    pub(crate) fn new(client: Arc<Client>, codes: Vec<i64>) -> Self {
        Self { client, codes }
    }

    /// 群号列表。
    ///
    /// # Python
    /// ```python
    /// def codes(self) -> list[int]: ...
    /// ```
    pub fn codes(&self) -> &Vec<i64> {
        &self.codes
    }
}

#[async_trait]
impl Selector for MultiGroupSelector {
    type Target = Arc<Group>;
    type Error = FetchGroupInfoError;

    async fn flush(&self) -> &Self {
        self.client.groups.make_dirty_batch(&self.codes).await;
        self
    }

    fn as_client(&self) -> &Arc<Client> {
        &self.client
    }
}

#[async_trait]
impl MultiSelector for MultiGroupSelector {
    type Key = i64;

    async fn fetch(&self) -> Result<HashMap<i64, Arc<Group>>, Self::Error> {
        self.client
            .groups
            .get_batch(&self.client, &self.codes)
            .await
    }
}

/// 所有群聊选择器。
///
/// # Python
/// ```python
/// class AllGroupSelector: ...
///    ...
/// ```
#[derive(Debug, Clone)]
pub struct AllGroupSelector {
    client: Arc<Client>,
}

impl AllGroupSelector {
    pub(crate) fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Selector for AllGroupSelector {
    type Target = Arc<Group>;
    type Error = FetchGroupInfoError;

    async fn flush(&self) -> &Self {
        self.client.groups.make_dirty_all().await;
        self
    }

    fn as_client(&self) -> &Arc<Client> {
        &self.client
    }
}

#[async_trait]
impl MultiSelector for AllGroupSelector {
    type Key = i64;

    async fn fetch(&self) -> Result<HashMap<i64, Arc<Group>>, Self::Error> {
        self.client.groups.refresh_all(&self.client).await
    }
}
