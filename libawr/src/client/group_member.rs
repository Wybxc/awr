//! 群成员。

use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use ricq::structs::{GroupMemberInfo, GroupMemberPermission};
use thiserror::Error;

use crate::{
    client::{group::FetchGroupInfoError, group_member_list::FetchGroupMemberListError},
    meta::selector::{OptionSelector, Selector},
    Client,
};

box_error_impl!(
    FetchGroupMemberInfoError,
    FetchGroupMemberInfoErrorImpl,
    "获取群成员信息失败。"
);

/// 获取群成员信息失败。
#[derive(Error, Debug)]
enum FetchGroupMemberInfoErrorImpl {
    /// 获取群信息失败。
    #[error("获取群信息失败")]
    FetchGroupInfoError(#[from] FetchGroupInfoError),
    /// 获取群成员列表失败。
    #[error("获取群成员列表失败")]
    FetchGroupMemberListError(#[from] FetchGroupMemberListError),
}

/// 群成员。
/// 
/// # Python
/// ```python
/// class GroupMember:
///     @property
///     def group_code(self) -> int: ...
///     @property
///     def uin(self) -> int: ...
///     @property
///     def gender(self) -> int: ...
///     @property
///     def nickname(self) -> str: ...
///     @property
///     def card_name(self) -> str: ...
///     @property
///     def level(self) -> int: ...
///     @property
///     def join_time(self) -> int: ...
///     @property
///     def last_speak_time(self) -> int: ...
///     @property
///     def special_title(self) -> str: ...
///     @property
///     def special_title_expire_time(self) -> int: ...
///     @property
///     def shut_up_timestamp(self) -> int: ...
///     @property
///     def permission(self) -> GroupMemberPermission: ...
/// ```
#[derive(Debug, Clone)]
pub struct GroupMember {
    selector: GroupMemberSelector,
    /// 群号。
    pub group_code: i64,
    /// QQ 号。
    pub uin: i64,
    /// 性别。
    pub gender: u8,
    /// 昵称。
    pub nickname: String,
    /// 群名片。
    pub card_name: String,
    /// 群等级。
    pub level: u16,
    /// 加群时间。
    pub join_time: i64,
    /// 最后发言时间。
    pub last_speak_time: i64,
    /// 特殊头衔。
    pub special_title: String,
    /// 特殊头衔过期时间。
    pub special_title_expire_time: i64,
    /// 剩余禁言时间。
    pub shut_up_timestamp: i64,
    /// 群成员权限。
    pub permission: GroupMemberPermission,
}

impl GroupMember {
    pub(crate) fn new(client: &Arc<Client>, info: GroupMemberInfo) -> Self {
        Self {
            selector: client.group(info.group_code).member(info.uin),
            group_code: info.group_code,
            uin: info.uin,
            gender: info.gender,
            nickname: info.nickname,
            card_name: info.card_name,
            level: info.level,
            join_time: info.join_time,
            last_speak_time: info.last_speak_time,
            special_title: info.special_title,
            special_title_expire_time: info.special_title_expire_time,
            shut_up_timestamp: info.shut_up_timestamp,
            permission: info.permission,
        }
    }
}

impl Deref for GroupMember {
    type Target = GroupMemberSelector;
    fn deref(&self) -> &Self::Target {
        &self.selector
    }
}

/// 群成员选择器。
/// 
/// # Python
/// ```python
/// class GroupMemberSelector:
///     @property
///     def group_code(self) -> int: ...
///     @property
///     def uin(self) -> int: ...
/// ```
#[derive(Debug, Clone)]
pub struct GroupMemberSelector {
    client: Arc<Client>,
    /// 群号。
    pub group_code: i64,
    /// 成员 QQ 号。
    pub uin: i64,
}

impl GroupMemberSelector {
    pub(crate) fn new(client: Arc<Client>, group_code: i64, uin: i64) -> Self {
        Self {
            client,
            group_code,
            uin,
        }
    }
}

#[async_trait]
impl Selector for GroupMemberSelector {
    type Target = Arc<GroupMember>;
    type Error = FetchGroupMemberInfoError;

    async fn flush(&self) -> &Self {
        self.client
            .group_member_lists
            .make_dirty(&self.group_code)
            .await;
        self
    }

    fn as_client(&self) -> &Arc<Client> {
        &self.client
    }
}

#[async_trait]
impl OptionSelector for GroupMemberSelector {
    async fn fetch(&self) -> Result<Option<Self::Target>, Self::Error> {
        Ok(self
            .client
            .group_member_lists
            .get(&self.client, &self.group_code)
            .await?
            .unwrap()
            .members()
            .get(&self.uin)
            .cloned())
    }
}
