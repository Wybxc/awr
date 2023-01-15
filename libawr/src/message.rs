//! 消息内容。

use ricq::msg::{
    elem::{self, RQElem},
    MessageChain, MessageChainBuilder,
};

/// 消息内容。
#[derive(Debug, Default, Clone)]
pub struct MessageContent {
    inner: MessageChain,
}

impl MessageContent {
    pub(crate) fn into_inner(self) -> MessageChain {
        self.inner
    }

    /// 获取消息内容的所有消息段。
    pub fn segments(&self) -> impl Iterator<Item = RQElem> + '_ {
        self.inner.0.iter().map(|elem| elem.clone().into())
    }

    /// 获取消息内容的所有消息段。
    pub fn into_segments(self) -> impl Iterator<Item = RQElem> {
        self.inner.0.into_iter().map(|elem| elem.into())
    }
}

/// 消息内容构造器。
#[derive(Default, Debug, Clone)]
pub struct MessageContentBuilder {
    pub(crate) inner: MessageChainBuilder,
}

impl MessageContentBuilder {
    /// 构造一个新的消息内容构造器。
    pub fn new() -> Self {
        Default::default()
    }

    /// 构造消息内容。
    pub fn build(self) -> MessageContent {
        MessageContent {
            inner: self.inner.build(),
        }
    }

    /// 添加消息段。
    pub fn push<T: MessageSegment>(mut self, segment: T) -> Self {
        segment.push_to(&mut self);
        self
    }
}

/// 消息段。
pub trait MessageSegment {
    /// 将消息段添加到消息内容构造器。
    fn push_to(self, builder: &mut MessageContentBuilder);
}

macro_rules! impl_message_segment {
    ($($ty:ty),*) => {
        $(
            impl MessageSegment for $ty {
                fn push_to(self, builder: &mut MessageContentBuilder) {
                    builder.inner.push(self);
                }
            }
        )*
    };
}

impl_message_segment!(elem::At);
impl_message_segment!(elem::Face);
impl_message_segment!(elem::FriendImage);
impl_message_segment!(elem::GroupImage);

impl MessageSegment for String {
    fn push_to(self, builder: &mut MessageContentBuilder) {
        builder.inner.push_str(&self);
    }
}

impl MessageSegment for &str {
    fn push_to(self, builder: &mut MessageContentBuilder) {
        builder.inner.push_str(self);
    }
}

/// 构造消息内容。
#[macro_export]
macro_rules! msg {
    ($($elem:expr),* $(,)?) => {
        $crate::message::MessageContentBuilder::new()
            $(
                .push($elem)
            )*
            .build()
    };
}
