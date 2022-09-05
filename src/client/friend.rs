//! 好友。
//!
//! 更多信息参考 [`Friend`]。

use pyo3::prelude::*;
use ricq::structs::FriendInfo;

use super::ClientImpl;

/// 好友。
///
/// # Python
/// ```python
/// class Friend: ...
/// ```
#[pyclass]
pub struct Friend {
    #[allow(unused)] // TODO: remove this
    pub(super) client: ClientImpl,
    pub(super) info: FriendInfo,
}

#[pymethods]
impl Friend {
    /// 好友 QQ 号。
    ///
    /// # Python
    /// ```python
    /// @property
    /// def uin(self) -> int: ...
    /// ```
    #[getter]
    pub fn uin(&self) -> i64 {
        self.info.uin
    }

    /// 好友昵称。
    ///
    /// # Python
    /// ```python
    /// @property
    /// def nickname(self) -> str: ...
    /// ```
    #[getter]
    pub fn nickname(&self) -> &str {
        &self.info.nick
    }

    /// 好友备注。
    ///
    /// # Python
    /// ```python
    /// @property
    /// def remark(self) -> str: ...
    /// ```
    #[getter]
    pub fn remark(&self) -> &str {
        &self.info.remark
    }

    /// TODO: 未知。
    ///
    /// # Python
    /// ```python
    /// @property
    /// def face_id(self) -> int: ...
    /// ```
    #[getter]
    pub fn face_id(&self) -> i16 {
        self.info.face_id
    }

    /// 好友分组编号。
    ///
    /// # Python
    /// ```python
    /// @property
    /// def group_id(self) -> int: ...
    /// ```
    #[getter]
    pub fn group_id(&self) -> u8 {
        self.info.group_id
    }

    fn __repr__(&self) -> String {
        format!(
            "Friend(uin={:?}, nickname={:?}, remark={:?}, face_id={:?}, group_id={:?})",
            self.uin(),
            self.nickname(),
            self.remark(),
            self.face_id(),
            self.group_id(),
        )
    }
}
