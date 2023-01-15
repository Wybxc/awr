//! 群成员列表。

use std::{backtrace::Backtrace, collections::HashMap, ops::Deref, sync::Arc};

use async_trait::async_trait;
use ricq::{structs::GroupMemberInfo, RQError};
use thiserror::Error;

use crate::meta::cache::MapCacheable;
use crate::{
    client::{group::FetchGroupInfoError, group_member::GroupMember},
    meta::selector::{OptionSelector, Selector},
    Client,
};

box_error_impl!(
    FetchGroupMemberListError,
    FetchGroupMemberListErrorImpl,
    "获取群成员列表错误。"
);

/// 获取群成员列表错误。
#[derive(Error, Debug)]
enum FetchGroupMemberListErrorImpl {
    /// 获取群信息错误。
    #[error("获取群信息错误")]
    FetchGroupInfoError(#[from] FetchGroupInfoError),

    /// 群不存在。
    #[error("群不存在")]
    GroupNotExist { backtrace: Backtrace },

    /// 获取群成员列表错误。
    #[error("获取群成员列表错误")]
    FetchGroupMemberListError {
        #[from]
        source: RQError,
        backtrace: Backtrace,
    },
}

/// 群成员列表。
#[derive(Debug, Clone)]
pub struct GroupMemberList {
    selector: GroupMemberListSelector,
    members: HashMap<i64, Arc<GroupMember>>,
    /// 群成员数量。
    pub total_count: i16,
}

impl GroupMemberList {
    pub(crate) fn new(
        client: &Arc<Client>,
        group_code: i64,
        members: Vec<GroupMemberInfo>,
    ) -> Self {
        let total_count = members.len() as i16;
        let members = members
            .into_iter()
            .map(|info| {
                let member = GroupMember::new(client, info);
                (member.uin, Arc::new(member))
            })
            .collect();
        Self {
            selector: GroupMemberListSelector::new(client.clone(), group_code),
            members,
            total_count,
        }
    }

    /// 获取所有群成员信息。
    ///
    /// # Python
    /// ```python
    /// def members(self) -> dict[int, GroupMember]:
    /// ```
    pub fn members(&self) -> &HashMap<i64, Arc<GroupMember>> {
        &self.members
    }
}

impl Deref for GroupMemberList {
    type Target = GroupMemberListSelector;
    fn deref(&self) -> &Self::Target {
        &self.selector
    }
}

#[async_trait]
impl MapCacheable for GroupMemberList {
    type Key = i64;
    type Error = FetchGroupMemberListError;

    async fn fetch_uncached(client: &Arc<Client>, code: &i64) -> Result<Option<Self>, Self::Error> {
        let group = client.group(*code).fetch().await?;
        if group.is_none() {
            return Err(FetchGroupMemberListErrorImpl::GroupNotExist {
                backtrace: Backtrace::capture(),
            }
            .into());
        }
        let group = group.unwrap();
        let owner_uin = group.owner_uin;
        let members = client.inner.get_group_member_list(*code, owner_uin).await?;
        Ok(Some(Self::new(client, *code, members)))
    }
}

/// 群成员列表选择器。
#[derive(Debug, Clone)]
pub struct GroupMemberListSelector {
    client: Arc<Client>,
    /// 群号。
    pub group_code: i64,
}

impl GroupMemberListSelector {
    pub(crate) fn new(client: Arc<Client>, group_code: i64) -> Self {
        Self { client, group_code }
    }
}

#[async_trait]
impl Selector for GroupMemberListSelector {
    type Target = Arc<GroupMemberList>;
    type Error = FetchGroupMemberListError;

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
impl OptionSelector for GroupMemberListSelector {
    async fn fetch(&self) -> Result<Option<Self::Target>, Self::Error> {
        self.client
            .group_member_lists
            .get(&self.client, &self.group_code)
            .await
    }
}
