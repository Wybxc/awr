use std::collections::HashMap;

use pyo3::{prelude::*, types::*};

use super::{FriendGroupInfo, FriendInfo};

/// 好友列表。
///
/// # Python
/// ```python
/// class FriendList:
///     @property
///     def total_count(self) -> int: ...
///     @property
///     def online_count(self) -> int: ...
/// ```
#[pyclass]
pub struct FriendList {
    /// 好友信息。
    pub(crate) friends: Vec<FriendInfo>,
    /// 好友分组信息。
    pub(crate) friend_groups: HashMap<u8, FriendGroupInfo>,
    /// 好友数量。
    #[pyo3(get)]
    pub total_count: i16,
    /// 在线好友数量。
    #[pyo3(get)]
    pub online_count: i16,
}

#[pymethods]
impl FriendList {
    /// 遍历好友信息的迭代器。
    ///
    /// 参考 [`FriendInfo`]。
    ///
    /// # Examples
    /// ```python
    /// friend_list = await client.get_friend_list()
    /// for friend in friend_list.friends():
    ///     print(friend.nickname)
    /// ```
    ///
    /// # Python
    /// ```python
    /// def friends(self) -> Iterator[FriendInfo]:
    /// ```
    pub fn friends(self_: Py<Self>, py: Python) -> FriendsIter {
        FriendsIter {
            list: self_.clone_ref(py),
            curr: 0,
            end: self_.borrow(py).friends.len(),
        }
    }

    /// 查找指定的好友。
    ///
    /// 参考 [`FriendInfo`]。
    ///
    /// # Examples
    /// ```python
    /// friend_list = await client.get_friend_list()
    /// friend = friend_list.find_friend(12345678)
    /// if friend:
    ///     print(friend.nickname)
    /// else:
    ///     print("未找到好友 12345678")
    /// ```
    ///
    /// # Python
    /// ```python
    /// def find_friend(self, uin: int) -> FriendInfo | None:
    /// ```
    pub fn find_friend(&self, uin: i64) -> Option<FriendInfo> {
        self.friends.iter().find(|f| f.uin == uin).cloned()
    }

    /// 获取所有好友分组信息。
    ///
    /// 参考 [`FriendGroupInfo`]。
    ///
    /// # Examples
    /// ```python
    /// friend_list = await client.get_friend_list()
    /// for group in friend_list.friend_groups():
    ///     print(group.name)
    /// ```
    ///
    /// # Python
    /// ```python
    /// def friend_groups(self) -> list[FriendGroupInfo]:
    /// ```
    pub fn friend_groups<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        let friend_groups = self
            .friend_groups
            .values()
            .map(|info| PyCell::new(py, info.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PyList::new(py, friend_groups))
    }

    /// 查找好友分组。
    ///
    /// 参考 [`FriendGroupInfo`]。
    ///
    /// # Examples
    /// ```python
    /// friend_list = await client.get_friend_list()
    /// friend = friend_list.find_friend(12345678)
    /// if friend:
    ///     group = friend_list.find_friend_group(friend.group_id)
    ///     if group:
    ///         print("好友 12345678 位于分组", group.name)
    /// ```
    ///  
    /// # Python
    /// ```python
    /// def find_friend_group(self, group_id: int) -> FriendGroupInfo | None:
    /// ```
    pub fn find_friend_group(&self, group_id: u8) -> Option<FriendGroupInfo> {
        self.friend_groups.get(&group_id).cloned()
    }
}

#[pyclass]
#[doc(hidden)]
pub struct FriendsIter {
    list: Py<FriendList>,
    curr: usize,
    end: usize,
}

#[pymethods]
impl FriendsIter {
    fn __iter__(self_: PyRef<'_, Self>) -> PyRef<'_, Self> {
        self_
    }

    fn __next__(mut self_: PyRefMut<'_, Self>, py: Python) -> Option<FriendInfo> {
        if self_.curr < self_.end {
            let info = self_.list.borrow(py).friends[self_.curr].clone();
            self_.curr += 1;
            Some(info)
        } else {
            None
        }
    }
}
