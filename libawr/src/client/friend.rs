//! 好友。

use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use ricq::{structs::FriendInfo, RQError};
use thiserror::Error;

use crate::{
    client::{
        friend_group::FriendGroupSelector, friend_list::FetchFriendListError,
        message_receipt::MessageReceipt, Client,
    },
    message::MessageContent,
    meta::selector::{OptionSelector, Selector},
};

box_error_impl!(
    FetchFriendInfoError,
    FetchFriendInfoErrorImpl,
    "获取好友信息错误。"
);

/// 获取好友信息错误。
#[derive(Error, Debug)]
enum FetchFriendInfoErrorImpl {
    /// 获取好友列表失败。
    #[error("获取好友列表失败")]
    FetchFriendListError(#[from] FetchFriendListError),
}

/// 好友。
///
/// # Python
/// ```python
/// class Friend():
///     @property
///     def uin(self) -> int: ...
///     @property
///     def nickname(self) -> str: ...
///     @property
///     def remark(self) -> str: ...
///     @property
///     def face_id(self) -> int: ...
///     @property
///     def group_id(self) -> int: ...
/// ```
#[derive(Debug, Clone)]
pub struct Friend {
    selector: FriendSelector,
    /// 好友 QQ 号。
    pub uin: i64,
    /// 好友昵称。
    pub nickname: String,
    /// 好友备注。
    pub remark: String,
    /// 好友头像 ID。
    pub face_id: i16,
    /// 好友分组编号。
    pub group_id: u8,
}

impl Friend {
    pub(crate) fn new(client: &Arc<Client>, info: FriendInfo) -> Self {
        Self {
            selector: client.friend(info.uin),
            uin: info.uin,
            nickname: info.nick,
            remark: info.remark,
            face_id: info.face_id,
            group_id: info.group_id,
        }
    }

    /// 获取所在的好友分组选择器。
    ///
    /// # Python
    /// ```python
    /// def friend_group(self) -> FriendGroupSelector: ...
    /// ```
    pub fn friend_group(&self) -> FriendGroupSelector {
        self.selector.client.friend_group(self.group_id)
    }
}

impl Deref for Friend {
    type Target = FriendSelector;
    fn deref(&self) -> &Self::Target {
        &self.selector
    }
}

/// 好友选择器。
///
/// # Python
/// ```python
/// class FriendSelector():
///     @property
///     def uin(self) -> int: ...
/// ```
#[derive(Debug, Clone)]
pub struct FriendSelector {
    client: Arc<Client>,
    /// 好友 QQ 号。
    pub uin: i64,
}

impl FriendSelector {
    pub(crate) fn new(client: Arc<Client>, uin: i64) -> Self {
        Self { client, uin }
    }

    /// 发送好友戳一戳。
    ///
    /// # Python
    /// ```python
    /// async def poke(self) -> None: ...
    /// ```
    pub async fn poke(&self) -> Result<(), RQError> {
        self.client.inner.friend_poke(self.uin).await
    }

    /// 发送好友消息。
    ///
    /// # Python
    /// ```python
    /// @overload
    /// async def send(self, *message: str | Element) -> MessageReceipt: ...
    /// @overload
    /// async def send(self, message: MessageContent) -> MessageReceipt: ...
    /// ```
    pub async fn send(&self, message: MessageContent) -> Result<MessageReceipt, RQError> {
        let receipt = self
            .client
            .inner
            .send_friend_message(self.uin, message.into_inner())
            .await?;
        Ok(MessageReceipt::new_from_friend(self.clone(), receipt))
    }

    /// 撤回好友消息。
    ///
    /// # Python
    /// ```python
    /// async def recall(self, message_receipt: MessageReceipt) -> None:
    /// ```
    pub async fn recall(&self, message_receipt: MessageReceipt) -> Result<(), RQError> {
        let ricq::structs::MessageReceipt { seqs, rands, time } = message_receipt.inner;
        self.client
            .inner
            .recall_friend_message(self.uin, time, seqs, rands)
            .await
    }
}

#[async_trait]
impl Selector for FriendSelector {
    type Target = Arc<Friend>;
    type Error = FetchFriendInfoError;

    async fn flush(&self) -> &Self {
        self.client.friend_list.make_dirty().await;
        self
    }

    fn as_client(&self) -> &Arc<Client> {
        &self.client
    }
}

#[async_trait]
impl OptionSelector for FriendSelector {
    async fn fetch(&self) -> Result<Option<Self::Target>, Self::Error> {
        Ok(self
            .client
            .get_friend_list()
            .await?
            .friends()
            .get(&self.uin)
            .cloned())
    }
}
