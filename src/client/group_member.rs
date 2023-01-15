//! 群成员。

use crate::utils::PyPropertyConvert;
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
pub struct GroupMember {
    inner: Arc<libawr::client::group_member::GroupMember>,
}

impl From<Arc<libawr::client::group_member::GroupMember>> for GroupMember {
    fn from(inner: Arc<libawr::client::group_member::GroupMember>) -> Self {
        Self { inner }
    }
}

impl_py_properties!(GroupMember {
     group_code: i64 => i64,
     uin: i64 => i64,
     gender: u8 => u8,
     nickname: String => &str,
     card_name: String => &str,
     level: u16 => u16,
     join_time: i64 => i64,
     last_speak_time: i64 => i64,
     special_title: String => &str,
     special_title_expire_time: i64 => i64,
     shut_up_timestamp: i64 => i64,
     permission: ricq::structs::GroupMemberPermission => GroupMemberPermission,
});
impl_remote_target!(GroupMember, GroupMemberSelector);

#[pyclass]
#[derive(Clone)]
pub struct GroupMemberSelector {
    inner: libawr::client::group_member::GroupMemberSelector,
}

impl From<libawr::client::group_member::GroupMemberSelector> for GroupMemberSelector {
    fn from(inner: libawr::client::group_member::GroupMemberSelector) -> Self {
        Self { inner }
    }
}

impl_py_properties!(GroupMemberSelector {
     group_code: i64 => i64,
     uin: i64 => i64,
});
impl_option_selector!(GroupMemberSelector, GroupMember);

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub enum GroupMemberPermission {
    #[pyo3(name = "OWNER")]
    Owner,
    #[pyo3(name = "ADMINISTRATOR")]
    Administrator,
    #[pyo3(name = "MEMBER")]
    Member,
}

impl PyPropertyConvert<ricq::structs::GroupMemberPermission, GroupMemberPermission> {
    fn convert(t: &ricq::structs::GroupMemberPermission) -> GroupMemberPermission {
        use ricq::structs::GroupMemberPermission as GMP;
        match t {
            GMP::Owner => GroupMemberPermission::Owner,
            GMP::Administrator => GroupMemberPermission::Administrator,
            GMP::Member => GroupMemberPermission::Member,
        }
    }
}
