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
    //pub use super::timecode::TimecodeInstant;
    //pub use super::timecode::TimecodeProperties;
    //pub use super::timecode::TimecodeUserBitFormat;

    pub use super::timecode::FrameRate;
    pub use super::timecode::FrameRateInfo;
    pub use super::timecode::Timecode;
    pub use super::timecode::TimecodeError;
    pub use super::timecode::TimecodeOffset;

    /// Encoder and decoder to and from audio LTC signal
    pub mod ltc {
        pub use crate::timecode::readwrite::LtcReader;
        pub use crate::timecode::readwrite::LtcReaderConfig;
        pub use crate::timecode::readwrite::LtcWriter;
        pub use crate::timecode::readwrite::LtcWriterConfig;
    }
}

#[allow(missing_docs)]
pub mod test_size;

/// Software version of the KS common library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
