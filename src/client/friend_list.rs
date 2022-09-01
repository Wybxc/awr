use std::collections::HashMap;

use pyo3::{prelude::*, types::*};

use super::{FriendGroupInfo, FriendInfo};

/// 好友列表。
#[pyclass]
pub struct FriendList {
    /// 好友信息。
    pub friends: Vec<FriendInfo>,
    /// 好友分组信息。
    pub friend_groups: HashMap<u8, FriendGroupInfo>,
    /// 好友数量。
    pub total_count: i16,
    /// 在线好友数量。
    pub online_count: i16,
}

#[pymethods]
impl FriendList {
    /// 遍历好友信息的迭代器。
    fn friends(self_: Py<Self>, py: Python) -> FriendsIter {
        FriendsIter {
            list: self_.clone_ref(py),
            curr: 0,
            end: self_.borrow(py).friends.len(),
        }
    }

    /// 查找指定的好友。
    fn find_friend(&self, uin: i64) -> Option<FriendInfo> {
        self.friends.iter().find(|f| f.uin == uin).cloned()
    }

    /// 获取所有好友分组信息。
    fn friend_groups<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        let friend_groups = self
            .friend_groups
            .values()
            .map(|info| PyCell::new(py, info.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PyList::new(py, friend_groups))
    }

    /// 查找好友分组。
    fn find_friend_group(&self, group_id: u8) -> Option<FriendGroupInfo> {
        self.friend_groups.get(&group_id).cloned()
    }
}

#[pyclass]
struct FriendsIter {
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
