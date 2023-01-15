use std::time::Duration;

use anyhow::Result;
use futures_util::Future;
use pyo3::{once_cell::GILOnceCell, prelude::*, types::*};

/// 将 Rust 定义的 Python 类实例化。
pub fn py_obj<T>(obj: impl Into<PyClassInitializer<T>>) -> PyResult<Py<T>>
where
    T: pyo3::PyClass,
{
    Python::with_gil(|py| Py::new(py, obj))
}

/// 导入 Python 类型。
///
/// # Panics
/// 若找不到类型，或者类型不是 `type` 实例，则 panic。
pub fn py_import_type<F>(py: Python, import: F) -> &PyType
where
    F: FnOnce(Python) -> Result<PyObject>,
{
    static TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();
    TYPE.get_or_init(py, || {
        let type_obj = import(py).unwrap();
        type_obj.cast_as::<PyType>(py).unwrap().into_py(py)
    })
    .as_ref(py)
}

/// 将 Python 的 timedelta 转换为 Rust 的 Duration。
pub fn from_timedelta(td: &PyAny) -> PyResult<Duration> {
    let is_timedelta = Python::with_gil(|py| {
        let timedelta = py_import_type(py, |py| {
            Ok(py.import("datetime")?.getattr("timedelta")?.into_py(py))
        });
        td.is_instance(timedelta)
    })?;
    if !is_timedelta {
        return Err(pyo3::exceptions::PyTypeError::new_err(format!(
            "expected datetime.timedelta, got {td}"
        )));
    }

    let seconds = td.getattr("total_seconds")?.call0()?;
    let seconds = seconds.extract::<f64>()?;
    match Duration::try_from_secs_f64(seconds) {
        Ok(duration) => Ok(duration),
        Err(_) => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "timedelta out of range: {seconds}s",
        ))),
    }
}

/// 构造一个 Python 的 dict。
#[doc(hidden)]
macro_rules! py_dict {
    ($py:expr, $($name:expr => $value:expr),*) => {
        [$(($name, $value),)*].into_py_dict($py)
    };
}

/// 将 [`tokio`] 的 Future 包装为 Python 的 Future。
pub fn py_future<F, T>(py: Python, future: F) -> PyResult<&PyAny>
where
    F: Future<Output = Result<T, anyhow::Error>> + Send + 'static,
    T: IntoPy<PyObject>,
{
    pyo3_asyncio::tokio::future_into_py(py, async move { Ok(future.await?) })
}

pub(crate) struct PyPropertyConvert<T, U>(std::marker::PhantomData<(T, U)>);

impl<T: Copy> PyPropertyConvert<T, T> {
    pub fn convert(t: &T) -> T {
        *t
    }
}

impl PyPropertyConvert<String, &str> {
    pub fn convert(t: &str) -> &str {
        t
    }
}

macro_rules! impl_py_properties {
    ($class: ident {$($name: ident : $from_ty: ty => $to_ty: ty ),* $(,)?}) => {
        #[pymethods]
        impl $class {
            $(
                #[getter]
                pub fn $name(&self) -> $to_ty {
                    $crate::utils::PyPropertyConvert::<$from_ty, $to_ty>::convert(&self.inner.$name)
                }
            )*

            fn __repr__(&self) -> String {
                let props: &[String] = &[
                    $(format!(concat!(stringify!($name), "={:?}"), self.$name()),)*
                ];
                format!(concat!(stringify!($class), "({})"), props.join(", "))
            }
        }
    };
}

macro_rules! impl_remote_target {
    ($class: ident, $selector: ident) => {
        #[pymethods]
        impl $class {
            pub fn __getattr__(&self, py: Python, name: &str) -> PyResult<PyObject> {
                use ::libawr::meta::selector::Selector;

                let selector: $selector = self.inner.as_selector().clone().into();
                selector.into_py(py).getattr(py, name)
            }
        }
    };
}

macro_rules! impl_single_selector {
    ($class: ident, $target: ident) => {
        #[pymethods]
        impl $class {
            pub fn as_selector(&self) -> Self {
                self.clone()
            }

            pub fn as_client(&self) -> $crate::client::Client {
                use ::libawr::meta::selector::Selector;

                self.inner.as_client().clone().into()
            }

            pub fn fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                use ::libawr::meta::selector::SingleSelector;
                use $crate::utils::py_future;

                let selector = self.inner.clone();
                py_future(
                    py,
                    async move { Ok($target::from(selector.fetch().await?)) },
                )
            }

            pub fn flush<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                use ::libawr::meta::selector::Selector;
                use $crate::utils::py_future;

                let selector = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    selector.flush().await;
                    Ok(self_)
                })
            }

            pub fn flush_and_fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                use ::libawr::meta::selector::SingleSelector;
                use $crate::utils::py_future;

                let selector = self.inner.clone();
                py_future(py, async move {
                    Ok($target::from(selector.flush_and_fetch().await?))
                })
            }
        }
    };
}

macro_rules! impl_option_selector {
    ($class: ident, $target: ident) => {
        #[pymethods]
        impl $class {
            pub fn as_selector(&self) -> Self {
                self.clone()
            }

            pub fn as_client(&self) -> $crate::client::Client {
                use ::libawr::meta::selector::Selector;

                self.inner.as_client().clone().into()
            }

            pub fn fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                use ::libawr::meta::selector::OptionSelector;
                use $crate::utils::py_future;

                let selector = self.inner.clone();
                py_future(
                    py,
                    async move { Ok(selector.fetch().await?.map($target::from)) },
                )
            }

            pub fn flush<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                use ::libawr::meta::selector::Selector;
                use $crate::utils::py_future;

                let selector = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    selector.flush().await;
                    Ok(self_)
                })
            }

            pub fn flush_and_fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                use ::libawr::meta::selector::OptionSelector;
                use $crate::utils::py_future;

                let selector = self.inner.clone();
                py_future(py, async move {
                    Ok(selector.flush_and_fetch().await?.map($target::from))
                })
            }
        }
    };
}

macro_rules! impl_multi_selector {
    ($class: ident, $target: ident) => {
        #[pymethods]
        impl $class {
            pub fn as_selector(&self) -> Self {
                self.clone()
            }

            pub fn as_client(&self) -> $crate::client::Client {
                use ::libawr::meta::selector::Selector;

                self.inner.as_client().clone().into()
            }

            pub fn fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                use crate::utils::{py_future, py_obj};
                use libawr::meta::selector::MultiSelector;
                use pyo3::types::PyDict;

                let selector = self.inner.clone();
                py_future(py, async move {
                    let result: Vec<_> = selector
                        .fetch()
                        .await?
                        .into_iter()
                        .map(|(k, v)| Ok((k, py_obj($target::from(v))?)))
                        .collect::<PyResult<_>>()?;
                    Ok(Python::with_gil(|py| -> Py<PyDict> {
                        result.into_py_dict(py).into()
                    }))
                })
            }

            pub fn flush<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                use crate::utils::py_future;
                use libawr::meta::selector::Selector;

                let selector = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    selector.flush().await;
                    Ok(self_)
                })
            }

            pub fn flush_and_fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                use crate::utils::{py_future, py_obj};
                use libawr::meta::selector::MultiSelector;
                use pyo3::types::PyDict;

                let selector = self.inner.clone();
                py_future(py, async move {
                    let result: Vec<_> = selector
                        .flush_and_fetch()
                        .await?
                        .into_iter()
                        .map(|(k, v)| Ok((k, py_obj($target::from(v))?)))
                        .collect::<PyResult<_>>()?;
                    Ok(Python::with_gil(|py| -> Py<PyDict> {
                        result.into_py_dict(py).into()
                    }))
                })
            }
        }
    };
}
