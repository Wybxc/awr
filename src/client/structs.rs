use pyo3::{prelude::*, types::*};

use crate::py_str;

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
///
/// # Python
/// ```python
/// class FriendInfo:
///     @property
///     def uin(self) -> int: ...
///     @property
///     def nickname(self) -> str: ...
///     @property
///     def remark(self) -> str: ...
///     @property
///     def face_id(self) -> int: ...
///     @property
///     def group_id(self) -> int: ...
/// ```
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

    /// TODO: 未知。
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
///
/// # Python
/// ```python
/// class FriendGroupInfo:
///     @property
///     def id(self) -> int: ...
///     @property
///     def name(self) -> str: ...
///     @property
///     def friend_count(self) -> int: ...
///     @property
///     def online_count(self) -> int: ...
///     @property
///     def seq_id(self) -> int: ...
/// ```
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

    /// TODO: 未知。
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

/// 群信息。
///
/// # Python
/// ```python
/// class GroupInfo:
///     @property
///     def uin(self) -> int: ...
///     @property
///     def code(self) -> int: ...
///     @property
///     def name(self) -> str: ...
///     @property
///     def memo(self) -> str: ...
///     @property
///     def owner_uin(self) -> int: ...
///     @property
///     def group_create_time(self) -> int: ...
///     @property
///     def group_level(self) -> int: ...
///     @property
///     def member_count(self) -> int: ...
///     @property
///     def max_member_count(self) -> int: ...
///     @property
///     def mute_all(self) -> bool: ...
///     @property
///     def my_shut_up_timestamp(self) -> int: ...
///     @property
///     def last_msg_seq(self) -> int: ...
/// ```
#[pyclass]
pub struct GroupInfo {
    /// uin。
    ///
    /// 含义可参考：[#181](https://github.com/Mrs4s/MiraiGo/issues/181)。
    #[pyo3(get)]
    pub uin: i64,

    /// 群号。
    #[pyo3(get)]
    pub code: i64,

    /// 群名称。
    #[pyo3(get)]
    pub name: Py<PyString>,

    /// 入群公告。
    #[pyo3(get)]
    pub memo: Py<PyString>,

    /// 群主 QQ 号。
    #[pyo3(get)]
    pub owner_uin: i64,

    /// 群创建时间。
    #[pyo3(get)]
    pub group_create_time: u32,

    /// 群等级。
    #[pyo3(get)]
    pub group_level: u32,

    /// 群成员数量。
    #[pyo3(get)]
    pub member_count: u16,

    /// 最大群成员数量。
    #[pyo3(get)]
    pub max_member_count: u16,

    /// 是否开启全员禁言。
    #[pyo3(get)]
    pub mute_all: bool,

    /// 被禁言剩余时间，单位秒。
    #[pyo3(get)]
    pub my_shut_up_timestamp: i64,

    /// TODO: 未知。
    #[pyo3(get)]
    pub last_msg_seq: i64,
}

impl From<ricq::structs::GroupInfo> for GroupInfo {
    fn from(info: ricq::structs::GroupInfo) -> Self {
        GroupInfo {
            uin: info.uin,
            code: info.code,
            name: py_str!(&info.name),
            memo: py_str!(&info.memo),
            owner_uin: info.owner_uin,
            group_create_time: info.group_create_time,
            group_level: info.group_level,
            member_count: info.member_count,
            max_member_count: info.max_member_count,
            mute_all: info.shut_up_timestamp != 0,
            my_shut_up_timestamp: info.my_shut_up_timestamp,
            last_msg_seq: info.last_msg_seq,
        }
    }
}

#[pymethods]
impl GroupInfo {
    fn __repr__(&self) -> String {
        Python::with_gil(|py| {
            format!(
                "GroupInfo(uin={}, code={}, name={}, memo={}, owner_uin={}, group_create_time={}, \
                    group_level={}, member_count={}, max_member_count={}, mute_all={}, \
                    my_shut_up_timestamp={}, last_msg_seq={})",
                self.uin,
                self.code,
                self.name.as_ref(py).repr().unwrap(),
                self.memo.as_ref(py).repr().unwrap(),
                self.owner_uin,
                self.group_create_time,
                self.group_level,
                self.member_count,
                self.max_member_count,
                self.mute_all,
                self.my_shut_up_timestamp,
                self.last_msg_seq
            )
        })
    }
}
