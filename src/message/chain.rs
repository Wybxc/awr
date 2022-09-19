//! 消息链。

use ricq_core::msg;
use ricq_core::msg::MessageChain;

use super::elements::Element;

pub(crate) async fn build_message_chain(elements: impl IntoIterator<Item = Element>) -> MessageChain {
    let iter = elements.into_iter();
    let mut result = msg::MessageChain::default();
    for elem in iter {
        match elem {
            Element::Text(text) => result.push(text.into_elem()),
            Element::At(at) => result.push(at.into_elem()),
        };
    }
    result
}
