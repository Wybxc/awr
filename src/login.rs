//! 账号登录。

use std::{error::Error, path::PathBuf, sync::Arc};

use pyo3::{
    exceptions::{PyRuntimeError, PyTypeError},
    prelude::*,
};
use tokio::sync::Mutex;

use crate::{client::Client, utils::py_future};

/// 协议。
#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    /// IPad 协议。
    #[pyo3(name = "IPAD")]
    IPad,
    /// 安卓手机协议。
    #[pyo3(name = "ANDROID_PHONE")]
    AndroidPhone,
    /// 安卓手表协议。
    #[pyo3(name = "ANDROID_WATCH")]
    AndroidWatch,
    /// Mac OS 协议。
    #[pyo3(name = "MAC_OS")]
    MacOS,
    /// 企点协议。
    #[pyo3(name = "QI_DIAN")]
    QiDian,
}

/// 登录保持。
#[pyclass]
pub struct AliveHandle {
    inner: Arc<Mutex<Option<libawr::login::AliveHandle>>>,
}

#[pymethods]
impl AliveHandle {
    /// 等待，直到连接断开。
    pub fn alive<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let inner = self.inner.clone();
        py_future(py, async move {
            inner
                .try_lock()
                .map_err(|_| PyRuntimeError::new_err("其他线程正在等待连接断开"))?
                .as_mut()
                .ok_or_else(|| PyRuntimeError::new_err("其他线程正在等待连接断开"))?
                .alive()
                .await?;
            Ok(())
        })
    }

    /// 断线重连。
    pub fn reconnect<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let inner = self.inner.clone();
        py_future(py, async move {
            inner
                .try_lock()
                .map_err(|_| PyRuntimeError::new_err("其他线程正在等待连接断开"))?
                .as_mut()
                .ok_or_else(|| PyRuntimeError::new_err("其他线程正在等待连接断开"))?
                .reconnect()
                .await?;
            Ok(())
        })
    }

    /// 开始自动断线重连。
    pub fn auto_reconnect<'py>(&mut self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let inner = self.inner.clone();
        py_future(py, async move {
            inner
                .try_lock()
                .map_err(|_| PyRuntimeError::new_err("其他线程正在等待连接断开"))?
                .take()
                .ok_or_else(|| PyRuntimeError::new_err("其他线程正在等待连接断开"))?
                .auto_reconnect()
                .await?;
            Ok(())
        })
    }
}

/// 密码登录。
#[pyfunction]
pub fn login_with_password<'py>(
    py: Python<'py>,
    uin: i64,
    password: String,
    protocol: &Protocol,
    data_folder: PathBuf,
) -> PyResult<&'py PyAny> {
    let protocol = match protocol {
        Protocol::IPad => libawr::login::Protocol::IPad,
        Protocol::AndroidPhone => libawr::login::Protocol::AndroidPhone,
        Protocol::AndroidWatch => libawr::login::Protocol::AndroidWatch,
        Protocol::MacOS => libawr::login::Protocol::MacOS,
        Protocol::QiDian => libawr::login::Protocol::QiDian,
    };

    py_future(py, async move {
        let (client, alive_handle) =
            libawr::login_with_password(uin, &password, protocol, data_folder).await?;

        let client = Client { inner: client };
        let alive_handle = AliveHandle {
            inner: Arc::new(Mutex::new(Some(alive_handle))),
        };
        Ok((client, alive_handle))
    })
}

/// 密码 MD5 登录。
#[pyfunction]
pub fn login_with_password_md5<'py>(
    py: Python<'py>,
    uin: i64,
    password_md5: Vec<u8>,
    protocol: &Protocol,
    data_folder: PathBuf,
) -> PyResult<&'py PyAny> {
    let protocol = match protocol {
        Protocol::IPad => libawr::login::Protocol::IPad,
        Protocol::AndroidPhone => libawr::login::Protocol::AndroidPhone,
        Protocol::AndroidWatch => libawr::login::Protocol::AndroidWatch,
        Protocol::MacOS => libawr::login::Protocol::MacOS,
        Protocol::QiDian => libawr::login::Protocol::QiDian,
    };

    py_future(py, async move {
        let (client, alive_handle) =
            libawr::login_with_password_md5(uin, &password_md5, protocol, data_folder).await?;

        let client = Client { inner: client };
        let alive_handle = AliveHandle {
            inner: Arc::new(Mutex::new(Some(alive_handle))),
        };
        Ok((client, alive_handle))
    })
}

/// 使用二维码登录。
#[pyfunction]
pub fn login_with_qrcode(
    py: Python<'_>,
    uin: i64,
    show_qrcode: PyObject,
    data_folder: PathBuf,
) -> PyResult<&'_ PyAny> {
    py_future(py, async move {
        let (client, alive_handle) = libawr::login_with_qrcode(
            uin,
            |qrcode| {
                Python::with_gil(|py| -> Result<(), Box<dyn Error + Send + Sync>> {
                    show_qrcode.as_ref(py).call1((Vec::from(qrcode),))?;
                    Ok(())
                })
            },
            data_folder,
        )
        .await?;

        let client = Client { inner: client };
        let alive_handle = AliveHandle {
            inner: Arc::new(Mutex::new(Some(alive_handle))),
        };
        Ok((client, alive_handle))
    })
}

/// 登录。
#[pyfunction(
    "*",
    password = "None",
    password_md5 = "None",
    show_qrcode = "None",
    protocol = "None",
    data_folder = "\"./bots\".into()"
)]
pub fn login<'py>(
    py: Python<'py>,
    uin: i64,
    password: Option<String>,
    password_md5: Option<Vec<u8>>,
    show_qrcode: Option<PyObject>,
    protocol: Option<&Protocol>,
    data_folder: PathBuf,
) -> PyResult<&'py PyAny> {
    if let Some(password) = password {
        login_with_password(
            py,
            uin,
            password,
            protocol.ok_or_else(|| PyTypeError::new_err("请指定协议"))?,
            data_folder,
        )
    } else if let Some(password_md5) = password_md5 {
        login_with_password_md5(
            py,
            uin,
            password_md5,
            protocol.ok_or_else(|| PyTypeError::new_err("请指定协议"))?,
            data_folder,
        )
    } else if let Some(show_qrcode) = show_qrcode {
        login_with_qrcode(py, uin, show_qrcode, data_folder)
    } else {
        Err(PyRuntimeError::new_err("请指定密码或二维码显示函数"))
    }
}
