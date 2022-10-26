//! 消息链。

use ricq_core::msg;
use ricq_core::msg::MessageChain;

use super::elements::Element;

use anyhow::{anyhow, Result};

/// 构建好友消息链.
pub(crate) async fn build_friend_message_chain(
    elements: impl IntoIterator<Item = Element>,
) -> Result<MessageChain> {
    let iter = elements.into_iter();
    let mut result = msg::MessageChain::default();
    for elem in iter {
        match elem {
            Element::Text(text) => result.push(text.into_elem()),
            Element::At(at) => result.push(at.into_elem()),
            Element::Face(face) => result.push(
                face.into_elem()
                    .ok_or_else(|| anyhow!("invalid face element"))?,
            ),
        };
    }
    Ok(result)
}
