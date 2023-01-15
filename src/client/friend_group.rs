//! 好友分组。

use std::sync::Arc;

use pyo3::prelude::*;

use crate::utils::py_future;

/// 好友分组。
#[pyclass]
#[derive(Clone)]
pub struct FriendGroup {
    pub(crate) inner: Arc<libawr::client::friend_group::FriendGroup>,
}

impl From<Arc<libawr::client::friend_group::FriendGroup>> for FriendGroup {
    fn from(inner: Arc<libawr::client::friend_group::FriendGroup>) -> Self {
        Self { inner }
    }
}

impl_py_properties!(FriendGroup {
    id: u8 => u8,
    name: String => &str,
    friend_count: i32 => i32,
    online_count: i32 => i32,
    seq_id: u8 => u8,
});
impl_remote_target!(FriendGroup, FriendGroupSelector);

#[pyclass]
#[derive(Clone)]
pub struct FriendGroupSelector {
    pub(crate) inner: libawr::client::friend_group::FriendGroupSelector,
}

impl From<libawr::client::friend_group::FriendGroupSelector> for FriendGroupSelector {
    fn from(inner: libawr::client::friend_group::FriendGroupSelector) -> Self {
        Self { inner }
    }
}

impl_py_properties!(FriendGroupSelector {
    id: u8 => u8,
});
impl_option_selector!(FriendGroupSelector, FriendGroup);

#[pymethods]
impl FriendGroupSelector {
    /// 删除好友分组。
    pub fn delete<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let inner = self.inner.clone();
        py_future(py, async move {
            inner.delete().await?;
            Ok(())
        })
    }

    /// 重命名好友分组。
    pub fn rename<'py>(&self, py: Python<'py>, name: String) -> PyResult<&'py PyAny> {
        let inner = self.inner.clone();
        py_future(py, async move {
            inner.rename(name).await?;
            Ok(())
        })
    }
}
