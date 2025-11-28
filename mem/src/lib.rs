#![warn(missing_docs)]
#![no_std]
//! Low-level data representation formats
mod message;
mod request;
mod timecode;

pub mod typeflags {
    //! Bitflags for various union type subtypes
    use super::message;
    use super::request;
    pub use message::MessageType;
    pub use request::RequestType;
}
/// Low level data types for defining network communication
pub mod network;
/// Low level data types for string handling
#[allow(missing_docs)]
pub mod str;
/// Low level data types for handling and formatting time
pub mod time;

/// Low level data types for SMPTE Timecode
pub mod smpte {
    pub use super::timecode::TimecodeInstant;
}
