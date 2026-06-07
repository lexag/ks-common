use crate::message::{LargeMessage, SmallMessage};
use ks_common_generic::typeflags::MessageType;

/// Wrapper type for both types of core -> client messages
#[allow(clippy::large_enum_variant)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum Message {
    /// Small, const-size, uC-friendly status updates
    Small(SmallMessage),
    /// Large dynamic information dump messages
    Large(LargeMessage),
}

impl Message {
    /// Get the type flag of this message
    pub fn to_type(&self) -> MessageType {
        match self {
            Message::Small(message) => message.to_type(),
            Message::Large(message) => message.to_type(),
        }
    }
}
