#![feature(prelude_import)]
//! 基于 [`ricq`] 包装，供 Python 使用的 QQ 无头客户端。
//!
//! 更多信息请参考 [`login`] 和 [`client`] 模块。
//!
//! # Examples
//! ```python
//! import awr
//! import asyncio
//!
//! async def main():
//!     ## 登录账号
//!     client = await awr.Dynamic().login(12345678, "./bots")
//!     ## 读取群列表
//!     print(await client.get_group_list())
//!     ## 保持连接
//!     await client.alive()
//!     
//! try:
//!     asyncio.run(main())
//! except KeyboardInterrupt:
//!     import sys
//!     sys.exit(0)
//! ```
//!
//! [`ricq`]: https://docs.rs/ricq/latest/ricq/
#![deny(missing_docs)]
#![feature(type_alias_impl_trait)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use pyo3::prelude::*;
use pyo3_built::pyo3_built;
use tracing::info;
#[macro_use]
mod utils {
    use anyhow::Result;
    use futures_util::Future;
    use pyo3::prelude::*;
    /// 获取 Python 的 None。
    pub fn py_none() -> PyObject {
        Python::with_gil(|py| py.None())
    }
    /// 将 Rust 定义的 Python 类实例化。
    pub fn py_obj<T>(obj: impl Into<PyClassInitializer<T>>) -> PyResult<Py<T>>
    where
        T: pyo3::PyClass,
    {
        Python::with_gil(|py| Py::new(py, obj))
    }
    /// 将 [`tokio`] 的 Future 包装为 Python 的 Future。
    pub fn py_future<F, T>(py: Python, future: F) -> PyResult<&PyAny>
    where
        F: Future<Output = Result<T, anyhow::Error>> + Send + 'static,
        T: IntoPy<PyObject>,
    {
        pyo3_asyncio::tokio::future_into_py(py, async move { Ok(future.await?) })
    }
    /// 自动重试直到得到 `Ok(..)`。
    pub async fn retry<F, T, D>(
        mut max_count: usize,
        mut f: impl FnMut() -> F,
        mut on_retry: impl FnMut(anyhow::Error, usize) -> D,
    ) -> Result<T>
    where
        F: Future<Output = Result<T>>,
        D: Future<Output = ()>,
    {
        loop {
            match f().await {
                Ok(t) => return Ok(t),
                Err(e) => {
                    if max_count == 0 {
                        return Err(e);
                    }
                    max_count -= 1;
                    on_retry(e, max_count).await;
                }
            }
        }
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
}
pub mod client {
    //! QQ 无头客户端。
    //!
    //! 更多信息参考 [`Client`]。
    use std::sync::Arc;
    use pyo3::prelude::*;
    use crate::client::friend::FriendSelector;
    mod friend {
        //! 好友。
        use std::sync::Arc;
        use libawr::selector::{RemoteTarget, SingleSelector};
        use pyo3::{prelude::*, types::PyTuple};
        use crate::utils::{py_future, py_none, py_obj};
        /// 好友。
        pub struct Friend {
            pub(crate) inner: Arc<libawr::client::friend::Friend>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Friend {
            #[inline]
            fn clone(&self) -> Friend {
                Friend {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            unsafe impl _pyo3::type_object::PyTypeInfo for Friend {
                type AsRefTarget = _pyo3::PyCell<Self>;
                const NAME: &'static str = "Friend";
                const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
                #[inline]
                fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                    use _pyo3::type_object::LazyStaticType;
                    static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                    TYPE_OBJECT.get_or_init::<Self>(py)
                }
            }
            impl _pyo3::PyClass for Friend {
                type Frozen = _pyo3::pyclass::boolean_struct::False;
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a Friend {
                type Holder = ::std::option::Option<_pyo3::PyRef<'py, Friend>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
                }
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a mut Friend {
                type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, Friend>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
                }
            }
            impl _pyo3::IntoPy<_pyo3::PyObject> for Friend {
                fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                    _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
                }
            }
            impl _pyo3::impl_::pyclass::PyClassImpl for Friend {
                const DOC: &'static str = "\u{597d}\u{53cb}\u{3002}\u{0}";
                const IS_BASETYPE: bool = false;
                const IS_SUBCLASS: bool = false;
                const IS_MAPPING: bool = false;
                const IS_SEQUENCE: bool = false;
                type Layout = _pyo3::PyCell<Self>;
                type BaseType = _pyo3::PyAny;
                type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<Friend>;
                type Inventory = Pyo3MethodsInventoryForFriend;
                type PyClassMutability = < < _pyo3 :: PyAny as _pyo3 :: impl_ :: pyclass :: PyClassBaseType > :: PyClassMutability as _pyo3 :: impl_ :: pycell :: PyClassMutability > :: MutableChild ;
                type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
                type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
                type BaseNativeType = _pyo3::PyAny;
                fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                    use _pyo3::impl_::pyclass::*;
                    let collector = PyClassImplCollector::<Self>::new();
                    static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                        methods: &[],
                        slots: &[],
                    };
                    PyClassItemsIter::new(
                        &INTRINSIC_ITEMS,
                        ::std::boxed::Box::new(::std::iter::Iterator::map(
                            _pyo3::inventory::iter::<
                                <Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory,
                            >(),
                            _pyo3::impl_::pyclass::PyClassInventory::items,
                        )),
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl Friend {}
            #[doc(hidden)]
            pub struct Pyo3MethodsInventoryForFriend {
                items: _pyo3::impl_::pyclass::PyClassItems,
            }
            impl Pyo3MethodsInventoryForFriend {
                pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                    Self { items }
                }
            }
            impl _pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForFriend {
                fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                    &self.items
                }
            }
            impl ::inventory::Collect for Pyo3MethodsInventoryForFriend {
                #[inline]
                fn registry() -> &'static ::inventory::Registry {
                    static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
                    &REGISTRY
                }
            }
        };
        impl From<Arc<libawr::client::friend::Friend>> for Friend {
            fn from(inner: Arc<libawr::client::friend::Friend>) -> Self {
                Self { inner }
            }
        }
        impl Friend {
            pub fn uin(&self) -> i64 {
                crate::utils::PyPropertyConvert::<i64, i64>::convert(&self.inner.uin)
            }
            pub fn nickname(&self) -> &str {
                crate::utils::PyPropertyConvert::<String, &str>::convert(&self.inner.nickname)
            }
            pub fn remark(&self) -> &str {
                crate::utils::PyPropertyConvert::<String, &str>::convert(&self.inner.remark)
            }
            pub fn face_id(&self) -> i16 {
                crate::utils::PyPropertyConvert::<i16, i16>::convert(&self.inner.face_id)
            }
            pub fn group_id(&self) -> u8 {
                crate::utils::PyPropertyConvert::<u8, u8>::convert(&self.inner.group_id)
            }
            fn __repr__(&self) -> String {
                {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Friend(", ")"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &[
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["uin="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.uin())],
                                    ));
                                    res
                                },
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["nickname="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.nickname())],
                                    ));
                                    res
                                },
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["remark="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.remark())],
                                    ));
                                    res
                                },
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["face_id="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.face_id())],
                                    ));
                                    res
                                },
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["group_id="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.group_id())],
                                    ));
                                    res
                                },
                            ]
                            .join(", "),
                        )],
                    ));
                    res
                }
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <Friend as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory::new(_pyo3::impl_::pyclass::PyClassItems {
                                methods: &[
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "uin\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                Friend::__pymethod_get_uin__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "nickname\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                Friend::__pymethod_get_nickname__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "remark\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                Friend::__pymethod_get_remark__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "face_id\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                Friend::__pymethod_get_face_id__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "group_id\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                Friend::__pymethod_get_group_id__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                ],
                                slots: &[_pyo3::ffi::PyType_Slot {
                                    slot: _pyo3::ffi::Py_tp_repr,
                                    pfunc: Friend::__pymethod___repr____ as _pyo3::ffi::reprfunc
                                        as _,
                                }],
                            })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl Friend {
                unsafe extern "C" fn __pymethod_get_uin__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &Friend = &*_ref;
                            let item = Friend::uin(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_get_nickname__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &Friend = &*_ref;
                            let item = Friend::nickname(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_get_remark__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &Friend = &*_ref;
                            let item = Friend::remark(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_get_face_id__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &Friend = &*_ref;
                            let item = Friend::face_id(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_get_group_id__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &Friend = &*_ref;
                            let item = Friend::group_id(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod___repr____(
                    _raw_slf: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let _slf = _raw_slf;
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &Friend = &*_ref;
                            _pyo3::callback::convert(_py, Friend::__repr__(_slf))
                        }),
                    )
                }
            }
        };
        impl Friend {
            pub fn to_selector(&self) -> FriendSelector {
                self.inner.to_selector().into()
            }
            pub fn __getattr__(&self, py: Python, name: &str) -> PyResult<PyObject> {
                self.to_selector().into_py(py).getattr(py, name)
            }
            pub fn flush<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let mut inner = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    inner.flush().await;
                    Ok(self_)
                })
            }
            pub fn sync<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let mut inner = self_.borrow(py).inner.clone();
                py_future(py, async move { Ok(inner.sync().await?) })
            }
            pub fn flush_and_sync<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let mut inner = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    inner.flush_and_sync().await?;
                    Python::with_gil(|py| {
                        self_.borrow_mut(py).inner = inner;
                    });
                    Ok(self_)
                })
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            impl _pyo3::impl_::pyclass::PyClass__getattr__SlotFragment<Friend>
                for _pyo3::impl_::pyclass::PyClassImplCollector<Friend>
            {
                #[inline]
                unsafe fn __getattr__(
                    self,
                    _py: _pyo3::Python,
                    _raw_slf: *mut _pyo3::ffi::PyObject,
                    arg0: *mut _pyo3::ffi::PyObject,
                ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
                    impl Friend {
                        unsafe fn __pymethod___getattr____(
                            _py: _pyo3::Python,
                            _raw_slf: *mut _pyo3::ffi::PyObject,
                            arg0: *mut _pyo3::ffi::PyObject,
                        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
                            let _slf = _raw_slf;
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &Friend = &*_ref;
                            _pyo3::callback::convert(
                                _py,
                                Friend::__getattr__(
                                    _slf,
                                    _py,
                                    _pyo3::impl_::extract_argument::extract_argument(
                                        _py.from_borrowed_ptr::<_pyo3::PyAny>(arg0),
                                        &mut {
                                            _pyo3 :: impl_ :: extract_argument :: FunctionArgumentHolder :: INIT
                                        },
                                        "name",
                                    )?,
                                ),
                            )
                        }
                    }
                    Friend::__pymethod___getattr____(_py, _raw_slf, arg0)
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <Friend as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory :: new (_pyo3 :: impl_ :: pyclass :: PyClassItems { methods : & [_pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: noargs ("to_selector\0" , _pyo3 :: impl_ :: pymethods :: PyCFunction (Friend :: __pymethod_to_selector__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (Friend :: __pymethod_flush__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("sync\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (Friend :: __pymethod_sync__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush_and_sync\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (Friend :: __pymethod_flush_and_sync__) , "\u{0}"))] , slots : & [{ unsafe extern "C" fn __wrap (_slf : * mut :: pyo3 :: ffi :: PyObject , attr : * mut :: pyo3 :: ffi :: PyObject) -> * mut :: pyo3 :: ffi :: PyObject { use :: std :: result :: Result :: * ; use :: pyo3 :: impl_ :: pyclass :: * ; let gil = :: pyo3 :: GILPool :: new () ; let py = gil . python () ; :: pyo3 :: callback :: panic_result_into_callback_output (py , :: std :: panic :: catch_unwind (move | | -> :: pyo3 :: PyResult < _ > { let collector = PyClassImplCollector :: < Friend > :: new () ; match collector . __getattribute__ (py , _slf , attr) { Ok (obj) => Ok (obj) , Err (e) if e . is_instance_of :: < :: pyo3 :: exceptions :: PyAttributeError > (py) => { collector . __getattr__ (py , _slf , attr) } Err (e) => Err (e) , } })) } :: pyo3 :: ffi :: PyType_Slot { slot : :: pyo3 :: ffi :: Py_tp_getattro , pfunc : __wrap as :: pyo3 :: ffi :: getattrofunc as _ , } }] , })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl Friend {
                unsafe extern "C" fn __pymethod_to_selector__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &Friend = &*_ref;
                            let mut ret = Friend::to_selector(_slf);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <Friend as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "flush",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = Friend::flush(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_sync__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <Friend as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "sync",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = Friend::sync(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush_and_sync__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<Friend>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <Friend as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "flush_and_sync",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = Friend::flush_and_sync(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
            }
        };
        impl Friend {}
        const _: () = {
            use :: pyo3 as _pyo3;
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <Friend as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory::new(_pyo3::impl_::pyclass::PyClassItems {
                                methods: &[],
                                slots: &[],
                            })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl Friend {}
        };
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
        pub struct FriendSelector {
            pub(crate) inner: libawr::client::friend::FriendSelector,
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            unsafe impl _pyo3::type_object::PyTypeInfo for FriendSelector {
                type AsRefTarget = _pyo3::PyCell<Self>;
                const NAME: &'static str = "FriendSelector";
                const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
                #[inline]
                fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                    use _pyo3::type_object::LazyStaticType;
                    static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                    TYPE_OBJECT.get_or_init::<Self>(py)
                }
            }
            impl _pyo3::PyClass for FriendSelector {
                type Frozen = _pyo3::pyclass::boolean_struct::False;
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a FriendSelector {
                type Holder = ::std::option::Option<_pyo3::PyRef<'py, FriendSelector>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
                }
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
                for &'a mut FriendSelector
            {
                type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, FriendSelector>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
                }
            }
            impl _pyo3::IntoPy<_pyo3::PyObject> for FriendSelector {
                fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                    _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
                }
            }
            impl _pyo3::impl_::pyclass::PyClassImpl for FriendSelector {
                const DOC : & 'static str = "\u{597d}\u{53cb}\u{9009}\u{62e9}\u{5668}\u{3002}\n\n# Examples\n```python\nawait client.friend(12345678).poke()\n```\n\n# Python\n```python\nclass FriendSelector:\n    @property\n    def uin(self) -> int: ...\n```\u{0}" ;
                const IS_BASETYPE: bool = false;
                const IS_SUBCLASS: bool = false;
                const IS_MAPPING: bool = false;
                const IS_SEQUENCE: bool = false;
                type Layout = _pyo3::PyCell<Self>;
                type BaseType = _pyo3::PyAny;
                type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<FriendSelector>;
                type Inventory = Pyo3MethodsInventoryForFriendSelector;
                type PyClassMutability = < < _pyo3 :: PyAny as _pyo3 :: impl_ :: pyclass :: PyClassBaseType > :: PyClassMutability as _pyo3 :: impl_ :: pycell :: PyClassMutability > :: MutableChild ;
                type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
                type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
                type BaseNativeType = _pyo3::PyAny;
                fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                    use _pyo3::impl_::pyclass::*;
                    let collector = PyClassImplCollector::<Self>::new();
                    static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                        methods: &[],
                        slots: &[],
                    };
                    PyClassItemsIter::new(
                        &INTRINSIC_ITEMS,
                        ::std::boxed::Box::new(::std::iter::Iterator::map(
                            _pyo3::inventory::iter::<
                                <Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory,
                            >(),
                            _pyo3::impl_::pyclass::PyClassInventory::items,
                        )),
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendSelector {}
            #[doc(hidden)]
            pub struct Pyo3MethodsInventoryForFriendSelector {
                items: _pyo3::impl_::pyclass::PyClassItems,
            }
            impl Pyo3MethodsInventoryForFriendSelector {
                pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                    Self { items }
                }
            }
            impl _pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForFriendSelector {
                fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                    &self.items
                }
            }
            impl ::inventory::Collect for Pyo3MethodsInventoryForFriendSelector {
                #[inline]
                fn registry() -> &'static ::inventory::Registry {
                    static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
                    &REGISTRY
                }
            }
        };
        impl From<libawr::client::friend::FriendSelector> for FriendSelector {
            fn from(inner: libawr::client::friend::FriendSelector) -> Self {
                Self { inner }
            }
        }
        impl FriendSelector {
            pub fn fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                let selector = self.inner.clone();
                py_future(py, async move {
                    match selector.fetch().await? {
                        Some(friend) => Ok(Some(py_obj(Friend::from(friend))?)),
                        None => Ok(None),
                    }
                })
            }
            pub fn flush<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let selector = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    selector.flush().await;
                    Ok(self_)
                })
            }
            pub fn flush_and_fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                let selector = self.inner.clone();
                py_future(py, async move {
                    match selector.flush_and_fetch().await? {
                        Some(friend) => Ok(Some(py_obj(Friend::from(friend))?)),
                        None => Ok(None),
                    }
                })
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <FriendSelector as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory :: new (_pyo3 :: impl_ :: pyclass :: PyClassItems { methods : & [_pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("fetch\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendSelector :: __pymethod_fetch__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendSelector :: __pymethod_flush__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush_and_fetch\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendSelector :: __pymethod_flush_and_fetch__) , "\u{0}"))] , slots : & [] , })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendSelector {
                unsafe extern "C" fn __pymethod_fetch__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendSelector>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendSelector = &*_ref;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendSelector as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "fetch",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendSelector::fetch(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendSelector>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendSelector as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "flush",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendSelector::flush(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush_and_fetch__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendSelector>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendSelector = &*_ref;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendSelector as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "flush_and_fetch",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendSelector::flush_and_fetch(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
            }
        };
        impl FriendSelector {
            pub fn poke<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                let selector = self.inner.clone();
                py_future(py, async move {
                    selector.poke().await?;
                    Ok(py_none())
                })
            }
            pub fn send<'py>(
                &self,
                py: Python<'py>,
                segments: &'py PyTuple,
            ) -> PyResult<&'py PyAny> {
                ::core::panicking::panic("not yet implemented")
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <FriendSelector as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory :: new (_pyo3 :: impl_ :: pyclass :: PyClassItems { methods : & [_pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("poke\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendSelector :: __pymethod_poke__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("send\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendSelector :: __pymethod_send__) , "\u{0}"))] , slots : & [] , })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendSelector {
                unsafe extern "C" fn __pymethod_poke__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendSelector>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendSelector = &*_ref;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendSelector as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "poke",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendSelector::poke(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_send__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendSelector>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendSelector = &*_ref;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendSelector as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "send",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: TupleVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendSelector::send(
                                _slf,
                                _py,
                                _pyo3::impl_::extract_argument::extract_argument(
                                    _args,
                                    &mut {
                                        _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT
                                    },
                                    "segments",
                                )?,
                            );
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
            }
        };
    }
    mod friend_list {
        //! 好友列表。
        //!
        //! 更多信息参考 [`FriendList`]。
        use std::sync::Arc;
        use libawr::selector::{RemoteTarget, SingleSelector};
        use pyo3::{prelude::*, types::*};
        use crate::{
            client::friend::Friend,
            utils::{py_future, py_obj},
        };
        /// 好友列表。
        ///
        /// # Python
        /// ```python
        /// class FriendList:
        /// ```
        pub struct FriendList {
            inner: Arc<libawr::client::friend_list::FriendList>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for FriendList {
            #[inline]
            fn clone(&self) -> FriendList {
                FriendList {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            unsafe impl _pyo3::type_object::PyTypeInfo for FriendList {
                type AsRefTarget = _pyo3::PyCell<Self>;
                const NAME: &'static str = "FriendList";
                const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
                #[inline]
                fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                    use _pyo3::type_object::LazyStaticType;
                    static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                    TYPE_OBJECT.get_or_init::<Self>(py)
                }
            }
            impl _pyo3::PyClass for FriendList {
                type Frozen = _pyo3::pyclass::boolean_struct::False;
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a FriendList {
                type Holder = ::std::option::Option<_pyo3::PyRef<'py, FriendList>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
                }
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a mut FriendList {
                type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, FriendList>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
                }
            }
            impl _pyo3::IntoPy<_pyo3::PyObject> for FriendList {
                fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                    _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
                }
            }
            impl _pyo3::impl_::pyclass::PyClassImpl for FriendList {
                const DOC : & 'static str = "\u{597d}\u{53cb}\u{5217}\u{8868}\u{3002}\n\n# Python\n```python\nclass FriendList:\n```\u{0}" ;
                const IS_BASETYPE: bool = false;
                const IS_SUBCLASS: bool = false;
                const IS_MAPPING: bool = false;
                const IS_SEQUENCE: bool = false;
                type Layout = _pyo3::PyCell<Self>;
                type BaseType = _pyo3::PyAny;
                type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<FriendList>;
                type Inventory = Pyo3MethodsInventoryForFriendList;
                type PyClassMutability = < < _pyo3 :: PyAny as _pyo3 :: impl_ :: pyclass :: PyClassBaseType > :: PyClassMutability as _pyo3 :: impl_ :: pycell :: PyClassMutability > :: MutableChild ;
                type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
                type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
                type BaseNativeType = _pyo3::PyAny;
                fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                    use _pyo3::impl_::pyclass::*;
                    let collector = PyClassImplCollector::<Self>::new();
                    static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                        methods: &[],
                        slots: &[],
                    };
                    PyClassItemsIter::new(
                        &INTRINSIC_ITEMS,
                        ::std::boxed::Box::new(::std::iter::Iterator::map(
                            _pyo3::inventory::iter::<
                                <Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory,
                            >(),
                            _pyo3::impl_::pyclass::PyClassInventory::items,
                        )),
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendList {}
            #[doc(hidden)]
            pub struct Pyo3MethodsInventoryForFriendList {
                items: _pyo3::impl_::pyclass::PyClassItems,
            }
            impl Pyo3MethodsInventoryForFriendList {
                pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                    Self { items }
                }
            }
            impl _pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForFriendList {
                fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                    &self.items
                }
            }
            impl ::inventory::Collect for Pyo3MethodsInventoryForFriendList {
                #[inline]
                fn registry() -> &'static ::inventory::Registry {
                    static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
                    &REGISTRY
                }
            }
        };
        impl From<Arc<libawr::client::friend_list::FriendList>> for FriendList {
            fn from(inner: Arc<libawr::client::friend_list::FriendList>) -> Self {
                Self { inner }
            }
        }
        impl FriendList {
            pub fn total_count(&self) -> i16 {
                crate::utils::PyPropertyConvert::<i16, i16>::convert(&self.inner.total_count)
            }
            pub fn online_count(&self) -> i16 {
                crate::utils::PyPropertyConvert::<i16, i16>::convert(&self.inner.online_count)
            }
            fn __repr__(&self) -> String {
                {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["FriendList(", ")"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &[
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["total_count="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.total_count())],
                                    ));
                                    res
                                },
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["online_count="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.online_count())],
                                    ));
                                    res
                                },
                            ]
                            .join(", "),
                        )],
                    ));
                    res
                }
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <FriendList as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory::new(_pyo3::impl_::pyclass::PyClassItems {
                                methods: &[
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "total_count\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                FriendList::__pymethod_get_total_count__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "online_count\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                FriendList::__pymethod_get_online_count__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                ],
                                slots: &[_pyo3::ffi::PyType_Slot {
                                    slot: _pyo3::ffi::Py_tp_repr,
                                    pfunc: FriendList::__pymethod___repr____ as _pyo3::ffi::reprfunc
                                        as _,
                                }],
                            })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendList {
                unsafe extern "C" fn __pymethod_get_total_count__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendList>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendList = &*_ref;
                            let item = FriendList::total_count(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_get_online_count__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendList>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendList = &*_ref;
                            let item = FriendList::online_count(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod___repr____(
                    _raw_slf: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let _slf = _raw_slf;
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendList>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendList = &*_ref;
                            _pyo3::callback::convert(_py, FriendList::__repr__(_slf))
                        }),
                    )
                }
            }
        };
        impl FriendList {
            pub fn to_selector(&self) -> FriendListSelector {
                self.inner.to_selector().into()
            }
            pub fn __getattr__(&self, py: Python, name: &str) -> PyResult<PyObject> {
                self.to_selector().into_py(py).getattr(py, name)
            }
            pub fn flush<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let mut inner = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    inner.flush().await;
                    Ok(self_)
                })
            }
            pub fn sync<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let mut inner = self_.borrow(py).inner.clone();
                py_future(py, async move { Ok(inner.sync().await?) })
            }
            pub fn flush_and_sync<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let mut inner = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    inner.flush_and_sync().await?;
                    Python::with_gil(|py| {
                        self_.borrow_mut(py).inner = inner;
                    });
                    Ok(self_)
                })
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            impl _pyo3::impl_::pyclass::PyClass__getattr__SlotFragment<FriendList>
                for _pyo3::impl_::pyclass::PyClassImplCollector<FriendList>
            {
                #[inline]
                unsafe fn __getattr__(
                    self,
                    _py: _pyo3::Python,
                    _raw_slf: *mut _pyo3::ffi::PyObject,
                    arg0: *mut _pyo3::ffi::PyObject,
                ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
                    impl FriendList {
                        unsafe fn __pymethod___getattr____(
                            _py: _pyo3::Python,
                            _raw_slf: *mut _pyo3::ffi::PyObject,
                            arg0: *mut _pyo3::ffi::PyObject,
                        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
                            let _slf = _raw_slf;
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendList>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendList = &*_ref;
                            _pyo3::callback::convert(
                                _py,
                                FriendList::__getattr__(
                                    _slf,
                                    _py,
                                    _pyo3::impl_::extract_argument::extract_argument(
                                        _py.from_borrowed_ptr::<_pyo3::PyAny>(arg0),
                                        &mut {
                                            _pyo3 :: impl_ :: extract_argument :: FunctionArgumentHolder :: INIT
                                        },
                                        "name",
                                    )?,
                                ),
                            )
                        }
                    }
                    FriendList::__pymethod___getattr____(_py, _raw_slf, arg0)
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <FriendList as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory :: new (_pyo3 :: impl_ :: pyclass :: PyClassItems { methods : & [_pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: noargs ("to_selector\0" , _pyo3 :: impl_ :: pymethods :: PyCFunction (FriendList :: __pymethod_to_selector__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendList :: __pymethod_flush__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("sync\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendList :: __pymethod_sync__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush_and_sync\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendList :: __pymethod_flush_and_sync__) , "\u{0}"))] , slots : & [{ unsafe extern "C" fn __wrap (_slf : * mut :: pyo3 :: ffi :: PyObject , attr : * mut :: pyo3 :: ffi :: PyObject) -> * mut :: pyo3 :: ffi :: PyObject { use :: std :: result :: Result :: * ; use :: pyo3 :: impl_ :: pyclass :: * ; let gil = :: pyo3 :: GILPool :: new () ; let py = gil . python () ; :: pyo3 :: callback :: panic_result_into_callback_output (py , :: std :: panic :: catch_unwind (move | | -> :: pyo3 :: PyResult < _ > { let collector = PyClassImplCollector :: < FriendList > :: new () ; match collector . __getattribute__ (py , _slf , attr) { Ok (obj) => Ok (obj) , Err (e) if e . is_instance_of :: < :: pyo3 :: exceptions :: PyAttributeError > (py) => { collector . __getattr__ (py , _slf , attr) } Err (e) => Err (e) , } })) } :: pyo3 :: ffi :: PyType_Slot { slot : :: pyo3 :: ffi :: Py_tp_getattro , pfunc : __wrap as :: pyo3 :: ffi :: getattrofunc as _ , } }] , })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendList {
                unsafe extern "C" fn __pymethod_to_selector__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendList>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendList = &*_ref;
                            let mut ret = FriendList::to_selector(_slf);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendList>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendList as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "flush",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendList::flush(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_sync__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendList>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendList as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "sync",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendList::sync(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush_and_sync__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendList>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendList as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "flush_and_sync",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendList::flush_and_sync(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
            }
        };
        impl FriendList {
            /// 获取好友信息。
            ///
            /// 参考 [`Friend`]。
            ///
            /// # Examples
            /// ```python
            /// friend_list = await client.get_friend_list()
            /// for friend in friend_list.friends().values():
            ///     print(friend.nickname)
            /// ```
            ///
            /// # Python
            /// ```python
            /// def friends(self) -> dict[int, Friend]:
            /// ```
            pub fn friends<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
                let friends: Vec<_> = self
                    .inner
                    .friends
                    .iter()
                    .map(|(uin, info)| Ok((*uin, py_obj(Friend::from(info.clone()))?)))
                    .collect::<PyResult<_>>()?;
                Ok(friends.into_py_dict(py))
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
            /// def friend_groups(self) -> dict[int, FriendGroup]:
            /// ```
            pub fn friend_groups<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
                let friend_groups: Vec<_> = self
                    .inner
                    .friends
                    .iter()
                    .map(|(uin, info)| Ok((*uin, py_obj(Friend::from(info.clone()))?)))
                    .collect::<PyResult<_>>()?;
                Ok(friend_groups.into_py_dict(py))
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <FriendList as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory :: new (_pyo3 :: impl_ :: pyclass :: PyClassItems { methods : & [_pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("friends\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendList :: __pymethod_friends__) , "\u{83b7}\u{53d6}\u{597d}\u{53cb}\u{4fe1}\u{606f}\u{3002}\n\n\u{53c2}\u{8003} [`Friend`]\u{3002}\n\n# Examples\n```python\nfriend_list = await client.get_friend_list()\nfor friend in friend_list.friends().values():\n    print(friend.nickname)\n```\n\n# Python\n```python\ndef friends(self) -> dict[int, Friend]:\n```\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("friend_groups\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendList :: __pymethod_friend_groups__) , "\u{83b7}\u{53d6}\u{6240}\u{6709}\u{597d}\u{53cb}\u{5206}\u{7ec4}\u{4fe1}\u{606f}\u{3002}\n\n\u{53c2}\u{8003} [`FriendGroup`]\u{3002}\n\n# Examples\n```python\nfriend_list = await client.get_friend_list()\nfor group in friend_list.friend_groups():\n    print(group.name)\n```\n\n# Python\n```python\ndef friend_groups(self) -> dict[int, FriendGroup]:\n```\u{0}"))] , slots : & [] , })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendList {
                unsafe extern "C" fn __pymethod_friends__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendList>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendList = &*_ref;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendList as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "friends",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendList::friends(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_friend_groups__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendList>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendList = &*_ref;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendList as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "friend_groups",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendList::friend_groups(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
            }
        };
        pub struct FriendListSelector {
            inner: libawr::client::friend_list::FriendListSelector,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for FriendListSelector {
            #[inline]
            fn clone(&self) -> FriendListSelector {
                FriendListSelector {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            unsafe impl _pyo3::type_object::PyTypeInfo for FriendListSelector {
                type AsRefTarget = _pyo3::PyCell<Self>;
                const NAME: &'static str = "FriendListSelector";
                const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
                #[inline]
                fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                    use _pyo3::type_object::LazyStaticType;
                    static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                    TYPE_OBJECT.get_or_init::<Self>(py)
                }
            }
            impl _pyo3::PyClass for FriendListSelector {
                type Frozen = _pyo3::pyclass::boolean_struct::False;
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
                for &'a FriendListSelector
            {
                type Holder = ::std::option::Option<_pyo3::PyRef<'py, FriendListSelector>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
                }
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
                for &'a mut FriendListSelector
            {
                type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, FriendListSelector>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
                }
            }
            impl _pyo3::IntoPy<_pyo3::PyObject> for FriendListSelector {
                fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                    _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
                }
            }
            impl _pyo3::impl_::pyclass::PyClassImpl for FriendListSelector {
                const DOC: &'static str = "\u{0}";
                const IS_BASETYPE: bool = false;
                const IS_SUBCLASS: bool = false;
                const IS_MAPPING: bool = false;
                const IS_SEQUENCE: bool = false;
                type Layout = _pyo3::PyCell<Self>;
                type BaseType = _pyo3::PyAny;
                type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<FriendListSelector>;
                type Inventory = Pyo3MethodsInventoryForFriendListSelector;
                type PyClassMutability = < < _pyo3 :: PyAny as _pyo3 :: impl_ :: pyclass :: PyClassBaseType > :: PyClassMutability as _pyo3 :: impl_ :: pycell :: PyClassMutability > :: MutableChild ;
                type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
                type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
                type BaseNativeType = _pyo3::PyAny;
                fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                    use _pyo3::impl_::pyclass::*;
                    let collector = PyClassImplCollector::<Self>::new();
                    static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                        methods: &[],
                        slots: &[],
                    };
                    PyClassItemsIter::new(
                        &INTRINSIC_ITEMS,
                        ::std::boxed::Box::new(::std::iter::Iterator::map(
                            _pyo3::inventory::iter::<
                                <Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory,
                            >(),
                            _pyo3::impl_::pyclass::PyClassInventory::items,
                        )),
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendListSelector {}
            #[doc(hidden)]
            pub struct Pyo3MethodsInventoryForFriendListSelector {
                items: _pyo3::impl_::pyclass::PyClassItems,
            }
            impl Pyo3MethodsInventoryForFriendListSelector {
                pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                    Self { items }
                }
            }
            impl _pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForFriendListSelector {
                fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                    &self.items
                }
            }
            impl ::inventory::Collect for Pyo3MethodsInventoryForFriendListSelector {
                #[inline]
                fn registry() -> &'static ::inventory::Registry {
                    static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
                    &REGISTRY
                }
            }
        };
        impl From<libawr::client::friend_list::FriendListSelector> for FriendListSelector {
            fn from(inner: libawr::client::friend_list::FriendListSelector) -> Self {
                Self { inner }
            }
        }
        impl FriendListSelector {
            pub fn fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                let selector = self.inner.clone();
                py_future(py, async move {
                    match selector.fetch().await? {
                        Some(friend) => Ok(Some(py_obj(FriendList::from(friend))?)),
                        None => Ok(None),
                    }
                })
            }
            pub fn flush<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let selector = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    selector.flush().await;
                    Ok(self_)
                })
            }
            pub fn flush_and_fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                let selector = self.inner.clone();
                py_future(py, async move {
                    match selector.flush_and_fetch().await? {
                        Some(friend) => Ok(Some(py_obj(FriendList::from(friend))?)),
                        None => Ok(None),
                    }
                })
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory = < FriendListSelector as _pyo3 :: impl_ :: pyclass :: PyClassImpl > :: Inventory ;
                            Inventory :: new (_pyo3 :: impl_ :: pyclass :: PyClassItems { methods : & [_pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("fetch\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendListSelector :: __pymethod_fetch__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendListSelector :: __pymethod_flush__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush_and_fetch\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendListSelector :: __pymethod_flush_and_fetch__) , "\u{0}"))] , slots : & [] , })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendListSelector {
                unsafe extern "C" fn __pymethod_fetch__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendListSelector>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendListSelector = &*_ref;
                            const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< FriendListSelector as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "fetch" , positional_parameter_names : & [] , positional_only_parameters : 0usize , required_positional_parameters : 0usize , keyword_only_parameters : & [] , } ;
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendListSelector::fetch(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendListSelector>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< FriendListSelector as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "flush" , positional_parameter_names : & [] , positional_only_parameters : 0usize , required_positional_parameters : 0usize , keyword_only_parameters : & [] , } ;
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendListSelector::flush(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush_and_fetch__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendListSelector>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendListSelector = &*_ref;
                            const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< FriendListSelector as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "flush_and_fetch" , positional_parameter_names : & [] , positional_only_parameters : 0usize , required_positional_parameters : 0usize , keyword_only_parameters : & [] , } ;
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendListSelector::flush_and_fetch(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
            }
        };
    }
    mod friend_group {
        //! 好友分组。
        //!
        //! 更多信息参考 [`FriendGroup`]。
        use std::sync::Arc;
        use libawr::selector::{RemoteTarget, SingleSelector};
        use pyo3::prelude::*;
        use crate::utils::{py_future, py_obj};
        /// 好友分组。
        ///
        /// # Python
        /// ```python
        /// class FriendGroup: ...
        /// ```
        pub struct FriendGroup {
            pub(crate) inner: Arc<libawr::client::friend_group::FriendGroup>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for FriendGroup {
            #[inline]
            fn clone(&self) -> FriendGroup {
                FriendGroup {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            unsafe impl _pyo3::type_object::PyTypeInfo for FriendGroup {
                type AsRefTarget = _pyo3::PyCell<Self>;
                const NAME: &'static str = "FriendGroup";
                const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
                #[inline]
                fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                    use _pyo3::type_object::LazyStaticType;
                    static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                    TYPE_OBJECT.get_or_init::<Self>(py)
                }
            }
            impl _pyo3::PyClass for FriendGroup {
                type Frozen = _pyo3::pyclass::boolean_struct::False;
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a FriendGroup {
                type Holder = ::std::option::Option<_pyo3::PyRef<'py, FriendGroup>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
                }
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a mut FriendGroup {
                type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, FriendGroup>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
                }
            }
            impl _pyo3::IntoPy<_pyo3::PyObject> for FriendGroup {
                fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                    _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
                }
            }
            impl _pyo3::impl_::pyclass::PyClassImpl for FriendGroup {
                const DOC : & 'static str = "\u{597d}\u{53cb}\u{5206}\u{7ec4}\u{3002}\n\n# Python\n```python\nclass FriendGroup: ...\n```\u{0}" ;
                const IS_BASETYPE: bool = false;
                const IS_SUBCLASS: bool = false;
                const IS_MAPPING: bool = false;
                const IS_SEQUENCE: bool = false;
                type Layout = _pyo3::PyCell<Self>;
                type BaseType = _pyo3::PyAny;
                type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<FriendGroup>;
                type Inventory = Pyo3MethodsInventoryForFriendGroup;
                type PyClassMutability = < < _pyo3 :: PyAny as _pyo3 :: impl_ :: pyclass :: PyClassBaseType > :: PyClassMutability as _pyo3 :: impl_ :: pycell :: PyClassMutability > :: MutableChild ;
                type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
                type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
                type BaseNativeType = _pyo3::PyAny;
                fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                    use _pyo3::impl_::pyclass::*;
                    let collector = PyClassImplCollector::<Self>::new();
                    static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                        methods: &[],
                        slots: &[],
                    };
                    PyClassItemsIter::new(
                        &INTRINSIC_ITEMS,
                        ::std::boxed::Box::new(::std::iter::Iterator::map(
                            _pyo3::inventory::iter::<
                                <Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory,
                            >(),
                            _pyo3::impl_::pyclass::PyClassInventory::items,
                        )),
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendGroup {}
            #[doc(hidden)]
            pub struct Pyo3MethodsInventoryForFriendGroup {
                items: _pyo3::impl_::pyclass::PyClassItems,
            }
            impl Pyo3MethodsInventoryForFriendGroup {
                pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                    Self { items }
                }
            }
            impl _pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForFriendGroup {
                fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                    &self.items
                }
            }
            impl ::inventory::Collect for Pyo3MethodsInventoryForFriendGroup {
                #[inline]
                fn registry() -> &'static ::inventory::Registry {
                    static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
                    &REGISTRY
                }
            }
        };
        impl From<Arc<libawr::client::friend_group::FriendGroup>> for FriendGroup {
            fn from(inner: Arc<libawr::client::friend_group::FriendGroup>) -> Self {
                Self { inner }
            }
        }
        impl FriendGroup {
            pub fn id(&self) -> u8 {
                crate::utils::PyPropertyConvert::<u8, u8>::convert(&self.inner.id)
            }
            pub fn name(&self) -> &str {
                crate::utils::PyPropertyConvert::<String, &str>::convert(&self.inner.name)
            }
            pub fn friend_count(&self) -> i32 {
                crate::utils::PyPropertyConvert::<i32, i32>::convert(&self.inner.friend_count)
            }
            pub fn online_count(&self) -> i32 {
                crate::utils::PyPropertyConvert::<i32, i32>::convert(&self.inner.online_count)
            }
            pub fn seq_id(&self) -> u8 {
                crate::utils::PyPropertyConvert::<u8, u8>::convert(&self.inner.seq_id)
            }
            fn __repr__(&self) -> String {
                {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["FriendGroup(", ")"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &[
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["id="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.id())],
                                    ));
                                    res
                                },
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["name="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.name())],
                                    ));
                                    res
                                },
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["friend_count="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.friend_count())],
                                    ));
                                    res
                                },
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["online_count="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.online_count())],
                                    ));
                                    res
                                },
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["seq_id="],
                                        &[::core::fmt::ArgumentV1::new_debug(&self.seq_id())],
                                    ));
                                    res
                                },
                            ]
                            .join(", "),
                        )],
                    ));
                    res
                }
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <FriendGroup as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory::new(_pyo3::impl_::pyclass::PyClassItems {
                                methods: &[
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "id\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                FriendGroup::__pymethod_get_id__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "name\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                FriendGroup::__pymethod_get_name__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "friend_count\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                FriendGroup::__pymethod_get_friend_count__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "online_count\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                FriendGroup::__pymethod_get_online_count__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                    _pyo3::class::PyMethodDefType::Getter({
                                        _pyo3::class::PyGetterDef::new(
                                            "seq_id\0",
                                            _pyo3::impl_::pymethods::PyGetter(
                                                FriendGroup::__pymethod_get_seq_id__,
                                            ),
                                            "\u{0}",
                                        )
                                    }),
                                ],
                                slots: &[_pyo3::ffi::PyType_Slot {
                                    slot: _pyo3::ffi::Py_tp_repr,
                                    pfunc: FriendGroup::__pymethod___repr____
                                        as _pyo3::ffi::reprfunc
                                        as _,
                                }],
                            })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendGroup {
                unsafe extern "C" fn __pymethod_get_id__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendGroup = &*_ref;
                            let item = FriendGroup::id(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_get_name__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendGroup = &*_ref;
                            let item = FriendGroup::name(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_get_friend_count__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendGroup = &*_ref;
                            let item = FriendGroup::friend_count(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_get_online_count__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendGroup = &*_ref;
                            let item = FriendGroup::online_count(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_get_seq_id__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _: *mut ::std::os::raw::c_void,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendGroup = &*_ref;
                            let item = FriendGroup::seq_id(_slf);
                            _pyo3::callback::convert(_py, item)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod___repr____(
                    _raw_slf: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let _slf = _raw_slf;
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendGroup = &*_ref;
                            _pyo3::callback::convert(_py, FriendGroup::__repr__(_slf))
                        }),
                    )
                }
            }
        };
        impl FriendGroup {
            pub fn to_selector(&self) -> FriendGroupSelector {
                self.inner.to_selector().into()
            }
            pub fn __getattr__(&self, py: Python, name: &str) -> PyResult<PyObject> {
                self.to_selector().into_py(py).getattr(py, name)
            }
            pub fn flush<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let mut inner = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    inner.flush().await;
                    Ok(self_)
                })
            }
            pub fn sync<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let mut inner = self_.borrow(py).inner.clone();
                py_future(py, async move { Ok(inner.sync().await?) })
            }
            pub fn flush_and_sync<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let mut inner = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    inner.flush_and_sync().await?;
                    Python::with_gil(|py| {
                        self_.borrow_mut(py).inner = inner;
                    });
                    Ok(self_)
                })
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            impl _pyo3::impl_::pyclass::PyClass__getattr__SlotFragment<FriendGroup>
                for _pyo3::impl_::pyclass::PyClassImplCollector<FriendGroup>
            {
                #[inline]
                unsafe fn __getattr__(
                    self,
                    _py: _pyo3::Python,
                    _raw_slf: *mut _pyo3::ffi::PyObject,
                    arg0: *mut _pyo3::ffi::PyObject,
                ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
                    impl FriendGroup {
                        unsafe fn __pymethod___getattr____(
                            _py: _pyo3::Python,
                            _raw_slf: *mut _pyo3::ffi::PyObject,
                            arg0: *mut _pyo3::ffi::PyObject,
                        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
                            let _slf = _raw_slf;
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendGroup = &*_ref;
                            _pyo3::callback::convert(
                                _py,
                                FriendGroup::__getattr__(
                                    _slf,
                                    _py,
                                    _pyo3::impl_::extract_argument::extract_argument(
                                        _py.from_borrowed_ptr::<_pyo3::PyAny>(arg0),
                                        &mut {
                                            _pyo3 :: impl_ :: extract_argument :: FunctionArgumentHolder :: INIT
                                        },
                                        "name",
                                    )?,
                                ),
                            )
                        }
                    }
                    FriendGroup::__pymethod___getattr____(_py, _raw_slf, arg0)
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <FriendGroup as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory :: new (_pyo3 :: impl_ :: pyclass :: PyClassItems { methods : & [_pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: noargs ("to_selector\0" , _pyo3 :: impl_ :: pymethods :: PyCFunction (FriendGroup :: __pymethod_to_selector__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendGroup :: __pymethod_flush__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("sync\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendGroup :: __pymethod_sync__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush_and_sync\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendGroup :: __pymethod_flush_and_sync__) , "\u{0}"))] , slots : & [{ unsafe extern "C" fn __wrap (_slf : * mut :: pyo3 :: ffi :: PyObject , attr : * mut :: pyo3 :: ffi :: PyObject) -> * mut :: pyo3 :: ffi :: PyObject { use :: std :: result :: Result :: * ; use :: pyo3 :: impl_ :: pyclass :: * ; let gil = :: pyo3 :: GILPool :: new () ; let py = gil . python () ; :: pyo3 :: callback :: panic_result_into_callback_output (py , :: std :: panic :: catch_unwind (move | | -> :: pyo3 :: PyResult < _ > { let collector = PyClassImplCollector :: < FriendGroup > :: new () ; match collector . __getattribute__ (py , _slf , attr) { Ok (obj) => Ok (obj) , Err (e) if e . is_instance_of :: < :: pyo3 :: exceptions :: PyAttributeError > (py) => { collector . __getattr__ (py , _slf , attr) } Err (e) => Err (e) , } })) } :: pyo3 :: ffi :: PyType_Slot { slot : :: pyo3 :: ffi :: Py_tp_getattro , pfunc : __wrap as :: pyo3 :: ffi :: getattrofunc as _ , } }] , })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendGroup {
                unsafe extern "C" fn __pymethod_to_selector__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendGroup = &*_ref;
                            let mut ret = FriendGroup::to_selector(_slf);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendGroup as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "flush",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendGroup::flush(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_sync__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendGroup as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "sync",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendGroup::sync(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush_and_sync__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroup>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                                _pyo3::impl_::extract_argument::FunctionDescription {
                                    cls_name: ::std::option::Option::Some(
                                        <FriendGroup as _pyo3::type_object::PyTypeInfo>::NAME,
                                    ),
                                    func_name: "flush_and_sync",
                                    positional_parameter_names: &[],
                                    positional_only_parameters: 0usize,
                                    required_positional_parameters: 0usize,
                                    keyword_only_parameters: &[],
                                };
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendGroup::flush_and_sync(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
            }
        };
        impl FriendGroup {}
        const _: () = {
            use :: pyo3 as _pyo3;
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory =
                                <FriendGroup as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                            Inventory::new(_pyo3::impl_::pyclass::PyClassItems {
                                methods: &[],
                                slots: &[],
                            })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendGroup {}
        };
        pub struct FriendGroupSelector {
            pub(crate) inner: libawr::client::friend_group::FriendGroupSelector,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for FriendGroupSelector {
            #[inline]
            fn clone(&self) -> FriendGroupSelector {
                FriendGroupSelector {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            unsafe impl _pyo3::type_object::PyTypeInfo for FriendGroupSelector {
                type AsRefTarget = _pyo3::PyCell<Self>;
                const NAME: &'static str = "FriendGroupSelector";
                const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
                #[inline]
                fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                    use _pyo3::type_object::LazyStaticType;
                    static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                    TYPE_OBJECT.get_or_init::<Self>(py)
                }
            }
            impl _pyo3::PyClass for FriendGroupSelector {
                type Frozen = _pyo3::pyclass::boolean_struct::False;
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
                for &'a FriendGroupSelector
            {
                type Holder = ::std::option::Option<_pyo3::PyRef<'py, FriendGroupSelector>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
                }
            }
            impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
                for &'a mut FriendGroupSelector
            {
                type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, FriendGroupSelector>>;
                #[inline]
                fn extract(
                    obj: &'py _pyo3::PyAny,
                    holder: &'a mut Self::Holder,
                ) -> _pyo3::PyResult<Self> {
                    _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
                }
            }
            impl _pyo3::IntoPy<_pyo3::PyObject> for FriendGroupSelector {
                fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                    _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
                }
            }
            impl _pyo3::impl_::pyclass::PyClassImpl for FriendGroupSelector {
                const DOC: &'static str = "\u{0}";
                const IS_BASETYPE: bool = false;
                const IS_SUBCLASS: bool = false;
                const IS_MAPPING: bool = false;
                const IS_SEQUENCE: bool = false;
                type Layout = _pyo3::PyCell<Self>;
                type BaseType = _pyo3::PyAny;
                type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<FriendGroupSelector>;
                type Inventory = Pyo3MethodsInventoryForFriendGroupSelector;
                type PyClassMutability = < < _pyo3 :: PyAny as _pyo3 :: impl_ :: pyclass :: PyClassBaseType > :: PyClassMutability as _pyo3 :: impl_ :: pycell :: PyClassMutability > :: MutableChild ;
                type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
                type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
                type BaseNativeType = _pyo3::PyAny;
                fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                    use _pyo3::impl_::pyclass::*;
                    let collector = PyClassImplCollector::<Self>::new();
                    static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                        methods: &[],
                        slots: &[],
                    };
                    PyClassItemsIter::new(
                        &INTRINSIC_ITEMS,
                        ::std::boxed::Box::new(::std::iter::Iterator::map(
                            _pyo3::inventory::iter::<
                                <Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory,
                            >(),
                            _pyo3::impl_::pyclass::PyClassInventory::items,
                        )),
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendGroupSelector {}
            #[doc(hidden)]
            pub struct Pyo3MethodsInventoryForFriendGroupSelector {
                items: _pyo3::impl_::pyclass::PyClassItems,
            }
            impl Pyo3MethodsInventoryForFriendGroupSelector {
                pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                    Self { items }
                }
            }
            impl _pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForFriendGroupSelector {
                fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                    &self.items
                }
            }
            impl ::inventory::Collect for Pyo3MethodsInventoryForFriendGroupSelector {
                #[inline]
                fn registry() -> &'static ::inventory::Registry {
                    static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
                    &REGISTRY
                }
            }
        };
        impl From<libawr::client::friend_group::FriendGroupSelector> for FriendGroupSelector {
            fn from(inner: libawr::client::friend_group::FriendGroupSelector) -> Self {
                Self { inner }
            }
        }
        impl FriendGroupSelector {
            pub fn fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                let selector = self.inner.clone();
                py_future(py, async move {
                    match selector.fetch().await? {
                        Some(friend) => Ok(Some(py_obj(FriendGroup::from(friend))?)),
                        None => Ok(None),
                    }
                })
            }
            pub fn flush<'py>(self_: Py<Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
                let selector = self_.borrow(py).inner.clone();
                py_future(py, async move {
                    selector.flush().await;
                    Ok(self_)
                })
            }
            pub fn flush_and_fetch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                let selector = self.inner.clone();
                py_future(py, async move {
                    match selector.flush_and_fetch().await? {
                        Some(friend) => Ok(Some(py_obj(FriendGroup::from(friend))?)),
                        None => Ok(None),
                    }
                })
            }
        }
        const _: () = {
            use :: pyo3 as _pyo3;
            const _: () = {
                #[allow(non_upper_case_globals)]
                extern "C" fn __init() {
                    static __INVENTORY: ::inventory::Node = ::inventory::Node {
                        value: &{
                            type Inventory = < FriendGroupSelector as _pyo3 :: impl_ :: pyclass :: PyClassImpl > :: Inventory ;
                            Inventory :: new (_pyo3 :: impl_ :: pyclass :: PyClassItems { methods : & [_pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("fetch\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendGroupSelector :: __pymethod_fetch__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendGroupSelector :: __pymethod_flush__) , "\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("flush_and_fetch\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (FriendGroupSelector :: __pymethod_flush_and_fetch__) , "\u{0}"))] , slots : & [] , })
                        },
                        next: ::inventory::core::cell::UnsafeCell::new(
                            ::inventory::core::option::Option::None,
                        ),
                    };
                    unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
                }
                #[used]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                #[link_section = ".init_array"]
                static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                    #[link_section = ".text.startup"]
                    unsafe extern "C" fn __init___rust_ctor___ctor() {
                        __init()
                    };
                    __init___rust_ctor___ctor
                };
            };
            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl FriendGroupSelector {
                unsafe extern "C" fn __pymethod_fetch__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroupSelector>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendGroupSelector = &*_ref;
                            const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< FriendGroupSelector as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "fetch" , positional_parameter_names : & [] , positional_only_parameters : 0usize , required_positional_parameters : 0usize , keyword_only_parameters : & [] , } ;
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendGroupSelector::fetch(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroupSelector>>()?;
                            #[allow(clippy::useless_conversion)]
                            let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                            const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< FriendGroupSelector as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "flush" , positional_parameter_names : & [] , positional_only_parameters : 0usize , required_positional_parameters : 0usize , keyword_only_parameters : & [] , } ;
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendGroupSelector::flush(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
                unsafe extern "C" fn __pymethod_flush_and_fetch__(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *mut _pyo3::ffi::PyObject,
                    _kwargs: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    let gil = _pyo3::GILPool::new();
                    let _py = gil.python();
                    _pyo3::callback::panic_result_into_callback_output(
                        _py,
                        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                            let _cell = _py
                                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                .downcast::<_pyo3::PyCell<FriendGroupSelector>>()?;
                            let _ref = _cell.try_borrow()?;
                            let _slf: &FriendGroupSelector = &*_ref;
                            const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< FriendGroupSelector as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "flush_and_fetch" , positional_parameter_names : & [] , positional_only_parameters : 0usize , required_positional_parameters : 0usize , keyword_only_parameters : & [] , } ;
                            let mut output = [::std::option::Option::None; 0usize];
                            let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                            let mut ret = FriendGroupSelector::flush_and_fetch(_slf, _py);
                            if false {
                                use _pyo3::impl_::ghost::IntoPyResult;
                                ret.assert_into_py_result();
                            }
                            _pyo3::callback::convert(_py, ret)
                        }),
                    )
                }
            }
        };
    }
    /// 客户端。
    pub struct Client {
        pub(crate) inner: Arc<libawr::client::Client>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Client {
        #[inline]
        fn clone(&self) -> Client {
            Client {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    const _: () = {
        use :: pyo3 as _pyo3;
        unsafe impl _pyo3::type_object::PyTypeInfo for Client {
            type AsRefTarget = _pyo3::PyCell<Self>;
            const NAME: &'static str = "Client";
            const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
            #[inline]
            fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                use _pyo3::type_object::LazyStaticType;
                static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                TYPE_OBJECT.get_or_init::<Self>(py)
            }
        }
        impl _pyo3::PyClass for Client {
            type Frozen = _pyo3::pyclass::boolean_struct::False;
        }
        impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a Client {
            type Holder = ::std::option::Option<_pyo3::PyRef<'py, Client>>;
            #[inline]
            fn extract(
                obj: &'py _pyo3::PyAny,
                holder: &'a mut Self::Holder,
            ) -> _pyo3::PyResult<Self> {
                _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
            }
        }
        impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a mut Client {
            type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, Client>>;
            #[inline]
            fn extract(
                obj: &'py _pyo3::PyAny,
                holder: &'a mut Self::Holder,
            ) -> _pyo3::PyResult<Self> {
                _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
            }
        }
        impl _pyo3::IntoPy<_pyo3::PyObject> for Client {
            fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
            }
        }
        impl _pyo3::impl_::pyclass::PyClassImpl for Client {
            const DOC: &'static str = "\u{5ba2}\u{6237}\u{7aef}\u{3002}\u{0}";
            const IS_BASETYPE: bool = false;
            const IS_SUBCLASS: bool = false;
            const IS_MAPPING: bool = false;
            const IS_SEQUENCE: bool = false;
            type Layout = _pyo3::PyCell<Self>;
            type BaseType = _pyo3::PyAny;
            type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<Client>;
            type Inventory = Pyo3MethodsInventoryForClient;
            type PyClassMutability = < < _pyo3 :: PyAny as _pyo3 :: impl_ :: pyclass :: PyClassBaseType > :: PyClassMutability as _pyo3 :: impl_ :: pycell :: PyClassMutability > :: MutableChild ;
            type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
            type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
            type BaseNativeType = _pyo3::PyAny;
            fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                use _pyo3::impl_::pyclass::*;
                let collector = PyClassImplCollector::<Self>::new();
                static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                    methods: &[],
                    slots: &[],
                };
                PyClassItemsIter::new(
                    &INTRINSIC_ITEMS,
                    ::std::boxed::Box::new(::std::iter::Iterator::map(
                        _pyo3::inventory::iter::<
                            <Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory,
                        >(),
                        _pyo3::impl_::pyclass::PyClassInventory::items,
                    )),
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl Client {}
        #[doc(hidden)]
        pub struct Pyo3MethodsInventoryForClient {
            items: _pyo3::impl_::pyclass::PyClassItems,
        }
        impl Pyo3MethodsInventoryForClient {
            pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                Self { items }
            }
        }
        impl _pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForClient {
            fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                &self.items
            }
        }
        impl ::inventory::Collect for Pyo3MethodsInventoryForClient {
            #[inline]
            fn registry() -> &'static ::inventory::Registry {
                static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
                &REGISTRY
            }
        }
    };
    impl From<libawr::client::Client> for Client {
        fn from(inner: libawr::client::Client) -> Self {
            Self {
                inner: Arc::new(inner),
            }
        }
    }
    impl Client {
        /// 当前账号的 QQ 号。
        ///
        /// # Python
        /// ```python
        /// @property
        /// def uin(self) -> int: ...
        /// ```
        pub fn uin(&self) -> i64 {
            self.inner.uin()
        }
        /// 当前账号是否在线。
        ///
        /// # Python
        /// ```python
        /// @property
        /// def is_online(self) -> bool: ...
        /// ```
        pub fn is_online(&self) -> bool {
            self.inner.is_online()
        }
        /// 构造好友选择器。
        ///
        /// # Python
        /// ```python
        /// def friend(self, uin: int) -> FriendSelector: ...
        /// ```
        pub fn friend(&self, uin: i64) -> FriendSelector {
            self.inner.friend(uin).into()
        }
    }
    const _: () = {
        use :: pyo3 as _pyo3;
        const _: () = {
            #[allow(non_upper_case_globals)]
            extern "C" fn __init() {
                static __INVENTORY: ::inventory::Node = ::inventory::Node {
                    value: &{
                        type Inventory = <Client as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                        Inventory :: new (_pyo3 :: impl_ :: pyclass :: PyClassItems { methods : & [_pyo3 :: class :: PyMethodDefType :: Getter ({ _pyo3 :: class :: PyGetterDef :: new ("uin\0" , _pyo3 :: impl_ :: pymethods :: PyGetter (Client :: __pymethod_get_uin__) , "\u{5f53}\u{524d}\u{8d26}\u{53f7}\u{7684} QQ \u{53f7}\u{3002}\n\n# Python\n```python\n@property\ndef uin(self) -> int: ...\n```\u{0}") }) , _pyo3 :: class :: PyMethodDefType :: Getter ({ _pyo3 :: class :: PyGetterDef :: new ("is_online\0" , _pyo3 :: impl_ :: pymethods :: PyGetter (Client :: __pymethod_get_is_online__) , "\u{5f53}\u{524d}\u{8d26}\u{53f7}\u{662f}\u{5426}\u{5728}\u{7ebf}\u{3002}\n\n# Python\n```python\n@property\ndef is_online(self) -> bool: ...\n```\u{0}") }) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("friend\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (Client :: __pymethod_friend__) , "\u{6784}\u{9020}\u{597d}\u{53cb}\u{9009}\u{62e9}\u{5668}\u{3002}\n\n# Python\n```python\ndef friend(self, uin: int) -> FriendSelector: ...\n```\u{0}"))] , slots : & [] , })
                    },
                    next: ::inventory::core::cell::UnsafeCell::new(
                        ::inventory::core::option::Option::None,
                    ),
                };
                unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
            }
            #[used]
            #[allow(non_upper_case_globals)]
            #[doc(hidden)]
            #[link_section = ".init_array"]
            static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                #[link_section = ".text.startup"]
                unsafe extern "C" fn __init___rust_ctor___ctor() {
                    __init()
                };
                __init___rust_ctor___ctor
            };
        };
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl Client {
            unsafe extern "C" fn __pymethod_get_uin__(
                _slf: *mut _pyo3::ffi::PyObject,
                _: *mut ::std::os::raw::c_void,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<Client>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &Client = &*_ref;
                        let item = Client::uin(_slf);
                        _pyo3::callback::convert(_py, item)
                    }),
                )
            }
            unsafe extern "C" fn __pymethod_get_is_online__(
                _slf: *mut _pyo3::ffi::PyObject,
                _: *mut ::std::os::raw::c_void,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<Client>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &Client = &*_ref;
                        let item = Client::is_online(_slf);
                        _pyo3::callback::convert(_py, item)
                    }),
                )
            }
            unsafe extern "C" fn __pymethod_friend__(
                _slf: *mut _pyo3::ffi::PyObject,
                _args: *mut _pyo3::ffi::PyObject,
                _kwargs: *mut _pyo3::ffi::PyObject,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<Client>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &Client = &*_ref;
                        const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                            _pyo3::impl_::extract_argument::FunctionDescription {
                                cls_name: ::std::option::Option::Some(
                                    <Client as _pyo3::type_object::PyTypeInfo>::NAME,
                                ),
                                func_name: "friend",
                                positional_parameter_names: &["uin"],
                                positional_only_parameters: 0usize,
                                required_positional_parameters: 1usize,
                                keyword_only_parameters: &[],
                            };
                        let mut output = [::std::option::Option::None; 1usize];
                        let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                        let mut ret = Client::friend(
                            _slf,
                            _pyo3::impl_::extract_argument::extract_argument(
                                _pyo3::impl_::extract_argument::unwrap_required_argument(
                                    output[0usize],
                                ),
                                &mut {
                                    _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT
                                },
                                "uin",
                            )?,
                        );
                        if false {
                            use _pyo3::impl_::ghost::IntoPyResult;
                            ret.assert_into_py_result();
                        }
                        _pyo3::callback::convert(_py, ret)
                    }),
                )
            }
        }
    };
}
pub mod login {
    //! 账号登录。
    use std::{error::Error, sync::Arc};
    use anyhow::Context;
    use libawr::login::Credential;
    use pyo3::{
        exceptions::{PyRuntimeError, PyTypeError},
        prelude::*,
    };
    use ricq::Protocol;
    use tokio::sync::Mutex;
    use crate::{client::Client, utils::py_future};
    /// 登录
    pub fn login<'py>(
        py: Python<'py>,
        uin: i64,
        password: Option<String>,
        password_md5: Option<&str>,
        show_qrcode: Option<Py<PyAny>>,
        protocol: Option<&str>,
        data_folder: String,
    ) -> PyResult<&'py PyAny> {
        let credential = if let Some(password) = password {
            let protocol = protocol.and_then(parse_protocol).unwrap_or(Protocol::IPad);
            Credential::Password { password, protocol }
        } else if let Some(password_md5) = password_md5 {
            let protocol = protocol.and_then(parse_protocol).unwrap_or(Protocol::IPad);
            let password_md5 = hex::decode(password_md5).with_context(|| {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["\u{8bfb}\u{53d6} MD5 \u{503c} ", " \u{5931}\u{8d25}"],
                    &[::core::fmt::ArgumentV1::new_display(&password_md5)],
                ));
                res
            })?;
            Credential::MD5 {
                password_md5,
                protocol,
            }
        } else {
            let show_qrcode = show_qrcode.ok_or_else(|| PyTypeError::new_err("参数错误"))?;
            Credential::QrCode {
                show_qrcode: Box::new(move |img| {
                    Python::with_gil(|py| -> Result<(), Box<dyn Error + Send + Sync>> {
                        show_qrcode.as_ref(py).call1((Vec::from(img),))?;
                        Ok(())
                    })
                }),
            }
        };
        py_future(py, async move {
            let (client, alive_handle) = libawr::login(uin, credential, data_folder).await?;
            let client = Client { inner: client };
            let alive_handle = AliveHandle {
                inner: Arc::new(Mutex::new(alive_handle)),
            };
            Ok((client, alive_handle))
        })
    }
    #[doc(hidden)]
    pub mod login {
        pub(crate) struct MakeDef;
        pub const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = MakeDef::DEF;
    }
    const _: () = {
        use :: pyo3 as _pyo3;
        impl login::MakeDef {
            const DEF: ::pyo3::impl_::pyfunction::PyMethodDef =
                _pyo3::impl_::pymethods::PyMethodDef::cfunction_with_keywords(
                    "login\0",
                    _pyo3::impl_::pymethods::PyCFunctionWithKeywords(__pyfunction_login),
                    "\u{767b}\u{5f55}\u{0}",
                );
        }
        unsafe extern "C" fn __pyfunction_login(
            _slf: *mut _pyo3::ffi::PyObject,
            _args: *mut _pyo3::ffi::PyObject,
            _kwargs: *mut _pyo3::ffi::PyObject,
        ) -> *mut _pyo3::ffi::PyObject {
            let gil = _pyo3::GILPool::new();
            let _py = gil.python();
            _pyo3::callback::panic_result_into_callback_output(
                _py,
                ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                    const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                        _pyo3::impl_::extract_argument::FunctionDescription {
                            cls_name: ::std::option::Option::None,
                            func_name: "login",
                            positional_parameter_names: &["uin"],
                            positional_only_parameters: 0usize,
                            required_positional_parameters: 1usize,
                            keyword_only_parameters: &[
                                _pyo3::impl_::extract_argument::KeywordOnlyParameterDescription {
                                    name: "password",
                                    required: false,
                                },
                                _pyo3::impl_::extract_argument::KeywordOnlyParameterDescription {
                                    name: "password_md5",
                                    required: false,
                                },
                                _pyo3::impl_::extract_argument::KeywordOnlyParameterDescription {
                                    name: "show_qrcode",
                                    required: false,
                                },
                                _pyo3::impl_::extract_argument::KeywordOnlyParameterDescription {
                                    name: "protocol",
                                    required: false,
                                },
                                _pyo3::impl_::extract_argument::KeywordOnlyParameterDescription {
                                    name: "data_folder",
                                    required: false,
                                },
                            ],
                        };
                    let mut output = [::std::option::Option::None; 6usize];
                    let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                    let mut ret = login(
                        _py,
                        _pyo3::impl_::extract_argument::extract_argument(
                            _pyo3::impl_::extract_argument::unwrap_required_argument(
                                output[0usize],
                            ),
                            &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                            "uin",
                        )?,
                        _pyo3::impl_::extract_argument::extract_optional_argument(
                            output[1usize],
                            &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                            "password",
                            || ::std::option::Option::None,
                        )?,
                        _pyo3::impl_::extract_argument::extract_optional_argument(
                            output[2usize],
                            &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                            "password_md5",
                            || ::std::option::Option::None,
                        )?,
                        _pyo3::impl_::extract_argument::extract_optional_argument(
                            output[3usize],
                            &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                            "show_qrcode",
                            || ::std::option::Option::None,
                        )?,
                        _pyo3::impl_::extract_argument::extract_optional_argument(
                            output[4usize],
                            &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                            "protocol",
                            || ::std::option::Option::Some("IPad"),
                        )?,
                        _pyo3::impl_::extract_argument::extract_argument_with_default(
                            output[5usize],
                            &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                            "data_folder",
                            || "./bots".to_string(),
                        )?,
                    );
                    if false {
                        use _pyo3::impl_::ghost::IntoPyResult;
                        ret.assert_into_py_result();
                    }
                    _pyo3::callback::convert(_py, ret)
                }),
            )
        }
    };
    fn parse_protocol(protocol: &str) -> Option<Protocol> {
        match protocol.to_ascii_lowercase().as_str() {
            "ipad" => Some(Protocol::IPad),
            "android_phone" => Some(Protocol::AndroidPhone),
            "android_watch" => Some(Protocol::AndroidWatch),
            "macos" => Some(Protocol::MacOS),
            "qidian" => Some(Protocol::QiDian),
            _ => None,
        }
    }
    /// 登录保持。
    pub struct AliveHandle {
        inner: Arc<Mutex<libawr::login::AliveHandle>>,
    }
    const _: () = {
        use :: pyo3 as _pyo3;
        unsafe impl _pyo3::type_object::PyTypeInfo for AliveHandle {
            type AsRefTarget = _pyo3::PyCell<Self>;
            const NAME: &'static str = "AliveHandle";
            const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
            #[inline]
            fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                use _pyo3::type_object::LazyStaticType;
                static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                TYPE_OBJECT.get_or_init::<Self>(py)
            }
        }
        impl _pyo3::PyClass for AliveHandle {
            type Frozen = _pyo3::pyclass::boolean_struct::False;
        }
        impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a AliveHandle {
            type Holder = ::std::option::Option<_pyo3::PyRef<'py, AliveHandle>>;
            #[inline]
            fn extract(
                obj: &'py _pyo3::PyAny,
                holder: &'a mut Self::Holder,
            ) -> _pyo3::PyResult<Self> {
                _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
            }
        }
        impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a mut AliveHandle {
            type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, AliveHandle>>;
            #[inline]
            fn extract(
                obj: &'py _pyo3::PyAny,
                holder: &'a mut Self::Holder,
            ) -> _pyo3::PyResult<Self> {
                _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
            }
        }
        impl _pyo3::IntoPy<_pyo3::PyObject> for AliveHandle {
            fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
            }
        }
        impl _pyo3::impl_::pyclass::PyClassImpl for AliveHandle {
            const DOC: &'static str = "\u{767b}\u{5f55}\u{4fdd}\u{6301}\u{3002}\u{0}";
            const IS_BASETYPE: bool = false;
            const IS_SUBCLASS: bool = false;
            const IS_MAPPING: bool = false;
            const IS_SEQUENCE: bool = false;
            type Layout = _pyo3::PyCell<Self>;
            type BaseType = _pyo3::PyAny;
            type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<AliveHandle>;
            type Inventory = Pyo3MethodsInventoryForAliveHandle;
            type PyClassMutability = < < _pyo3 :: PyAny as _pyo3 :: impl_ :: pyclass :: PyClassBaseType > :: PyClassMutability as _pyo3 :: impl_ :: pycell :: PyClassMutability > :: MutableChild ;
            type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
            type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
            type BaseNativeType = _pyo3::PyAny;
            fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                use _pyo3::impl_::pyclass::*;
                let collector = PyClassImplCollector::<Self>::new();
                static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                    methods: &[],
                    slots: &[],
                };
                PyClassItemsIter::new(
                    &INTRINSIC_ITEMS,
                    ::std::boxed::Box::new(::std::iter::Iterator::map(
                        _pyo3::inventory::iter::<
                            <Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory,
                        >(),
                        _pyo3::impl_::pyclass::PyClassInventory::items,
                    )),
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl AliveHandle {}
        #[doc(hidden)]
        pub struct Pyo3MethodsInventoryForAliveHandle {
            items: _pyo3::impl_::pyclass::PyClassItems,
        }
        impl Pyo3MethodsInventoryForAliveHandle {
            pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                Self { items }
            }
        }
        impl _pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForAliveHandle {
            fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                &self.items
            }
        }
        impl ::inventory::Collect for Pyo3MethodsInventoryForAliveHandle {
            #[inline]
            fn registry() -> &'static ::inventory::Registry {
                static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
                &REGISTRY
            }
        }
    };
    impl AliveHandle {
        /// 等待，直到连接断开。
        pub fn alive<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
            let inner = self.inner.clone();
            py_future(py, async move {
                inner
                    .try_lock()
                    .map_err(|_| PyRuntimeError::new_err("其他线程正在等待连接断开"))?
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
                    .reconnect()
                    .await?;
                Ok(())
            })
        }
        /// 开始自动断线重连。
        pub fn auto_reconnect<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
            let inner = self.inner.clone();
            py_future(py, async move {
                inner
                    .try_lock()
                    .map_err(|_| PyRuntimeError::new_err("其他线程正在等待连接断开"))?
                    .auto_reconnect()
                    .await?;
                Ok(())
            })
        }
    }
    const _: () = {
        use :: pyo3 as _pyo3;
        const _: () = {
            #[allow(non_upper_case_globals)]
            extern "C" fn __init() {
                static __INVENTORY: ::inventory::Node = ::inventory::Node {
                    value: &{
                        type Inventory =
                            <AliveHandle as _pyo3::impl_::pyclass::PyClassImpl>::Inventory;
                        Inventory :: new (_pyo3 :: impl_ :: pyclass :: PyClassItems { methods : & [_pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("alive\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (AliveHandle :: __pymethod_alive__) , "\u{7b49}\u{5f85}\u{ff0c}\u{76f4}\u{5230}\u{8fde}\u{63a5}\u{65ad}\u{5f00}\u{3002}\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("reconnect\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (AliveHandle :: __pymethod_reconnect__) , "\u{65ad}\u{7ebf}\u{91cd}\u{8fde}\u{3002}\u{0}")) , _pyo3 :: class :: PyMethodDefType :: Method (_pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("auto_reconnect\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (AliveHandle :: __pymethod_auto_reconnect__) , "\u{5f00}\u{59cb}\u{81ea}\u{52a8}\u{65ad}\u{7ebf}\u{91cd}\u{8fde}\u{3002}\u{0}"))] , slots : & [] , })
                    },
                    next: ::inventory::core::cell::UnsafeCell::new(
                        ::inventory::core::option::Option::None,
                    ),
                };
                unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
            }
            #[used]
            #[allow(non_upper_case_globals)]
            #[doc(hidden)]
            #[link_section = ".init_array"]
            static __init___rust_ctor___ctor: unsafe extern "C" fn() = {
                #[link_section = ".text.startup"]
                unsafe extern "C" fn __init___rust_ctor___ctor() {
                    __init()
                };
                __init___rust_ctor___ctor
            };
        };
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl AliveHandle {
            unsafe extern "C" fn __pymethod_alive__(
                _slf: *mut _pyo3::ffi::PyObject,
                _args: *mut _pyo3::ffi::PyObject,
                _kwargs: *mut _pyo3::ffi::PyObject,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<AliveHandle>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &AliveHandle = &*_ref;
                        const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                            _pyo3::impl_::extract_argument::FunctionDescription {
                                cls_name: ::std::option::Option::Some(
                                    <AliveHandle as _pyo3::type_object::PyTypeInfo>::NAME,
                                ),
                                func_name: "alive",
                                positional_parameter_names: &[],
                                positional_only_parameters: 0usize,
                                required_positional_parameters: 0usize,
                                keyword_only_parameters: &[],
                            };
                        let mut output = [::std::option::Option::None; 0usize];
                        let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                        let mut ret = AliveHandle::alive(_slf, _py);
                        if false {
                            use _pyo3::impl_::ghost::IntoPyResult;
                            ret.assert_into_py_result();
                        }
                        _pyo3::callback::convert(_py, ret)
                    }),
                )
            }
            unsafe extern "C" fn __pymethod_reconnect__(
                _slf: *mut _pyo3::ffi::PyObject,
                _args: *mut _pyo3::ffi::PyObject,
                _kwargs: *mut _pyo3::ffi::PyObject,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<AliveHandle>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &AliveHandle = &*_ref;
                        const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                            _pyo3::impl_::extract_argument::FunctionDescription {
                                cls_name: ::std::option::Option::Some(
                                    <AliveHandle as _pyo3::type_object::PyTypeInfo>::NAME,
                                ),
                                func_name: "reconnect",
                                positional_parameter_names: &[],
                                positional_only_parameters: 0usize,
                                required_positional_parameters: 0usize,
                                keyword_only_parameters: &[],
                            };
                        let mut output = [::std::option::Option::None; 0usize];
                        let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                        let mut ret = AliveHandle::reconnect(_slf, _py);
                        if false {
                            use _pyo3::impl_::ghost::IntoPyResult;
                            ret.assert_into_py_result();
                        }
                        _pyo3::callback::convert(_py, ret)
                    }),
                )
            }
            unsafe extern "C" fn __pymethod_auto_reconnect__(
                _slf: *mut _pyo3::ffi::PyObject,
                _args: *mut _pyo3::ffi::PyObject,
                _kwargs: *mut _pyo3::ffi::PyObject,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<AliveHandle>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &AliveHandle = &*_ref;
                        const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                            _pyo3::impl_::extract_argument::FunctionDescription {
                                cls_name: ::std::option::Option::Some(
                                    <AliveHandle as _pyo3::type_object::PyTypeInfo>::NAME,
                                ),
                                func_name: "auto_reconnect",
                                positional_parameter_names: &[],
                                positional_only_parameters: 0usize,
                                required_positional_parameters: 0usize,
                                keyword_only_parameters: &[],
                            };
                        let mut output = [::std::option::Option::None; 0usize];
                        let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                        let mut ret = AliveHandle::auto_reconnect(_slf, _py);
                        if false {
                            use _pyo3::impl_::ghost::IntoPyResult;
                            ret.assert_into_py_result();
                        }
                        _pyo3::callback::convert(_py, ret)
                    }),
                )
            }
        }
    };
}
mod loguru {
    //! [`tracing`] 与 Python 的 Loguru 的桥接模块。
    use anyhow::{anyhow, Result};
    use pyo3::{intern, once_cell::GILOnceCell, prelude::*, types::*};
    use std::{fmt::Write, sync::Arc};
    use tracing::Level;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};
    /// 初始化日志输出。
    pub(crate) fn init(module: &PyModule) -> PyResult<()> {
        let layer = LoguruLayer::new()?;
        tracing_subscriber::registry()
            .with(layer)
            .with(
                tracing_subscriber::filter::Targets::new()
                    .with_target("ricq", Level::DEBUG)
                    .with_target("password_login", Level::DEBUG)
                    .with_target("awr", Level::DEBUG),
            )
            .init();
        Python::with_gil(|py| -> PyResult<()> {
            let logger_module = py.import("loguru")?.getattr("_logger")?;
            logger_module.setattr("get_frame", module.getattr("_getframe")?)
        })?;
        Ok(())
    }
    /// 将 [`tracing`] 的输出桥接到 Python 的 Loguru 中。
    pub(crate) struct LoguruLayer {
        log_fn: PyObject,
    }
    impl LoguruLayer {
        /// 创建一个新的 LoguruLayer 对象。
        pub(crate) fn new() -> Result<Self, PyErr> {
            let log_fn = Python::with_gil(|py| -> PyResult<PyObject> {
                let loguru = py.import("loguru")?;
                let logger = loguru.getattr("logger")?;
                let log_fn = logger.getattr("log")?;
                Ok(log_fn.into())
            })?;
            Ok(LoguruLayer { log_fn })
        }
    }
    impl<S> Layer<S> for LoguruLayer
    where
        S: tracing::Subscriber,
    {
        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _ctx: tracing_subscriber::layer::Context<'_, S>,
        ) {
            Python::with_gil(|py| {
                if let Ok(mut frame) = LAST_RUST_FRAME
                    .get_or_init(py, || Arc::new(std::sync::RwLock::new(None)))
                    .write()
                {
                    *frame = FakePyFrame::new(
                        event
                            .metadata()
                            .module_path()
                            .unwrap_or_else(|| event.metadata().target()),
                        event.metadata().file().unwrap_or("<rust>"),
                        "",
                        event.metadata().line().unwrap_or(0),
                    )
                    .ok();
                }
            });
            let message = {
                let mut visiter = LoguruVisiter::new();
                event.record(&mut visiter);
                visiter.0
            };
            let level = match event.metadata().level().as_str() {
                "WARN" => "WARNING",
                s => s,
            };
            Python::with_gil(|py| {
                let level: Py<PyString> = level.into_py(py);
                let message: Py<PyAny> = message.into_py(py);
                let args = (level, message);
                self.log_fn.call(py, args, None).unwrap();
            });
        }
    }
    /// 遍历并格式化日志信息。
    struct LoguruVisiter(String);
    impl LoguruVisiter {
        /// 创建一个新的 LoguruVisiter 对象。
        pub fn new() -> Self {
            LoguruVisiter(String::new())
        }
    }
    impl tracing::field::Visit for LoguruVisiter {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            if field.name() == "message" {
                self.0.push_str(value);
            } else {
                self.0
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", "="],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&field.name()),
                            ::core::fmt::ArgumentV1::new_display(&value),
                        ],
                    ))
                    .unwrap();
            }
        }
        fn record_error(
            &mut self,
            field: &tracing::field::Field,
            value: &(dyn std::error::Error + 'static),
        ) {
            self.0
                .write_fmt(::core::fmt::Arguments::new_v1(
                    &["", "="],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&field.name()),
                        ::core::fmt::ArgumentV1::new_display(&value),
                    ],
                ))
                .unwrap();
        }
        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            if field.name() == "message" {
                self.0
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&value)],
                    ))
                    .unwrap();
            } else {
                self.0
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", "="],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&field.name()),
                            ::core::fmt::ArgumentV1::new_debug(&value),
                        ],
                    ))
                    .unwrap();
            }
        }
    }
    #[doc(hidden)]
    pub struct FakePyFrame {
        f_globals: Py<PyDict>,
        f_code: Py<FakePyCode>,
        f_lineno: u32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FakePyFrame {
        #[inline]
        fn clone(&self) -> FakePyFrame {
            FakePyFrame {
                f_globals: ::core::clone::Clone::clone(&self.f_globals),
                f_code: ::core::clone::Clone::clone(&self.f_code),
                f_lineno: ::core::clone::Clone::clone(&self.f_lineno),
            }
        }
    }
    const _: () = {
        use :: pyo3 as _pyo3;
        unsafe impl _pyo3::type_object::PyTypeInfo for FakePyFrame {
            type AsRefTarget = _pyo3::PyCell<Self>;
            const NAME: &'static str = "FakePyFrame";
            const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
            #[inline]
            fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                use _pyo3::type_object::LazyStaticType;
                static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                TYPE_OBJECT.get_or_init::<Self>(py)
            }
        }
        impl _pyo3::PyClass for FakePyFrame {
            type Frozen = _pyo3::pyclass::boolean_struct::False;
        }
        impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a FakePyFrame {
            type Holder = ::std::option::Option<_pyo3::PyRef<'py, FakePyFrame>>;
            #[inline]
            fn extract(
                obj: &'py _pyo3::PyAny,
                holder: &'a mut Self::Holder,
            ) -> _pyo3::PyResult<Self> {
                _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
            }
        }
        impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a mut FakePyFrame {
            type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, FakePyFrame>>;
            #[inline]
            fn extract(
                obj: &'py _pyo3::PyAny,
                holder: &'a mut Self::Holder,
            ) -> _pyo3::PyResult<Self> {
                _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
            }
        }
        impl _pyo3::IntoPy<_pyo3::PyObject> for FakePyFrame {
            fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
            }
        }
        impl _pyo3::impl_::pyclass::PyClassImpl for FakePyFrame {
            const DOC: &'static str = "\u{0}";
            const IS_BASETYPE: bool = false;
            const IS_SUBCLASS: bool = false;
            const IS_MAPPING: bool = false;
            const IS_SEQUENCE: bool = false;
            type Layout = _pyo3::PyCell<Self>;
            type BaseType = _pyo3::PyAny;
            type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<FakePyFrame>;
            type Inventory = Pyo3MethodsInventoryForFakePyFrame;
            type PyClassMutability = < < _pyo3 :: PyAny as _pyo3 :: impl_ :: pyclass :: PyClassBaseType > :: PyClassMutability as _pyo3 :: impl_ :: pycell :: PyClassMutability > :: MutableChild ;
            type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
            type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
            type BaseNativeType = _pyo3::PyAny;
            fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                use _pyo3::impl_::pyclass::*;
                let collector = PyClassImplCollector::<Self>::new();
                static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                    methods: &[
                        _pyo3::class::PyMethodDefType::Getter({
                            _pyo3::class::PyGetterDef::new(
                                "f_globals\0",
                                _pyo3::impl_::pymethods::PyGetter(
                                    FakePyFrame::__pymethod_get_f_globals__,
                                ),
                                "\u{0}",
                            )
                        }),
                        _pyo3::class::PyMethodDefType::Getter({
                            _pyo3::class::PyGetterDef::new(
                                "f_code\0",
                                _pyo3::impl_::pymethods::PyGetter(
                                    FakePyFrame::__pymethod_get_f_code__,
                                ),
                                "\u{0}",
                            )
                        }),
                        _pyo3::class::PyMethodDefType::Getter({
                            _pyo3::class::PyGetterDef::new(
                                "f_lineno\0",
                                _pyo3::impl_::pymethods::PyGetter(
                                    FakePyFrame::__pymethod_get_f_lineno__,
                                ),
                                "\u{0}",
                            )
                        }),
                    ],
                    slots: &[],
                };
                PyClassItemsIter::new(
                    &INTRINSIC_ITEMS,
                    ::std::boxed::Box::new(::std::iter::Iterator::map(
                        _pyo3::inventory::iter::<
                            <Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory,
                        >(),
                        _pyo3::impl_::pyclass::PyClassInventory::items,
                    )),
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl FakePyFrame {
            unsafe extern "C" fn __pymethod_get_f_globals__(
                _slf: *mut _pyo3::ffi::PyObject,
                _: *mut ::std::os::raw::c_void,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<FakePyFrame>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &FakePyFrame = &*_ref;
                        let item = ::std::clone::Clone::clone(&(_slf.f_globals));
                        let item: _pyo3::Py<_pyo3::PyAny> = _pyo3::IntoPy::into_py(item, _py);
                        ::std::result::Result::Ok(_pyo3::conversion::IntoPyPointer::into_ptr(item))
                    }),
                )
            }
            unsafe extern "C" fn __pymethod_get_f_code__(
                _slf: *mut _pyo3::ffi::PyObject,
                _: *mut ::std::os::raw::c_void,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<FakePyFrame>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &FakePyFrame = &*_ref;
                        let item = ::std::clone::Clone::clone(&(_slf.f_code));
                        let item: _pyo3::Py<_pyo3::PyAny> = _pyo3::IntoPy::into_py(item, _py);
                        ::std::result::Result::Ok(_pyo3::conversion::IntoPyPointer::into_ptr(item))
                    }),
                )
            }
            unsafe extern "C" fn __pymethod_get_f_lineno__(
                _slf: *mut _pyo3::ffi::PyObject,
                _: *mut ::std::os::raw::c_void,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<FakePyFrame>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &FakePyFrame = &*_ref;
                        let item = ::std::clone::Clone::clone(&(_slf.f_lineno));
                        let item: _pyo3::Py<_pyo3::PyAny> = _pyo3::IntoPy::into_py(item, _py);
                        ::std::result::Result::Ok(_pyo3::conversion::IntoPyPointer::into_ptr(item))
                    }),
                )
            }
        }
        #[doc(hidden)]
        pub struct Pyo3MethodsInventoryForFakePyFrame {
            items: _pyo3::impl_::pyclass::PyClassItems,
        }
        impl Pyo3MethodsInventoryForFakePyFrame {
            pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                Self { items }
            }
        }
        impl _pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForFakePyFrame {
            fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                &self.items
            }
        }
        impl ::inventory::Collect for Pyo3MethodsInventoryForFakePyFrame {
            #[inline]
            fn registry() -> &'static ::inventory::Registry {
                static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
                &REGISTRY
            }
        }
    };
    #[doc(hidden)]
    pub struct FakePyCode {
        co_filename: Py<PyString>,
        co_name: Py<PyString>,
    }
    const _: () = {
        use :: pyo3 as _pyo3;
        unsafe impl _pyo3::type_object::PyTypeInfo for FakePyCode {
            type AsRefTarget = _pyo3::PyCell<Self>;
            const NAME: &'static str = "FakePyCode";
            const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
            #[inline]
            fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                use _pyo3::type_object::LazyStaticType;
                static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                TYPE_OBJECT.get_or_init::<Self>(py)
            }
        }
        impl _pyo3::PyClass for FakePyCode {
            type Frozen = _pyo3::pyclass::boolean_struct::False;
        }
        impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a FakePyCode {
            type Holder = ::std::option::Option<_pyo3::PyRef<'py, FakePyCode>>;
            #[inline]
            fn extract(
                obj: &'py _pyo3::PyAny,
                holder: &'a mut Self::Holder,
            ) -> _pyo3::PyResult<Self> {
                _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
            }
        }
        impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a mut FakePyCode {
            type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, FakePyCode>>;
            #[inline]
            fn extract(
                obj: &'py _pyo3::PyAny,
                holder: &'a mut Self::Holder,
            ) -> _pyo3::PyResult<Self> {
                _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
            }
        }
        impl _pyo3::IntoPy<_pyo3::PyObject> for FakePyCode {
            fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
            }
        }
        impl _pyo3::impl_::pyclass::PyClassImpl for FakePyCode {
            const DOC: &'static str = "\u{0}";
            const IS_BASETYPE: bool = false;
            const IS_SUBCLASS: bool = false;
            const IS_MAPPING: bool = false;
            const IS_SEQUENCE: bool = false;
            type Layout = _pyo3::PyCell<Self>;
            type BaseType = _pyo3::PyAny;
            type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<FakePyCode>;
            type Inventory = Pyo3MethodsInventoryForFakePyCode;
            type PyClassMutability = < < _pyo3 :: PyAny as _pyo3 :: impl_ :: pyclass :: PyClassBaseType > :: PyClassMutability as _pyo3 :: impl_ :: pycell :: PyClassMutability > :: MutableChild ;
            type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
            type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
            type BaseNativeType = _pyo3::PyAny;
            fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                use _pyo3::impl_::pyclass::*;
                let collector = PyClassImplCollector::<Self>::new();
                static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                    methods: &[
                        _pyo3::class::PyMethodDefType::Getter({
                            _pyo3::class::PyGetterDef::new(
                                "co_filename\0",
                                _pyo3::impl_::pymethods::PyGetter(
                                    FakePyCode::__pymethod_get_co_filename__,
                                ),
                                "\u{0}",
                            )
                        }),
                        _pyo3::class::PyMethodDefType::Getter({
                            _pyo3::class::PyGetterDef::new(
                                "co_name\0",
                                _pyo3::impl_::pymethods::PyGetter(
                                    FakePyCode::__pymethod_get_co_name__,
                                ),
                                "\u{0}",
                            )
                        }),
                    ],
                    slots: &[],
                };
                PyClassItemsIter::new(
                    &INTRINSIC_ITEMS,
                    ::std::boxed::Box::new(::std::iter::Iterator::map(
                        _pyo3::inventory::iter::<
                            <Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory,
                        >(),
                        _pyo3::impl_::pyclass::PyClassInventory::items,
                    )),
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl FakePyCode {
            unsafe extern "C" fn __pymethod_get_co_filename__(
                _slf: *mut _pyo3::ffi::PyObject,
                _: *mut ::std::os::raw::c_void,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<FakePyCode>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &FakePyCode = &*_ref;
                        let item = ::std::clone::Clone::clone(&(_slf.co_filename));
                        let item: _pyo3::Py<_pyo3::PyAny> = _pyo3::IntoPy::into_py(item, _py);
                        ::std::result::Result::Ok(_pyo3::conversion::IntoPyPointer::into_ptr(item))
                    }),
                )
            }
            unsafe extern "C" fn __pymethod_get_co_name__(
                _slf: *mut _pyo3::ffi::PyObject,
                _: *mut ::std::os::raw::c_void,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<FakePyCode>>()?;
                        let _ref = _cell.try_borrow()?;
                        let _slf: &FakePyCode = &*_ref;
                        let item = ::std::clone::Clone::clone(&(_slf.co_name));
                        let item: _pyo3::Py<_pyo3::PyAny> = _pyo3::IntoPy::into_py(item, _py);
                        ::std::result::Result::Ok(_pyo3::conversion::IntoPyPointer::into_ptr(item))
                    }),
                )
            }
        }
        #[doc(hidden)]
        pub struct Pyo3MethodsInventoryForFakePyCode {
            items: _pyo3::impl_::pyclass::PyClassItems,
        }
        impl Pyo3MethodsInventoryForFakePyCode {
            pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                Self { items }
            }
        }
        impl _pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForFakePyCode {
            fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                &self.items
            }
        }
        impl ::inventory::Collect for Pyo3MethodsInventoryForFakePyCode {
            #[inline]
            fn registry() -> &'static ::inventory::Registry {
                static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
                &REGISTRY
            }
        }
    };
    impl FakePyFrame {
        fn new(name: &str, file_path: &str, function: &str, line: u32) -> Result<FakePyFrame> {
            let f_globals = Python::with_gil(|py| {
                let name: Py<PyString> = name.into_py(py);
                [("__name__", name)].into_py_dict(py).into()
            });
            let f_code = Python::with_gil(|py| {
                Py::new(
                    py,
                    FakePyCode {
                        co_filename: PyString::new(py, file_path).into(),
                        co_name: PyString::new(py, function).into(),
                    },
                )
            })?;
            Ok(FakePyFrame {
                f_globals,
                f_code,
                f_lineno: line,
            })
        }
    }
    #[doc(hidden)]
    pub fn getframe(py: Python, depth: usize) -> PyResult<FakePyFrame> {
        let frames: &PyList = py
            .import("inspect")?
            .call_method("stack", (), None)?
            .extract()?;
        Ok(if frames.len() > depth {
            let frame_info = frames.get_item(depth)?;
            let name = frame_info
                .getattr({
                    static INTERNED: ::pyo3::once_cell::Interned =
                        ::pyo3::once_cell::Interned::new("frame");
                    INTERNED.get(py)
                })?
                .getattr({
                    static INTERNED: ::pyo3::once_cell::Interned =
                        ::pyo3::once_cell::Interned::new("f_globals");
                    INTERNED.get(py)
                })?
                .get_item({
                    static INTERNED: ::pyo3::once_cell::Interned =
                        ::pyo3::once_cell::Interned::new("__name__");
                    INTERNED.get(py)
                })?
                .extract()?;
            let file_path = frame_info
                .getattr({
                    static INTERNED: ::pyo3::once_cell::Interned =
                        ::pyo3::once_cell::Interned::new("filename");
                    INTERNED.get(py)
                })?
                .extract()?;
            let function = frame_info
                .getattr({
                    static INTERNED: ::pyo3::once_cell::Interned =
                        ::pyo3::once_cell::Interned::new("function");
                    INTERNED.get(py)
                })?
                .extract()?;
            let line = frame_info
                .getattr({
                    static INTERNED: ::pyo3::once_cell::Interned =
                        ::pyo3::once_cell::Interned::new("lineno");
                    INTERNED.get(py)
                })?
                .extract()?;
            FakePyFrame::new(name, file_path, function, line)?
        } else {
            let frame = LAST_RUST_FRAME
                .get_or_init(py, || Arc::new(std::sync::RwLock::new(None)))
                .read()
                .map(|frame| {
                    frame
                        .as_ref()
                        .map(|f| Ok(f.clone()))
                        .unwrap_or_else(|| FakePyFrame::new("<unknown>", "", "", 0))
                })
                .map_err(|e| {
                    ::anyhow::Error::msg({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&e)],
                        ));
                        res
                    })
                });
            frame??
        })
    }
    #[doc(hidden)]
    pub mod getframe {
        pub(crate) struct MakeDef;
        pub const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = MakeDef::DEF;
    }
    const _: () = {
        use :: pyo3 as _pyo3;
        impl getframe::MakeDef {
            const DEF: ::pyo3::impl_::pyfunction::PyMethodDef =
                _pyo3::impl_::pymethods::PyMethodDef::cfunction_with_keywords(
                    "_getframe\0",
                    _pyo3::impl_::pymethods::PyCFunctionWithKeywords(__pyfunction_getframe),
                    "\u{0}",
                );
        }
        unsafe extern "C" fn __pyfunction_getframe(
            _slf: *mut _pyo3::ffi::PyObject,
            _args: *mut _pyo3::ffi::PyObject,
            _kwargs: *mut _pyo3::ffi::PyObject,
        ) -> *mut _pyo3::ffi::PyObject {
            let gil = _pyo3::GILPool::new();
            let _py = gil.python();
            _pyo3::callback::panic_result_into_callback_output(
                _py,
                ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                    const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                        _pyo3::impl_::extract_argument::FunctionDescription {
                            cls_name: ::std::option::Option::None,
                            func_name: "_getframe",
                            positional_parameter_names: &["depth"],
                            positional_only_parameters: 0usize,
                            required_positional_parameters: 1usize,
                            keyword_only_parameters: &[],
                        };
                    let mut output = [::std::option::Option::None; 1usize];
                    let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                    let mut ret = getframe(
                        _py,
                        _pyo3::impl_::extract_argument::extract_argument(
                            _pyo3::impl_::extract_argument::unwrap_required_argument(
                                output[0usize],
                            ),
                            &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                            "depth",
                        )?,
                    );
                    if false {
                        use _pyo3::impl_::ghost::IntoPyResult;
                        ret.assert_into_py_result();
                    }
                    _pyo3::callback::convert(_py, ret)
                }),
            )
        }
    };
    /// 最后一次日志记录时的 rust 堆栈
    static LAST_RUST_FRAME: GILOnceCell<Arc<std::sync::RwLock<Option<FakePyFrame>>>> =
        GILOnceCell::new();
}
const LOGO: &str = r#"
 █████╗ ██╗    ██╗██████╗ 
██╔══██╗██║    ██║██╔══██╗
███████║██║ █╗ ██║██████╔╝
██╔══██║██║███╗██║██╔══██╗
██║  ██║╚███╔███╔╝██║  ██║
╚═╝  ╚═╝ ╚══╝╚══╝ ╚═╝  ╚═╝
"#;
/// 初始化 AWR 环境：
/// - 设置日志输出。
/// - 打印版本信息。
#[doc(hidden)]
pub fn init(module: &PyModule) -> PyResult<()> {
    loguru::init(module)?;
    {
        use ::tracing::__macro_support::Callsite as _;
        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "event src/lib.rs:63",
                    "awr",
                    ::tracing::Level::INFO,
                    Some("src/lib.rs"),
                    Some(63u32),
                    Some("awr"),
                    ::tracing_core::field::FieldSet::new(
                        &["message"],
                        ::tracing_core::callsite::Identifier(&CALLSITE),
                    ),
                    ::tracing::metadata::Kind::EVENT,
                )
            };
            ::tracing::callsite::DefaultCallsite::new(&META)
        };
        let enabled = ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            && {
                let interest = CALLSITE.interest();
                !interest.is_never()
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
            };
        if enabled {
            (|value_set: ::tracing::field::ValueSet| {
                let meta = CALLSITE.metadata();
                ::tracing::Event::dispatch(meta, &value_set);
            })({
                #[allow(unused_imports)]
                use ::tracing::field::{debug, display, Value};
                let mut iter = CALLSITE.metadata().fields().iter();
                CALLSITE.metadata().fields().value_set(&[(
                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                    Some(&::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&LOGO)],
                    ) as &dyn Value),
                )])
            });
        } else {
        }
    };
    Ok(())
}
#[doc(hidden)]
pub mod init {
    pub(crate) struct MakeDef;
    pub const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = MakeDef::DEF;
}
const _: () = {
    use :: pyo3 as _pyo3;
    impl init::MakeDef {
        const DEF : :: pyo3 :: impl_ :: pyfunction :: PyMethodDef = _pyo3 :: impl_ :: pymethods :: PyMethodDef :: cfunction_with_keywords ("init\0" , _pyo3 :: impl_ :: pymethods :: PyCFunctionWithKeywords (__pyfunction_init) , "\u{521d}\u{59cb}\u{5316} AWR \u{73af}\u{5883}\u{ff1a}\n- \u{8bbe}\u{7f6e}\u{65e5}\u{5fd7}\u{8f93}\u{51fa}\u{3002}\n- \u{6253}\u{5370}\u{7248}\u{672c}\u{4fe1}\u{606f}\u{3002}\u{0}") ;
    }
    unsafe extern "C" fn __pyfunction_init(
        _slf: *mut _pyo3::ffi::PyObject,
        _args: *mut _pyo3::ffi::PyObject,
        _kwargs: *mut _pyo3::ffi::PyObject,
    ) -> *mut _pyo3::ffi::PyObject {
        let gil = _pyo3::GILPool::new();
        let _py = gil.python();
        _pyo3::callback::panic_result_into_callback_output(
            _py,
            ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                    _pyo3::impl_::extract_argument::FunctionDescription {
                        cls_name: ::std::option::Option::None,
                        func_name: "init",
                        positional_parameter_names: &["module"],
                        positional_only_parameters: 0usize,
                        required_positional_parameters: 1usize,
                        keyword_only_parameters: &[],
                    };
                let mut output = [::std::option::Option::None; 1usize];
                let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                let mut ret = init(_pyo3::impl_::extract_argument::extract_argument(
                    _pyo3::impl_::extract_argument::unwrap_required_argument(output[0usize]),
                    &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                    "module",
                )?);
                if false {
                    use _pyo3::impl_::ghost::IntoPyResult;
                    ret.assert_into_py_result();
                }
                _pyo3::callback::convert(_py, ret)
            }),
        )
    }
};
/// 构建信息。
#[allow(dead_code)]
pub mod build {
    ///The Continuous Integration platform detected during compilation.
    #[allow(dead_code)]
    pub const CI_PLATFORM: Option<&str> = None;
    ///The full version.
    #[allow(dead_code)]
    pub const PKG_VERSION: &str = r"0.1.0";
    ///The major version.
    #[allow(dead_code)]
    pub const PKG_VERSION_MAJOR: &str = r"0";
    ///The minor version.
    #[allow(dead_code)]
    pub const PKG_VERSION_MINOR: &str = r"1";
    ///The patch version.
    #[allow(dead_code)]
    pub const PKG_VERSION_PATCH: &str = r"0";
    ///The pre-release version.
    #[allow(dead_code)]
    pub const PKG_VERSION_PRE: &str = r"";
    ///A colon-separated list of authors.
    #[allow(dead_code)]
    pub const PKG_AUTHORS: &str = r"\u{5fd8}\u{5fe7}\u{5317}\u{8431}\u{8349}<wybxc@qq.com>";
    ///The name of the package.
    #[allow(dead_code)]
    pub const PKG_NAME: &str = r"awr";
    ///The description.
    #[allow(dead_code)]
    pub const PKG_DESCRIPTION: &str = r"";
    ///The homepage.
    #[allow(dead_code)]
    pub const PKG_HOMEPAGE: &str = r"";
    ///The license.
    #[allow(dead_code)]
    pub const PKG_LICENSE: &str = r"AGPL-3.0";
    ///The source repository as advertised in Cargo.toml.
    #[allow(dead_code)]
    pub const PKG_REPOSITORY: &str = r"";
    ///The target triple that was being compiled for.
    #[allow(dead_code)]
    pub const TARGET: &str = r"x86_64-unknown-linux-gnu";
    ///The host triple of the rust compiler.
    #[allow(dead_code)]
    pub const HOST: &str = r"x86_64-unknown-linux-gnu";
    ///`release` for release builds, `debug` for other builds.
    #[allow(dead_code)]
    pub const PROFILE: &str = r"debug";
    ///The compiler that cargo resolved to use.
    #[allow(dead_code)]
    pub const RUSTC: &str = r"rustc";
    ///The documentation generator that cargo resolved to use.
    #[allow(dead_code)]
    pub const RUSTDOC: &str = r"rustdoc";
    ///Value of OPT_LEVEL for the profile used during compilation.
    #[allow(dead_code)]
    pub const OPT_LEVEL: &str = r"0";
    ///The parallelism that was specified during compilation.
    #[allow(dead_code)]
    pub const NUM_JOBS: u32 = 16;
    ///Value of DEBUG for the profile used during compilation.
    #[allow(dead_code)]
    pub const DEBUG: bool = true;
    ///The features that were enabled during compilation.
    #[allow(dead_code)]
    pub const FEATURES: [&str; 0] = [];
    ///The features as a comma-separated string.
    #[allow(dead_code)]
    pub const FEATURES_STR: &str = r"";
    ///The output of `rustc -V`
    #[allow(dead_code)]
    pub const RUSTC_VERSION: &str = r"rustc 1.66.0-nightly (bed4ad65b 2022-10-25)";
    ///The output of `rustdoc -V`
    #[allow(dead_code)]
    pub const RUSTDOC_VERSION: &str = r"rustdoc 1.66.0-nightly (bed4ad65b 2022-10-25)";
    ///An array of effective dependencies as documented by `Cargo.lock`.
    #[allow(dead_code)]
    pub const DEPENDENCIES: [(&str, &str); 181] = [
        ("adler", "1.0.2"),
        ("aho-corasick", "0.7.19"),
        ("android_system_properties", "0.1.5"),
        ("anyhow", "1.0.68"),
        ("async-trait", "0.1.61"),
        ("autocfg", "1.1.0"),
        ("awr", "0.1.0"),
        ("base16ct", "0.1.1"),
        ("bitflags", "1.3.2"),
        ("built", "0.5.2"),
        ("bumpalo", "3.11.1"),
        ("byteorder", "1.4.3"),
        ("bytes", "1.3.0"),
        ("cached", "0.35.0"),
        ("cargo-lock", "8.0.3"),
        ("cc", "1.0.76"),
        ("cfg-if", "1.0.0"),
        ("chrono", "0.4.23"),
        ("codespan-reporting", "0.11.1"),
        ("const-oid", "0.7.1"),
        ("const_panic", "0.2.7"),
        ("core-foundation-sys", "0.8.3"),
        ("crc32fast", "1.3.2"),
        ("crypto-bigint", "0.3.2"),
        ("ctor", "0.1.26"),
        ("cxx", "1.0.82"),
        ("cxx-build", "1.0.82"),
        ("cxxbridge-flags", "1.0.82"),
        ("cxxbridge-macro", "1.0.82"),
        ("der", "0.5.1"),
        ("derivative", "2.2.0"),
        ("either", "1.8.0"),
        ("elliptic-curve", "0.11.12"),
        ("fastrand", "1.8.0"),
        ("ff", "0.11.1"),
        ("fixedbitset", "0.4.2"),
        ("flate2", "1.0.24"),
        ("form_urlencoded", "1.1.0"),
        ("futures", "0.3.25"),
        ("futures-channel", "0.3.25"),
        ("futures-core", "0.3.25"),
        ("futures-executor", "0.3.25"),
        ("futures-io", "0.3.25"),
        ("futures-macro", "0.3.25"),
        ("futures-sink", "0.3.25"),
        ("futures-task", "0.3.25"),
        ("futures-util", "0.3.25"),
        ("generic-array", "0.14.6"),
        ("getrandom", "0.2.8"),
        ("ghost", "0.1.7"),
        ("group", "0.11.0"),
        ("hashbrown", "0.12.3"),
        ("heck", "0.3.3"),
        ("hermit-abi", "0.1.19"),
        ("hex", "0.4.3"),
        ("iana-time-zone", "0.1.53"),
        ("iana-time-zone-haiku", "0.1.1"),
        ("idna", "0.3.0"),
        ("indexmap", "1.9.2"),
        ("indoc", "1.0.7"),
        ("instant", "0.1.12"),
        ("inventory", "0.3.3"),
        ("itertools", "0.10.5"),
        ("itoa", "1.0.4"),
        ("jcers", "0.1.2"),
        ("jcers_proc", "0.1.0"),
        ("js-sys", "0.3.60"),
        ("konst", "0.3.4"),
        ("konst_kernel", "0.3.4"),
        ("konst_proc_macros", "0.3.0"),
        ("lazy_static", "1.4.0"),
        ("libawr", "0.1.0"),
        ("libc", "0.2.137"),
        ("link-cplusplus", "1.0.7"),
        ("lock_api", "0.4.9"),
        ("log", "0.4.17"),
        ("md5", "0.7.0"),
        ("memchr", "2.5.0"),
        ("memoffset", "0.6.5"),
        ("miniz_oxide", "0.5.4"),
        ("mio", "0.8.5"),
        ("multimap", "0.8.3"),
        ("nu-ansi-term", "0.46.0"),
        ("num-integer", "0.1.45"),
        ("num-traits", "0.2.15"),
        ("num_cpus", "1.14.0"),
        ("once_cell", "1.16.0"),
        ("overload", "0.1.1"),
        ("p256", "0.10.1"),
        ("parking_lot", "0.12.1"),
        ("parking_lot_core", "0.9.4"),
        ("percent-encoding", "2.2.0"),
        ("petgraph", "0.6.2"),
        ("pin-project-lite", "0.2.9"),
        ("pin-utils", "0.1.0"),
        ("ppv-lite86", "0.2.17"),
        ("proc-macro2", "1.0.47"),
        ("prost", "0.9.0"),
        ("prost-build", "0.9.0"),
        ("prost-derive", "0.9.0"),
        ("prost-types", "0.9.0"),
        ("pyo3", "0.17.3"),
        ("pyo3-asyncio", "0.17.0"),
        ("pyo3-build-config", "0.17.3"),
        ("pyo3-built", "0.4.7"),
        ("pyo3-ffi", "0.17.3"),
        ("pyo3-macros", "0.17.3"),
        ("pyo3-macros-backend", "0.17.3"),
        ("quote", "1.0.21"),
        ("rand", "0.8.5"),
        ("rand_chacha", "0.3.1"),
        ("rand_core", "0.6.4"),
        ("redox_syscall", "0.2.16"),
        ("regex", "1.7.0"),
        ("regex-syntax", "0.6.28"),
        ("remove_dir_all", "0.5.3"),
        ("ricq", "0.1.19"),
        ("ricq-core", "0.1.19"),
        ("ryu", "1.0.11"),
        ("scopeguard", "1.1.0"),
        ("scratch", "1.0.2"),
        ("sec1", "0.2.1"),
        ("semver", "1.0.14"),
        ("serde", "1.0.147"),
        ("serde_derive", "1.0.147"),
        ("serde_json", "1.0.91"),
        ("sharded-slab", "0.1.4"),
        ("signal-hook-registry", "1.4.0"),
        ("slab", "0.4.7"),
        ("smallvec", "1.10.0"),
        ("socket2", "0.4.7"),
        ("subtle", "2.4.1"),
        ("syn", "1.0.103"),
        ("target-lexicon", "0.12.5"),
        ("tempfile", "3.3.0"),
        ("termcolor", "1.1.3"),
        ("thiserror", "1.0.37"),
        ("thiserror-impl", "1.0.37"),
        ("thread_local", "1.1.4"),
        ("tinyvec", "1.6.0"),
        ("tinyvec_macros", "0.1.0"),
        ("tokio", "1.24.1"),
        ("tokio-macros", "1.8.0"),
        ("tokio-stream", "0.1.11"),
        ("tokio-util", "0.7.4"),
        ("toml", "0.5.9"),
        ("tracing", "0.1.37"),
        ("tracing-attributes", "0.1.23"),
        ("tracing-core", "0.1.30"),
        ("tracing-log", "0.1.3"),
        ("tracing-subscriber", "0.3.16"),
        ("typenum", "1.15.0"),
        ("unicode-bidi", "0.3.8"),
        ("unicode-ident", "1.0.5"),
        ("unicode-normalization", "0.1.22"),
        ("unicode-segmentation", "1.10.0"),
        ("unicode-width", "0.1.10"),
        ("unindent", "0.1.10"),
        ("url", "2.3.1"),
        ("valuable", "0.1.0"),
        ("version_check", "0.9.4"),
        ("wasi", "0.11.0+wasi-snapshot-preview1"),
        ("wasm-bindgen", "0.2.83"),
        ("wasm-bindgen-backend", "0.2.83"),
        ("wasm-bindgen-macro", "0.2.83"),
        ("wasm-bindgen-macro-support", "0.2.83"),
        ("wasm-bindgen-shared", "0.2.83"),
        ("which", "4.3.0"),
        ("winapi", "0.3.9"),
        ("winapi-i686-pc-windows-gnu", "0.4.0"),
        ("winapi-util", "0.1.5"),
        ("winapi-x86_64-pc-windows-gnu", "0.4.0"),
        ("windows-sys", "0.42.0"),
        ("windows_aarch64_gnullvm", "0.42.0"),
        ("windows_aarch64_msvc", "0.42.0"),
        ("windows_i686_gnu", "0.42.0"),
        ("windows_i686_msvc", "0.42.0"),
        ("windows_x86_64_gnu", "0.42.0"),
        ("windows_x86_64_gnullvm", "0.42.0"),
        ("windows_x86_64_msvc", "0.42.0"),
        ("zeroize", "1.5.7"),
    ];
    ///The effective dependencies as a comma-separated string.
    #[allow(dead_code)]
    pub const DEPENDENCIES_STR: &str = r"adler 1.0.2, aho-corasick 0.7.19, android_system_properties 0.1.5, anyhow 1.0.68, async-trait 0.1.61, autocfg 1.1.0, awr 0.1.0, base16ct 0.1.1, bitflags 1.3.2, built 0.5.2, bumpalo 3.11.1, byteorder 1.4.3, bytes 1.3.0, cached 0.35.0, cargo-lock 8.0.3, cc 1.0.76, cfg-if 1.0.0, chrono 0.4.23, codespan-reporting 0.11.1, const-oid 0.7.1, const_panic 0.2.7, core-foundation-sys 0.8.3, crc32fast 1.3.2, crypto-bigint 0.3.2, ctor 0.1.26, cxx 1.0.82, cxx-build 1.0.82, cxxbridge-flags 1.0.82, cxxbridge-macro 1.0.82, der 0.5.1, derivative 2.2.0, either 1.8.0, elliptic-curve 0.11.12, fastrand 1.8.0, ff 0.11.1, fixedbitset 0.4.2, flate2 1.0.24, form_urlencoded 1.1.0, futures 0.3.25, futures-channel 0.3.25, futures-core 0.3.25, futures-executor 0.3.25, futures-io 0.3.25, futures-macro 0.3.25, futures-sink 0.3.25, futures-task 0.3.25, futures-util 0.3.25, generic-array 0.14.6, getrandom 0.2.8, ghost 0.1.7, group 0.11.0, hashbrown 0.12.3, heck 0.3.3, hermit-abi 0.1.19, hex 0.4.3, iana-time-zone 0.1.53, iana-time-zone-haiku 0.1.1, idna 0.3.0, indexmap 1.9.2, indoc 1.0.7, instant 0.1.12, inventory 0.3.3, itertools 0.10.5, itoa 1.0.4, jcers 0.1.2, jcers_proc 0.1.0, js-sys 0.3.60, konst 0.3.4, konst_kernel 0.3.4, konst_proc_macros 0.3.0, lazy_static 1.4.0, libawr 0.1.0, libc 0.2.137, link-cplusplus 1.0.7, lock_api 0.4.9, log 0.4.17, md5 0.7.0, memchr 2.5.0, memoffset 0.6.5, miniz_oxide 0.5.4, mio 0.8.5, multimap 0.8.3, nu-ansi-term 0.46.0, num-integer 0.1.45, num-traits 0.2.15, num_cpus 1.14.0, once_cell 1.16.0, overload 0.1.1, p256 0.10.1, parking_lot 0.12.1, parking_lot_core 0.9.4, percent-encoding 2.2.0, petgraph 0.6.2, pin-project-lite 0.2.9, pin-utils 0.1.0, ppv-lite86 0.2.17, proc-macro2 1.0.47, prost 0.9.0, prost-build 0.9.0, prost-derive 0.9.0, prost-types 0.9.0, pyo3 0.17.3, pyo3-asyncio 0.17.0, pyo3-build-config 0.17.3, pyo3-built 0.4.7, pyo3-ffi 0.17.3, pyo3-macros 0.17.3, pyo3-macros-backend 0.17.3, quote 1.0.21, rand 0.8.5, rand_chacha 0.3.1, rand_core 0.6.4, redox_syscall 0.2.16, regex 1.7.0, regex-syntax 0.6.28, remove_dir_all 0.5.3, ricq 0.1.19, ricq-core 0.1.19, ryu 1.0.11, scopeguard 1.1.0, scratch 1.0.2, sec1 0.2.1, semver 1.0.14, serde 1.0.147, serde_derive 1.0.147, serde_json 1.0.91, sharded-slab 0.1.4, signal-hook-registry 1.4.0, slab 0.4.7, smallvec 1.10.0, socket2 0.4.7, subtle 2.4.1, syn 1.0.103, target-lexicon 0.12.5, tempfile 3.3.0, termcolor 1.1.3, thiserror 1.0.37, thiserror-impl 1.0.37, thread_local 1.1.4, tinyvec 1.6.0, tinyvec_macros 0.1.0, tokio 1.24.1, tokio-macros 1.8.0, tokio-stream 0.1.11, tokio-util 0.7.4, toml 0.5.9, tracing 0.1.37, tracing-attributes 0.1.23, tracing-core 0.1.30, tracing-log 0.1.3, tracing-subscriber 0.3.16, typenum 1.15.0, unicode-bidi 0.3.8, unicode-ident 1.0.5, unicode-normalization 0.1.22, unicode-segmentation 1.10.0, unicode-width 0.1.10, unindent 0.1.10, url 2.3.1, valuable 0.1.0, version_check 0.9.4, wasi 0.11.0+wasi-snapshot-preview1, wasm-bindgen 0.2.83, wasm-bindgen-backend 0.2.83, wasm-bindgen-macro 0.2.83, wasm-bindgen-macro-support 0.2.83, wasm-bindgen-shared 0.2.83, which 4.3.0, winapi 0.3.9, winapi-i686-pc-windows-gnu 0.4.0, winapi-util 0.1.5, winapi-x86_64-pc-windows-gnu 0.4.0, windows-sys 0.42.0, windows_aarch64_gnullvm 0.42.0, windows_aarch64_msvc 0.42.0, windows_i686_gnu 0.42.0, windows_i686_msvc 0.42.0, windows_x86_64_gnu 0.42.0, windows_x86_64_gnullvm 0.42.0, windows_x86_64_msvc 0.42.0, zeroize 1.5.7";
    ///The build time in RFC2822, UTC.
    #[allow(dead_code)]
    pub const BUILT_TIME_UTC: &str = r"Tue, 17 Jan 2023 11:30:01 +0000";
    ///The target architecture, given by `CARGO_CFG_TARGET_ARCH`.
    #[allow(dead_code)]
    pub const CFG_TARGET_ARCH: &str = r"x86_64";
    ///The endianness, given by `CARGO_CFG_TARGET_ENDIAN`.
    #[allow(dead_code)]
    pub const CFG_ENDIAN: &str = r"little";
    ///The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`.
    #[allow(dead_code)]
    pub const CFG_ENV: &str = r"gnu";
    ///The OS-family, given by `CARGO_CFG_TARGET_FAMILY`.
    #[allow(dead_code)]
    pub const CFG_FAMILY: &str = r"unix";
    ///The operating system, given by `CARGO_CFG_TARGET_OS`.
    #[allow(dead_code)]
    pub const CFG_OS: &str = r"linux";
    ///The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`.
    #[allow(dead_code)]
    pub const CFG_POINTER_WIDTH: &str = r"64";
}
#[doc(hidden)]
pub fn awr(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function({
        use init as wrapped_pyfunction;
        ::pyo3::impl_::pyfunction::wrap_pyfunction(&wrapped_pyfunction::DEF, m)
    }?)?;
    m.add("__version__", "0.1.0")?;
    m.add("__build__", {
        use pyo3::types::PyDict;
        use pyo3::types::PyString;
        let info = PyDict::new(py);
        let build = PyDict::new(py);
        build.set_item("rustc", build::RUSTC)?;
        build.set_item("rustc-version", build::RUSTC_VERSION)?;
        build.set_item("opt-level", build::OPT_LEVEL)?;
        build.set_item("debug", build::DEBUG)?;
        build.set_item("jobs", build::NUM_JOBS)?;
        info.set_item("build", build)?;
        let dt = py
            .import("email.utils")?
            .getattr("parsedate_to_datetime")?
            .call1((build::BUILT_TIME_UTC,))?;
        info.set_item("info-time", dt)?;
        let deps = PyDict::new(py);
        for (name, version) in build::DEPENDENCIES.iter() {
            deps.set_item(name, version)?;
        }
        info.set_item("dependencies", deps)?;
        let features = build::FEATURES
            .iter()
            .map(|feat| PyString::new(py, feat))
            .collect::<Vec<_>>();
        info.set_item("features", features)?;
        let host = PyDict::new(py);
        host.set_item("triple", build::HOST)?;
        info.set_item("host", host)?;
        let target = PyDict::new(py);
        target.set_item("arch", build::CFG_TARGET_ARCH)?;
        target.set_item("os", build::CFG_OS)?;
        target.set_item("family", build::CFG_FAMILY)?;
        target.set_item("env", build::CFG_ENV)?;
        target.set_item("triple", build::TARGET)?;
        target.set_item("endianness", build::CFG_ENDIAN)?;
        target.set_item("pointer-width", build::CFG_POINTER_WIDTH)?;
        target.set_item("profile", build::PROFILE)?;
        info.set_item("target", target)?;
        info
    })?;
    m.add_function({
        use loguru::getframe as wrapped_pyfunction;
        ::pyo3::impl_::pyfunction::wrap_pyfunction(&wrapped_pyfunction::DEF, m)
    }?)?;
    Ok(())
}
#[doc(hidden)]
pub mod awr {
    pub(crate) struct MakeDef;
    pub static DEF: ::pyo3::impl_::pymodule::ModuleDef = MakeDef::make_def();
    pub const NAME: &'static str = "awr\u{0}";
    /// This autogenerated function is called by the python interpreter when importing
    /// the module.
    #[export_name = "PyInit_awr"]
    pub unsafe extern "C" fn init() -> *mut ::pyo3::ffi::PyObject {
        DEF.module_init()
    }
}
const _: () = {
    use ::pyo3::impl_::pymodule as impl_;
    impl awr::MakeDef {
        const fn make_def() -> impl_::ModuleDef {
            const INITIALIZER: impl_::ModuleInitializer = impl_::ModuleInitializer(awr);
            unsafe { impl_::ModuleDef::new(awr::NAME, "\u{0}", INITIALIZER) }
        }
    }
};
