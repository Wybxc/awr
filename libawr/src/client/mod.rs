//! 客户端。
//!
//! 更多信息，请参考 [`Client`]。

pub mod account_info;
pub mod friend;
pub mod friend_group;
pub mod friend_list;
pub mod group;
pub mod group_member;
pub mod group_member_list;
pub mod message_receipt;

use std::{collections::HashMap, sync::Arc, time::Duration};

use ricq::RQError;

use self::{
    friend::FriendSelector,
    friend_group::FriendGroupSelector,
    friend_list::{FetchFriendListError, FriendList},
};
use crate::client::group_member_list::{FetchGroupMemberListError, GroupMemberListSelector};
use crate::meta::cache::{Cached, CachedMap};
use crate::{
    client::{
        account_info::{AccountInfo, AccountInfoSelector, ReadAccountInfoError},
        group_member::{FetchGroupMemberInfoError, GroupMember},
    },
    meta::selector::SingleSelector,
};
use crate::{
    client::{
        friend::FetchFriendInfoError,
        friend_group::{FetchFriendGroupError, FriendGroup},
        friend_list::FriendListSelector,
        group::{AllGroupSelector, FetchGroupInfoError, Group, GroupSelector, MultiGroupSelector},
        group_member::GroupMemberSelector,
        group_member_list::GroupMemberList,
    },
    consts::*,
    meta::selector::{MultiSelector, OptionSelector},
};

/// 客户端。
///
/// # Python
/// ```python
/// class Client:
///     @property
///     def uin(self) -> int:
/// ```
pub struct Client {
    pub(crate) inner: Arc<ricq::Client>,
    /// 当前账号的 QQ 号。
    pub uin: i64,
    pub(crate) friend_list: Cached<FriendList>,
    pub(crate) groups: CachedMap<Group>,
    pub(crate) group_member_lists: CachedMap<GroupMemberList>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").field("uin", &self.uin).finish()
    }
}

impl Client {
    pub(crate) async fn new(client: Arc<ricq::Client>) -> Self {
        let uin = client.uin().await;
        Self {
            inner: client,
            uin,
            friend_list: Cached::new(FRIEND_LIST_CACHE_TIME),
            groups: CachedMap::new(GROUP_CACHE_TIME),
            group_member_lists: CachedMap::new(GROUP_MEMBER_LIST_CACHE_TIME),
        }
    }

    /// 设置好友列表缓存过期时间。
    ///
    /// # Python
    /// ```python
    /// async def set_friend_list_cache_time(self, time: datetime.timedelta) -> None: ...
    /// ```
    pub async fn set_friend_list_cache_time(&self, time: Duration) {
        self.friend_list.set_cache_time(time).await;
    }

    /// 设置群信息缓存过期时间。
    ///
    /// # Python
    /// ```python
    /// async def set_group_cache_time(self, time: datetime.timedelta) -> None: ...
    /// ```
    pub async fn set_group_cache_time(&self, time: Duration) {
        self.groups.set_cache_time(time).await;
    }

    /// 设置群成员列表缓存过期时间。
    ///
    /// # Python
    /// ```python
    /// async def set_group_member_list_cache_time(self, time: datetime.timedelta) -> None: ...
    /// ```
    pub async fn set_group_member_list_cache_time(&self, time: Duration) {
        self.group_member_lists.set_cache_time(time).await;
    }

    /// 当前账号是否在线。
    ///
    /// # Python
    /// ```python    
    /// def is_online(self) -> bool: ...
    /// ```
    pub fn is_online(&self) -> bool {
        self.inner.online.load(std::sync::atomic::Ordering::Acquire)
    }

    /// 构造账号信息选择器。
    ///
    /// # Python
    /// ```python
    /// def account_info(self) -> AccountInfoSelector: ...
    /// ```
    pub fn account_info(self: &Arc<Self>) -> AccountInfoSelector {
        AccountInfoSelector::new(self.clone())
    }

    /// 获取账号信息。
    ///
    /// # Python
    /// ```python
    /// async def get_account_info(self) -> AccountInfo: ...
    /// ```
    pub async fn get_account_info(self: &Arc<Self>) -> Result<AccountInfo, ReadAccountInfoError> {
        self.account_info().fetch().await
    }

    /// 构造好友分组选择器。
    ///
    /// # Python
    /// ```python
    /// def friend_group(self, group_id: int) -> FriendGroupSelector: ...
    /// ```
    pub fn friend_list(self: &Arc<Self>) -> FriendListSelector {
        FriendListSelector::new(self.clone())
    }

    /// 获取好友列表对象。
    ///
    /// 好友列表会被缓存，如果缓存未过期则直接返回缓存的值。
    /// 如果需要强制刷新好友列表缓存，请使用 [`flush_friend_list`]。
    ///
    /// # Python
    /// ```python
    /// async def get_friend_list(self) -> FriendList: ...
    /// ```
    ///
    /// [`flush_friend_list`]: Self::flush_friend_list
    pub async fn get_friend_list(
        self: &Arc<Self>,
    ) -> Result<Arc<FriendList>, FetchFriendListError> {
        self.friend_list.get(self).await
    }

    /// 刷新好友列表缓存。
    ///
    /// # Python
    /// ```python
    /// async def flush_friend_list(self) -> None: ...
    /// ```
    pub async fn flush_friend_list(self: &Arc<Self>) -> Result<(), FetchFriendListError> {
        self.friend_list.make_dirty().await;
        Ok(())
    }

    /// 构造好友选择器。
    ///
    /// # Python
    /// ```python
    /// def friend(self, uin: int) -> FriendSelector:
    /// ```
    pub fn friend(self: &Arc<Self>, uin: i64) -> FriendSelector {
        FriendSelector::new(self.clone(), uin)
    }

    /// 获取好友对象。
    ///
    /// 好友对象会缓存在好友列表缓存中，如果缓存未过期则直接返回缓存的值。
    /// 如果需要强制刷新好友对象缓存，请使用 [`FriendSelector::flush`] 或 [`flush_friend_list`]。
    ///
    /// # Python
    /// ```python
    /// async def get_friend(self, uin: int) -> Friend | None: ...
    /// ```
    ///
    /// [`FriendSelector::flush`]: crate::meta::selector::Selector::flush
    /// [`flush_friend_list`]: Self::flush_friend_list
    pub async fn get_friend(
        self: &Arc<Self>,
        uin: i64,
    ) -> Result<Option<Arc<friend::Friend>>, FetchFriendInfoError> {
        self.friend(uin).fetch().await
    }

    /// 构造好友分组选择器。
    ///
    /// # Python
    /// ```python
    /// def friend_group(self, group_id: int) -> FriendGroupSelector: ...
    /// ```
    pub fn friend_group(self: &Arc<Self>, id: u8) -> FriendGroupSelector {
        FriendGroupSelector::new(self.clone(), id)
    }

    /// 获取好友分组对象。
    ///
    /// 好友分组对象会缓存在好友列表缓存中，如果缓存未过期则直接返回缓存的值。
    /// 如果需要强制刷新好友分组对象缓存，请使用 [`FriendGroupSelector::flush`] 或 [`flush_friend_list`]。
    ///
    /// # Python
    /// ```python
    /// async def get_friend_group(self, id: int) -> FriendGroup | None: ...
    /// ```
    ///
    /// [`FriendGroupSelector::flush`]: crate::meta::selector::Selector::flush
    /// [`flush_friend_list`]: Self::flush_friend_list
    pub async fn get_friend_group(
        self: &Arc<Self>,
        id: u8,
    ) -> Result<Option<Arc<FriendGroup>>, FetchFriendGroupError> {
        self.friend_group(id).fetch().await
    }

    /// 创建好友分组。
    ///
    /// 此方法会强制更新好友列表缓存。
    ///
    /// # Python
    /// ```python
    /// async def create_friend_group(self, name: str) -> None: ...
    /// ```    
    pub async fn create_friend_group(self: &Arc<Self>, name: String) -> Result<(), RQError> {
        // https://github.com/takayama-lily/oicq/blob/870652fbabc688371372aeec775c4233dbb770bc/lib/internal/internal.ts#L134
        self.inner.friend_list_add_group(0xd, name).await?;
        self.friend_list.make_dirty().await;
        Ok(())
    }

    /// 构造群选择器。
    ///
    /// # Python
    /// ```python
    /// def group(self, code: int) -> GroupSelector: ...
    /// ```
    pub fn group(self: &Arc<Self>, code: i64) -> GroupSelector {
        GroupSelector::new(self.clone(), code)
    }

    /// 获取群对象。
    ///
    /// 群对象会被缓存，如果缓存未过期则直接返回缓存的值。
    /// 如果需要强制刷新群对象缓存，请使用 [`GroupSelector::flush`]。
    ///
    /// # Python
    /// ```python
    /// async def get_group(self, code: int) -> Group | None: ...
    /// ```
    ///
    /// [`GroupSelector::flush`]: crate::meta::selector::Selector::flush
    pub async fn get_group(
        self: &Arc<Self>,
        code: i64,
    ) -> Result<Option<Arc<Group>>, FetchGroupInfoError> {
        self.group(code).fetch().await
    }

    /// 构造多个群选择器。
    ///
    /// # Python
    /// ```python
    /// def groups(self, *codes: int) -> MultiGroupSelector: ...
    /// ```
    pub fn groups(self: &Arc<Self>, codes: Vec<i64>) -> MultiGroupSelector {
        MultiGroupSelector::new(self.clone(), codes)
    }

    /// 获取多个群对象。
    ///
    /// 群对象会被缓存，如果缓存未过期则直接返回缓存的值。
    /// 如果需要强制刷新群对象缓存，请使用 [`MultiGroupSelector::flush`]。
    ///
    /// # Python
    /// ```python
    /// async def get_groups(self, *codes: int) -> dict[int, Group]: ...
    /// ```
    ///
    /// [`MultiGroupSelector::flush`]: crate::meta::selector::Selector::flush
    pub async fn get_groups(
        self: &Arc<Self>,
        codes: Vec<i64>,
    ) -> Result<HashMap<i64, Arc<Group>>, FetchGroupInfoError> {
        self.groups(codes).fetch().await
    }

    /// 构造所有群选择器。
    ///
    /// # Python
    /// ```python
    /// def all_groups(self) -> AllGroupSelector: ...
    /// ```
    pub fn all_groups(self: &Arc<Self>) -> AllGroupSelector {
        AllGroupSelector::new(self.clone())
    }

    /// 获取所有群对象。
    ///
    /// 此方法会刷新所有群对象的缓存。
    ///
    /// # Python
    /// ```python
    /// async def get_all_groups(self) -> dict[int, Group]: ...
    /// ```
    pub async fn get_all_groups(
        self: &Arc<Self>,
    ) -> Result<HashMap<i64, Arc<Group>>, FetchGroupInfoError> {
        self.all_groups().fetch().await
    }

    /// 构造群成员选择器。
    ///
    /// # Python
    /// ```python
    /// def group_member(self, group_code: int, uin: int) -> GroupMemberSelector: ...
    /// ```
    pub fn group_member(self: &Arc<Self>, group_code: i64, uin: i64) -> GroupMemberSelector {
        GroupMemberSelector::new(self.clone(), group_code, uin)
    }

    /// 获取群成员对象。
    ///
    /// 群成员对象会被缓存，如果缓存未过期则直接返回缓存的值。
    /// 如果需要强制刷新群成员对象缓存，请使用 [`GroupMemberSelector::flush`]。
    ///
    /// # Python
    /// ```python
    /// async def get_group_member(self, group_code: int, uin: int) -> GroupMember | None: ...
    /// ```
    ///
    /// [`GroupMemberSelector::flush`]: crate::meta::selector::Selector::flush
    pub async fn get_group_member(
        self: &Arc<Self>,
        group_code: i64,
        uin: i64,
    ) -> Result<Option<Arc<GroupMember>>, FetchGroupMemberInfoError> {
        self.group_member(group_code, uin).fetch().await
    }

    /// 构造群成员列表选择器。
    ///
    /// # Python
    /// ```python
    /// def group_member_list(self, group_code: int) -> GroupMemberListSelector: ...
    /// ```
    pub fn group_member_list(self: &Arc<Self>, group_code: i64) -> GroupMemberListSelector {
        GroupMemberListSelector::new(self.clone(), group_code)
    }

    /// 获取群成员列表。
    ///
    /// 群成员列表会被缓存，如果缓存未过期则直接返回缓存的值。
    /// 如果需要强制刷新群成员列表缓存，请使用 [`GroupMemberListSelector::flush`]。
    ///
    /// # Python
    /// ```python
    /// async def get_group_member_list(self, group_code: int) -> GroupMemberList | None: ...
    /// ```
    ///
    /// [`GroupMemberListSelector::flush`]: crate::meta::selector::Selector::flush
    pub async fn get_group_member_list(
        self: &Arc<Self>,
        group_code: i64,
    ) -> Result<Option<Arc<GroupMemberList>>, FetchGroupMemberListError> {
        self.group_member_list(group_code).fetch().await
    }
}
