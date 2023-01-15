//! QQ 无头客户端。
//!
//! 更多信息参考 [`Client`]。

use std::sync::Arc;

use pyo3::{prelude::*, types::*};

use crate::{
    client::{
        friend::FriendSelector,
        friend_group::FriendGroupSelector,
        friend_list::FriendListSelector,
        group::{AllGroupSelector, GroupSelector, MultiGroupSelector},
        group_member_list::GroupMemberListSelector,
    },
    utils::*,
};

pub mod account_info;
pub mod friend;
pub mod friend_group;
pub mod friend_list;
pub mod group;
pub mod group_member;
mod group_member_list;

/// 客户端。
#[pyclass]
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<libawr::client::Client>,
}

impl From<Arc<libawr::client::Client>> for Client {
    fn from(inner: Arc<libawr::client::Client>) -> Self {
        Self { inner }
    }
}

impl_py_properties!(Client {
    uin: i64 => i64,
});

#[pymethods]
impl Client {
    /// 设置好友列表缓存过期时间。
    pub fn set_friend_list_cache_time<'py>(
        &self,
        py: Python<'py>,
        time: &PyAny,
    ) -> PyResult<&'py PyAny> {
        let inner = self.inner.clone();
        let time = from_timedelta(time)?;
        py_future(py, async move {
            inner.set_friend_list_cache_time(time).await;
            Ok(())
        })
    }

    /// 设置群信息缓存过期时间。
    pub fn set_group_cache_time<'py>(&self, py: Python<'py>, time: &PyAny) -> PyResult<&'py PyAny> {
        let inner = self.inner.clone();
        let time = from_timedelta(time)?;
        py_future(py, async move {
            inner.set_group_cache_time(time).await;
            Ok(())
        })
    }

    /// 设置群成员列表缓存过期时间。
    pub fn set_group_member_list_cache_time<'py>(
        &self,
        py: Python<'py>,
        time: &PyAny,
    ) -> PyResult<&'py PyAny> {
        let inner = self.inner.clone();
        let time = from_timedelta(time)?;
        py_future(py, async move {
            inner.set_group_member_list_cache_time(time).await;
            Ok(())
        })
    }

    /// 当前账号是否在线。    
    pub fn is_online(&self) -> bool {
        self.inner.is_online()
    }

    /// 构造好友列表选择器。
    pub fn friend_list(&self) -> FriendListSelector {
        self.inner.friend_list().into()
    }

    /// 获取好友列表对象。
    pub fn get_friend_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        self.friend_list().fetch(py)
    }

    /// 刷新好友列表缓存。
    pub fn flush_friend_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let inner = self.inner.clone();
        py_future(py, async move {
            inner.flush_friend_list().await?;
            Ok(())
        })
    }

    /// 构造好友选择器。
    pub fn friend(&self, uin: i64) -> FriendSelector {
        self.inner.friend(uin).into()
    }

    /// 获取好友对象。
    pub fn get_friend<'py>(&self, py: Python<'py>, uin: i64) -> PyResult<&'py PyAny> {
        self.friend(uin).fetch(py)
    }

    /// 构建好友分组选择器。
    pub fn friend_group(&self, id: u8) -> FriendGroupSelector {
        self.inner.friend_group(id).into()
    }

    /// 获取好友分组对象。
    pub fn get_friend_group<'py>(&self, py: Python<'py>, id: u8) -> PyResult<&'py PyAny> {
        self.friend_group(id).fetch(py)
    }

    /// 创建好友分组。
    pub fn create_friend_group<'py>(&self, py: Python<'py>, name: String) -> PyResult<&'py PyAny> {
        let inner = self.inner.clone();
        py_future(py, async move {
            inner.create_friend_group(name).await?;
            Ok(())
        })
    }

    /// 构造群选择器。
    pub fn group(&self, code: i64) -> GroupSelector {
        self.inner.group(code).into()
    }

    /// 获取群对象。
    pub fn get_group<'py>(&self, py: Python<'py>, code: i64) -> PyResult<&'py PyAny> {
        self.group(code).fetch(py)
    }

    /// 构造多个群选择器。
    #[args(codes = "*")]
    pub fn groups(&self, codes: &PyTuple) -> PyResult<MultiGroupSelector> {
        let codes: Vec<i64> = codes
            .iter()
            .map(|code| code.extract::<i64>())
            .collect::<PyResult<_>>()?;
        Ok(self.inner.groups(codes).into())
    }

    /// 获取多个群对象。
    #[args(codes = "*")]
    pub fn get_groups<'py>(&self, py: Python<'py>, codes: &PyTuple) -> PyResult<&'py PyAny> {
        self.groups(codes)?.fetch(py) // 麻，懒得再写一遍 PyDict 的转换了
    }

    /// 构造所有群选择器。
    pub fn all_groups(&self) -> AllGroupSelector {
        self.inner.all_groups().into()
    }

    /// 获取所有群对象。
    pub fn get_all_groups<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        self.all_groups().fetch(py)
    }

    /// 构造群成员选择器。
    pub fn group_member(&self, code: i64, uin: i64) -> group_member::GroupMemberSelector {
        self.inner.group_member(code, uin).into()
    }

    /// 获取群成员对象。
    pub fn get_group_member<'py>(
        &self,
        py: Python<'py>,
        code: i64,
        uin: i64,
    ) -> PyResult<&'py PyAny> {
        self.group_member(code, uin).fetch(py)
    }

    /// 构造群成员列表选择器。
    pub fn group_member_list(&self, code: i64) -> GroupMemberListSelector {
        self.inner.group_member_list(code).into()
    }

    /// 获取群成员列表对象。
    pub fn get_group_member_list<'py>(&self, py: Python<'py>, code: i64) -> PyResult<&'py PyAny> {
        self.group_member_list(code).fetch(py)
    }
}
