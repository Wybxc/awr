use std::{path::PathBuf, sync::Arc};

use pyo3::prelude::*;

use tokio::task::JoinHandle;

mod friend_list;
mod structs;

pub use friend_list::FriendList;
pub use structs::*;

use crate::{
    login::reconnect,
    py_intern,
    utils::{py_future, py_none, py_obj},
};

/// QQ 客户端。
#[pyclass]
pub struct Client {
    client: Arc<ricq::Client>,
    alive: Option<JoinHandle<()>>,
    uin: i64,
    data_folder: PathBuf,
}

impl Client {
    pub async fn new(
        client: Arc<ricq::Client>,
        alive: JoinHandle<()>,
        data_folder: PathBuf,
    ) -> Self {
        let uin = client.uin().await;
        Self {
            client,
            alive: Some(alive),
            uin,
            data_folder,
        }
    }
}

#[pymethods]
impl Client {
    /// 等待并保持客户端连接，期间会自动进行断线重连。
    ///
    /// 多次调用此方法时，后续的调用将直接返回。
    pub fn alive<'py>(&mut self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let client = self.client.clone();
        let data_folder = self.data_folder.clone();
        let alive = self.alive.take();
        py_future(py, async move {
            if let Some(mut alive) = alive {
                loop {
                    alive.await?;

                    // 断线重连
                    if let Some(handle) = reconnect(&client, &data_folder).await? {
                        alive = handle;
                    } else {
                        break;
                    }
                }
            }
            tracing::info!("客户端 {} 连接断开", client.uin().await);
            Ok(py_none())
        })
    }

    /// 获取客户端 QQ 号。
    pub fn uin(&self) -> i64 {
        self.uin
    }

    /// 是否在线。
    pub fn online(&self) -> bool {
        self.client
            .online
            .load(std::sync::atomic::Ordering::Acquire)
    }

    /// 获取账号信息。
    pub fn account_info<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let client = self.client.clone();
        py_future(py, async move {
            let info = client.account_info.read().await;
            let info = AccountInfo {
                nickname: py_intern!(&info.nickname),
                age: info.age,
                gender: info.gender,
            };
            Ok(py_obj(info)?)
        })
    }

    /// 获取好友列表。
    pub fn get_friend_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let client = self.client.clone();
        py_future(py, async move {
            let friend_list = client.get_friend_list().await?;
            let friends = friend_list
                .friends
                .into_iter()
                .map(|info| info.into())
                .collect();
            let friend_groups = friend_list
                .friend_groups
                .into_iter()
                .map(|(key, info)| (key, info.into()))
                .collect();
            let total_count = friend_list.total_count;
            let online_count = friend_list.online_friend_count;
            let friend_list = FriendList {
                friends,
                friend_groups,
                total_count,
                online_count,
            };
            Ok(py_obj(friend_list)?)
        })
    }
}
