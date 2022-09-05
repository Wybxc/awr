//! 好友列表。
//!
//! 更多信息参考 [`FriendList`]。

use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use pyo3::{prelude::*, types::*};
use ricq::structs::{FriendGroupInfo, FriendInfo};

use super::{friend::Friend, friend_group::FriendGroup, Cacheable, ClientImpl};

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
#[derive(Clone)]
pub struct FriendList {
    pub(crate) client: ClientImpl,
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
    /// 参考 [`Friend`]。
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
    /// def friends(self) -> Iterator[Friend]:
    /// ```
    pub fn friends(self_: Py<Self>, py: Python) -> FriendIter {
        FriendIter {
            list: self_.clone_ref(py),
            curr: 0,
            end: self_.borrow(py).friends.len(),
        }
    }

    /// 查找指定的好友。
    ///
    /// 参考 [`Friend`]。
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
    /// def find_friend(self, uin: int) -> Friend | None:
    /// ```
    pub fn find_friend(&self, uin: i64) -> Option<Friend> {
        self.friends
            .iter()
            .find(|info| info.uin == uin)
            .map(|info| Friend {
                client: self.client.clone(),
                info: info.clone(),
            })
    }

    /// 获取所有好友分组信息。
    ///
    /// 参考 [`FriendGroup`]。
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
    /// def friend_groups(self) -> list[FriendGroup]:
    /// ```
    pub fn friend_groups<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        let friend_groups = self
            .friend_groups
            .values()
            .map(|info| {
                PyCell::new(
                    py,
                    FriendGroup {
                        client: self.client.clone(),
                        info: info.clone(),
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PyList::new(py, friend_groups))
    }

    /// 查找好友分组。
    ///
    /// 参考 [`FriendGroup`]。
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
    /// def find_friend_group(self, group_id: int) -> FriendGroup | None:
    /// ```
    pub fn find_friend_group(&self, group_id: u8) -> Option<FriendGroup> {
        self.friend_groups
            .get(&group_id)
            .cloned()
            .map(|info| FriendGroup {
                client: self.client.clone(),
                info,
            })
    }
}

#[async_trait]
impl Cacheable for FriendList {
    /// 请求获取好友列表。
    async fn fetch(client: ClientImpl) -> Result<Self> {
        let friend_list = client.get_friend_list().await?;
        let friend_list = FriendList {
            client,
            friends: friend_list.friends,
            friend_groups: friend_list.friend_groups,
            total_count: friend_list.total_count,
            online_count: friend_list.online_friend_count,
        };
        Ok(friend_list)
    }
}

#[pyclass]
#[doc(hidden)]
pub struct FriendIter {
    list: Py<FriendList>,
    curr: usize,
    end: usize,
}

#[pymethods]
impl FriendIter {
    fn __iter__(self_: PyRef<'_, Self>) -> PyRef<'_, Self> {
        self_
    }

    fn __next__(&mut self, py: Python) -> Option<Friend> {
        if self.curr < self.end {
            let info = self.list.borrow(py).friends[self.curr].clone();
            self.curr += 1;
            Some(Friend {
                client: self.list.borrow(py).client.clone(),
                info,
            })
        } else {
            None
        }
    }
}
