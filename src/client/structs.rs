use pyo3::{prelude::*, types::*};

use crate::py_str;

/// 账号信息。
#[pyclass]
#[derive(Debug, Clone)]
pub struct AccountInfo {
    /// 昵称。
    #[pyo3(get)]
    pub nickname: Py<PyString>,

    /// 年龄。
    #[pyo3(get)]
    pub age: u8,

    /// 性别。
    #[pyo3(get)]
    pub gender: u8,
}

#[pymethods]
impl AccountInfo {
    fn __repr__(&self) -> String {
        Python::with_gil(|py| {
            format!(
                "AccountInfo(nickname={}, age={}, gender={})",
                self.nickname.as_ref(py).repr().unwrap(),
                self.age,
                self.gender
            )
        })
    }
}

/// 好友信息。
#[pyclass]
#[derive(Debug, Clone)]
pub struct FriendInfo {
    /// 好友 QQ 号。
    #[pyo3(get)]
    pub uin: i64,

    /// 好友昵称。
    #[pyo3(get)]
    pub nickname: Py<PyString>,

    /// 好友备注。
    #[pyo3(get)]
    pub remark: Py<PyString>,

    #[pyo3(get)]
    pub face_id: i16,

    /// 好友分组编号。
    #[pyo3(get)]
    pub group_id: u8,
}

impl From<ricq::structs::FriendInfo> for FriendInfo {
    fn from(info: ricq::structs::FriendInfo) -> Self {
        FriendInfo {
            uin: info.uin,
            nickname: py_str!(&info.nick),
            remark: py_str!(&info.remark),
            face_id: info.face_id,
            group_id: info.group_id,
        }
    }
}

#[pymethods]
impl FriendInfo {
    fn __repr__(&self) -> String {
        Python::with_gil(|py| {
            format!(
                "FriendInfo(uin={}, nickname={}, remark={}, face_id={}, group_id={})",
                self.uin,
                self.nickname.as_ref(py).repr().unwrap(),
                self.remark.as_ref(py).repr().unwrap(),
                self.face_id,
                self.group_id
            )
        })
    }
}

/// 好友分组信息。
#[pyclass]
#[derive(Debug, Clone)]
pub struct FriendGroupInfo {
    /// 好友分组编号。
    #[pyo3(get)]
    pub id: u8,

    /// 好友分组名称。
    #[pyo3(get)]
    pub name: Py<PyString>,

    /// 分组中的好友数量。
    #[pyo3(get)]
    pub friend_count: i32,

    /// 分组中在线的好友数量。
    #[pyo3(get)]
    pub online_count: i32,

    #[pyo3(get)]
    pub seq_id: u8,
}

impl From<ricq::structs::FriendGroupInfo> for FriendGroupInfo {
    fn from(info: ricq::structs::FriendGroupInfo) -> Self {
        FriendGroupInfo {
            id: info.group_id,
            name: py_str!(&info.group_name),
            friend_count: info.friend_count,
            online_count: info.online_friend_count,
            seq_id: info.seq_id,
        }
    }
}

#[pymethods]
impl FriendGroupInfo {
    fn __repr__(&self) -> String {
        Python::with_gil(|py| {
            format!(
                "FriendGroupInfo(group_id={}, group_name={}, friend_count={}, online_friend_count={}, seq_id={})",
                self.id,
                self.name.as_ref(py).repr().unwrap(),
                self.friend_count,
                self.online_count,
                self.seq_id
            )
        })
    }
}
