//! 群成员列表。

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};

use crate::client::group_member::GroupMember;
use crate::utils::py_obj;

#[pyclass]
#[derive(Clone)]
struct GroupMemberList {
    inner: Arc<libawr::client::group_member_list::GroupMemberList>,
}

impl From<Arc<libawr::client::group_member_list::GroupMemberList>> for GroupMemberList {
    fn from(inner: Arc<libawr::client::group_member_list::GroupMemberList>) -> Self {
        Self { inner }
    }
}

impl_py_properties!(GroupMemberList {
    total_count: i16 => i16,
});
impl_remote_target!(GroupMemberList, GroupMemberListSelector);

#[pymethods]
impl GroupMemberList {
    /// 获取所有群成员信息。
    pub fn members<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let members: Vec<_> = self
            .inner
            .members()
            .iter()
            .map(|(uin, info)| Ok((*uin, py_obj(GroupMember::from(info.clone()))?)))
            .collect::<PyResult<_>>()?;
        Ok(members.into_py_dict(py))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct GroupMemberListSelector {
    inner: libawr::client::group_member_list::GroupMemberListSelector,
}

impl From<libawr::client::group_member_list::GroupMemberListSelector> for GroupMemberListSelector {
    fn from(inner: libawr::client::group_member_list::GroupMemberListSelector) -> Self {
        Self { inner }
    }
}

impl_py_properties!(GroupMemberListSelector {
    group_code: i64 => i64,
});
impl_option_selector!(GroupMemberListSelector, GroupMemberList);
