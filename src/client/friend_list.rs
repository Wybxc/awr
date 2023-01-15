//! 好友列表。
//!
//! 更多信息参考 [`FriendList`]。

use std::sync::Arc;

use pyo3::{prelude::*, types::*};

use crate::{
    client::{friend::Friend, friend_group::FriendGroup},
    utils::*,
};

/// 好友列表。
#[pyclass]
#[derive(Clone)]
pub struct FriendList {
    inner: Arc<libawr::client::friend_list::FriendList>,
}

impl From<Arc<libawr::client::friend_list::FriendList>> for FriendList {
    fn from(inner: Arc<libawr::client::friend_list::FriendList>) -> Self {
        Self { inner }
    }
}

impl_py_properties!(FriendList {
    total_count: i16 => i16,
    online_count: i16 => i16,
});
impl_remote_target!(FriendList, FriendListSelector);

#[pymethods]
impl FriendList {
    /// 获取好友信息。
    pub fn friends<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let friends: Vec<_> = self
            .inner
            .friends()
            .iter()
            .map(|(uin, info)| Ok((*uin, py_obj(Friend::from(info.clone()))?)))
            .collect::<PyResult<_>>()?;
        Ok(friends.into_py_dict(py))
    }

    /// 获取所有好友分组信息。
    pub fn friend_groups<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let friend_groups: Vec<_> = self
            .inner
            .friend_groups()
            .iter()
            .map(|(uin, info)| Ok((*uin, py_obj(FriendGroup::from(info.clone()))?)))
            .collect::<PyResult<_>>()?;
        Ok(friend_groups.into_py_dict(py))
    }
}

/// 好友列表选择器。
#[pyclass]
#[derive(Clone)]
pub struct FriendListSelector {
    inner: libawr::client::friend_list::FriendListSelector,
}

impl From<libawr::client::friend_list::FriendListSelector> for FriendListSelector {
    fn from(inner: libawr::client::friend_list::FriendListSelector) -> Self {
        Self { inner }
    }
}

impl_py_properties!(FriendListSelector {});
impl_single_selector!(FriendListSelector, FriendList);
