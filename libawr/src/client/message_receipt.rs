//! 消息回执。

use ricq::{structs::MessageReceipt as Receipt, RQError};

use crate::client::friend::FriendSelector;

#[derive(Debug, Clone)]
enum MessageReceiptContext {
    #[allow(unused)] // TODO: remove this
    Group {
        group_id: i64,
        target_id: i64,
    },
    Friend(FriendSelector),
}

/// 消息回执，可以用于撤回消息。
#[derive(Debug, Clone)]
pub struct MessageReceipt {
    context: MessageReceiptContext,
    pub(crate) inner: Receipt,
}

impl MessageReceipt {
    pub(crate) fn new_from_friend(selector: FriendSelector, receipt: Receipt) -> Self {
        Self {
            context: MessageReceiptContext::Friend(selector),
            inner: receipt,
        }
    }

    /// 消息发送时间。
    pub fn time(&self) -> i64 {
        self.inner.time
    }

    /// 撤回消息。
    pub async fn recall(self) -> Result<(), RQError> {
        match self.context.clone() {
            MessageReceiptContext::Friend(selector) => selector.recall(self).await,
            _ => unimplemented!(),
        }
    }
}
