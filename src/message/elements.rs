//! 消息元素。

use pyo3::{exceptions::PyTypeError, prelude::*, types::*};
use ricq_core::msg::elem;

pub(crate) enum Element {
    Text(Text),
    At(At),
    Face(Face),
}

impl FromPyObject<'_> for Element {
    fn extract(obj: &PyAny) -> PyResult<Self> {
        if obj.is_instance_of::<PyString>()? {
            let text = obj.extract()?;
            return Ok(Self::Text(Text { text }));
        }
        let elem_type: String = obj.get_item("type")?.extract()?;
        let elem_type = elem_type.to_lowercase();
        match elem_type.as_str() {
            "text" => Ok(Element::Text(obj.extract()?)),
            "at" => Ok(Element::At(obj.extract()?)),
            "face" => Ok(Element::Face(obj.extract()?)),
            _ => Err(PyTypeError::new_err(format!(
                "unknown message element type '{elem_type}'"
            ))),
        }
    }
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
            let text = obj.get_item("text")?.extract()?;
            return Ok(Self { text });
        }
        let repr = obj.repr()?.to_str()?;
        Err(PyTypeError::new_err(format!(
            "expected str or Text, got {repr}"
        )))
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
pub struct At {
    target: i64,
}

impl FromPyObject<'_> for At {
    fn extract(obj: &PyAny) -> PyResult<Self> {
        let target = obj.get_item("target")?.extract()?;
        Ok(Self { target })
    }
}

impl At {
    pub(crate) fn into_elem(self) -> elem::At {
        elem::At::new(self.target)
    }
}

/// Face。
///
/// # Python
/// ```python
/// class Face(TypedDict):
///     type: Literal["face"]
///     id: int | None
///     name: str | None
/// ```
pub struct Face {
    id: Option<i32>,
    name: Option<String>,
}

impl FromPyObject<'_> for Face {
    fn extract(obj: &PyAny) -> PyResult<Self> {
        let id = obj.get_item("id")?.extract()?;
        let name = obj.get_item("name")?.extract()?;
        Ok(Self { id, name })
    }
}

impl Face {
    pub(crate) fn into_elem(self) -> Option<elem::Face> {
        if let Some(id) = self.id {
            Some(elem::Face::new(id))
        } else if let Some(name) = self.name {
            elem::Face::new_from_name(&name)
        } else {
            None
        }
    }
}
