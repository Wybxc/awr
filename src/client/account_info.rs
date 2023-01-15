//! 账号信息。

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct AccountInfo {
    pub(crate) inner: libawr::client::account_info::AccountInfo,
}

impl From<libawr::client::account_info::AccountInfo> for AccountInfo {
    fn from(inner: libawr::client::account_info::AccountInfo) -> Self {
        Self { inner }
    }
}

impl_py_properties!(AccountInfo {
    nickname: String => &str,
    age: u8 => u8,
    gender: u8 => u8,
});

#[pyclass]
#[derive(Clone)]
pub struct AccountInfoSelector {
    pub(crate) inner: libawr::client::account_info::AccountInfoSelector,
}

impl From<libawr::client::account_info::AccountInfoSelector> for AccountInfoSelector {
    fn from(inner: libawr::client::account_info::AccountInfoSelector) -> Self {
        Self { inner }
    }
}

impl_py_properties!(AccountInfoSelector {});
impl_single_selector!(AccountInfoSelector, AccountInfo);
