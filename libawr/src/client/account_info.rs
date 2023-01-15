//! 账号信息。
//!
//! 更多信息请参考 [`AccountInfo`]。

use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;

use crate::meta::selector::{Selector, SingleSelector};
use crate::Client;

box_error_impl!(
    ReadAccountInfoError,
    ReadAccountInfoErrorImpl,
    "读取账号信息失败"
);

#[derive(Debug, Error)]
enum ReadAccountInfoErrorImpl {}

/// 账号信息。
///
/// # Python
/// ```python
/// class AccountInfo:
///     @property
///     def nickname(self) -> str: ...
///     @property
///     def age(self) -> int: ...
///     @property
///     def gender(self) -> int: ...
/// ```
#[derive(Debug, Clone)]
pub struct AccountInfo {
    /// 昵称。
    pub nickname: String,
    /// 年龄。
    pub age: u8,
    /// 性别。
    pub gender: u8,
}

/// 账号信息选择器。
///
/// # Python
/// ```python
/// class AccountInfoSelector:
///     ...
/// ```
#[derive(Debug, Clone)]
pub struct AccountInfoSelector {
    client: Arc<Client>,
}

impl AccountInfoSelector {
    pub(crate) fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Selector for AccountInfoSelector {
    type Target = AccountInfo;
    type Error = ReadAccountInfoError;

    async fn flush(&self) -> &Self {
        self
    }

    fn as_client(&self) -> &Arc<Client> {
        &self.client
    }
}

#[async_trait]
impl SingleSelector for AccountInfoSelector {
    async fn fetch(&self) -> Result<Self::Target, Self::Error> {
        let account_info = self.client.inner.account_info.read().await;
        Ok(AccountInfo {
            nickname: account_info.nickname.clone(),
            age: account_info.age,
            gender: account_info.gender,
        })
    }
}
