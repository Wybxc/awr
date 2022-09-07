//! 好友。
//!
//! 更多信息参考 [`Friend`]。

use std::sync::Arc;

use pyo3::prelude::*;
use ricq::structs::FriendInfo;

use crate::utils::{py_future, py_none, py_obj};

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
    pub(super) client: Arc<ClientImpl>,
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

    /// 获取好友选择器。
    ///
    /// # Python
    /// ```python
    /// def as_selector(self) -> FriendSelector: ...
    /// ```
    pub fn as_selector(&self) -> FriendSelector {
        FriendSelector {
            client: self.client.clone(),
            uin: self.uin(),
        }
    }

    /// 好友戳一戳。
    ///
    /// # Python
    /// ```python
    /// async def poke(self) -> None: ...
    /// ```
    pub fn poke<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        self.as_selector().poke(py)
    }
}

/// 好友选择器。
///
/// # Examples
/// ```python
/// await client.friend(12345678).poke()
/// ```
///
/// # Python
/// ```python
/// class FriendSelector:
///     @property
///     def uin(self) -> int: ...
/// ```
#[pyclass]
pub struct FriendSelector {
    pub(super) client: Arc<ClientImpl>,
    /// 好友 QQ 号。
    #[pyo3(get)]
    pub(super) uin: i64,
}

#[pymethods]
impl FriendSelector {
    /// 得到好友对象。
    ///
    /// 参考 [`Friend`]。
    ///
    /// # Python
    /// ```python
    /// async def hydrate(self) -> Friend | None: ...
    /// ```
    pub fn hydrate<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let client = self.client.clone();
        let uin = self.uin;
        py_future(py, async move {
            let friend_list = client.get_friend_list_cached().await?;
            match friend_list.find_friend(uin) {
                Some(friend) => Ok(Some(py_obj(friend)?)),
                None => Ok(None),
            }
        })
    }

    /// 好友戳一戳。
    ///
    /// # Python
    /// ```python
    /// async def poke(self) -> None: ...
    /// ```
    pub fn poke<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let client = self.client.inner().clone();
        let uin = self.uin;
        py_future(py, async move {
            client.friend_poke(uin).await?;
            Ok(py_none())
        })
    }
}
