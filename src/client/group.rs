//! 群。

use std::sync::Arc;

use pyo3::{prelude::*, types::IntoPyDict};

use crate::client::{
    group_member::GroupMemberSelector, group_member_list::GroupMemberListSelector,
};

/// 群聊。
#[pyclass]
#[derive(Clone)]
pub struct Group {
    inner: Arc<libawr::client::group::Group>,
}

impl From<Arc<libawr::client::group::Group>> for Group {
    fn from(inner: Arc<libawr::client::group::Group>) -> Self {
        Self { inner }
    }
}

impl_py_properties!(Group {
    uin: i64 => i64,
    code: i64 => i64,
    name: String => &str,
    memo: String => &str,
    owner_uin: i64 => i64,
    group_create_time: u32 => u32,
    group_level: u32 => u32,
    member_count: u16 => u16,
    max_member_count: u16 => u16,
    shut_up_timestamp: i64 => i64,
    my_shut_up_timestamp: i64 => i64,
    last_msg_seq: Option<i64> => Option<i64>,
});
impl_remote_target!(Group, GroupSelector);

/// 群聊选择器。
#[pyclass]
#[derive(Clone)]
pub struct GroupSelector {
    inner: libawr::client::group::GroupSelector,
}

impl From<libawr::client::group::GroupSelector> for GroupSelector {
    fn from(inner: libawr::client::group::GroupSelector) -> Self {
        Self { inner }
    }
}

impl_py_properties!(GroupSelector {
    code: i64 => i64,
});
impl_option_selector!(GroupSelector, Group);

#[pymethods]
impl GroupSelector {
    /// 获取群成员列表选择器。
    pub fn member_list(&self) -> GroupMemberListSelector {
        self.inner.member_list().into()
    }

    /// 获取群成员选择器。
    pub fn member(&self, uin: i64) -> GroupMemberSelector {
        self.inner.member(uin).into()
    }
}

/// 多个群聊选择器。
#[pyclass]
#[derive(Clone)]
pub struct MultiGroupSelector {
    inner: libawr::client::group::MultiGroupSelector,
}

impl From<libawr::client::group::MultiGroupSelector> for MultiGroupSelector {
    fn from(inner: libawr::client::group::MultiGroupSelector) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl MultiGroupSelector {
    /// 群号列表。
    pub fn codes(&self) -> Vec<i64> {
        self.inner.codes().clone()
    }
}

impl_py_properties!(MultiGroupSelector {});
impl_multi_selector!(MultiGroupSelector, Group);

/// 所有群聊选择器。
#[pyclass]
#[derive(Clone)]
pub struct AllGroupSelector {
    inner: libawr::client::group::AllGroupSelector,
}

impl From<libawr::client::group::AllGroupSelector> for AllGroupSelector {
    fn from(inner: libawr::client::group::AllGroupSelector) -> Self {
        Self { inner }
    }
}

impl_py_properties!(AllGroupSelector {});
impl_multi_selector!(AllGroupSelector, Group);
