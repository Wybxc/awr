//! 好友。

use std::sync::Arc;

use pyo3::prelude::*;

use crate::{client::friend_group::FriendGroupSelector, utils::*};

/// 好友。
#[pyclass]
#[derive(Clone)]
pub struct Friend {
    pub(crate) inner: Arc<libawr::client::friend::Friend>,
}

impl From<Arc<libawr::client::friend::Friend>> for Friend {
    fn from(inner: Arc<libawr::client::friend::Friend>) -> Self {
        Self { inner }
    }
}

impl_py_properties!(Friend {
    uin: i64 => i64,
    nickname: String => &str,
    remark: String => &str,
    face_id: i16 => i16,
    group_id: u8 => u8,
});
impl_remote_target!(Friend, FriendSelector);

#[pymethods]
impl Friend {
    /// 获取所在的好友分组选择器。
    pub fn friend_group(&self) -> FriendGroupSelector {
        self.inner.friend_group().into()
    }
}

/// 好友选择器。
#[pyclass]
#[derive(Clone)]
pub struct FriendSelector {
    pub(crate) inner: libawr::client::friend::FriendSelector,
}

impl From<libawr::client::friend::FriendSelector> for FriendSelector {
    fn from(inner: libawr::client::friend::FriendSelector) -> Self {
        Self { inner }
    }
}

impl_py_properties!(FriendSelector {
    uin: i64 => i64,
});
impl_option_selector!(FriendSelector, Friend);

#[pymethods]
impl FriendSelector {
    pub fn poke<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let selector = self.inner.clone();
        py_future(py, async move {
            selector.poke().await?;
            Ok(())
        })
    }

    // #[args(segments = "*")]
    // pub fn send<'py>(&self, py: Python<'py>, segments: &'py PyTuple) -> PyResult<&'py PyAny> {
    //     todo!()
    // }

    // /// 撤回消息。
    // ///
    // /// # Python
    // /// ```python
    // /// async def recall(self, receipt: MessageReceipt) -> None: ...
    // /// ```
    // pub fn recall<'py>(
    //     &self,
    //     py: Python<'py>,
    //     receipt: PyRef<'py, MessageReceipt>,
    // ) -> PyResult<&'py PyAny> {
    //     let client = self.client.inner().clone();
    //     let uin = self.uin;
    //     let msg_time = receipt.msg_time();
    //     let seqs = receipt.seqs();
    //     let rands = receipt.rands();
    //     py_future(py, async move {
    //         client
    //             .recall_friend_message(uin, msg_time, seqs, rands)
    //             .await?;
    //         Ok(py_none())
    //     })
    // }
}
