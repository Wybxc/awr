//! 消息元素。

use pyo3::{exceptions::PyTypeError, prelude::*, types::*};
use ricq_core::msg::elem;

macro_rules! type_check {
    ($($checker:ident, $value:expr),*$(,)?) => {
        $(fn $checker(obj: &PyAny) -> PyResult<()> {
            let elem_type: String = obj.get_item("type")?.extract()?;
            let elem_type = elem_type.to_lowercase();
            if elem_type != $value {
                return Err(PyTypeError::new_err(format!(
                    "expected type '{}', got '{}'",
                    $value, elem_type
                )));
            }
            Ok(())
        })*
    };
}

type_check! {
    check_text, "text",
    check_at, "at",
}

#[derive(FromPyObject)]
pub(crate) enum Element {
    Text(Text),
    At(At),
}

/// 文本。
///
/// # Python
/// ```python
/// class Text(TypedDict):
///     type: Literal["text"]
///     text: str
/// ```
pub struct Text {
    text: String,
}

impl FromPyObject<'_> for Text {
    fn extract(obj: &PyAny) -> PyResult<Self> {
        if obj.is_instance_of::<PyString>()? {
            let text = obj.extract()?;
            return Ok(Self { text });
        }
        if obj.is_instance_of::<PyDict>()? {
            check_text(obj)?;
            let text = obj.get_item("text")?.extract()?;
            return Ok(Self { text });
        }
        Err(PyTypeError::new_err("Text"))
    }
}

impl Text {
    pub(crate) fn into_elem(self) -> elem::Text {
        elem::Text::new(self.text)
    }
}

/// At。
///
/// # Python
/// ```python
/// class At(TypedDict):
///     type: Literal["at"]
///     target: int
/// ```
#[derive(FromPyObject)]
pub struct At {
    #[pyo3(from_py_with = "check_at")]
    _type: (),
    #[pyo3(item)]
    target: i64,
}

impl At {
    pub(crate) fn into_elem(self) -> elem::At {
        elem::At::new(self.target)
    }
}
