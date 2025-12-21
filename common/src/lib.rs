#![warn(missing_docs)]
#![no_std]
//! This crate contains common functionality and data formats shared between the ClicKS core and
//! client components
mod test_size;

pub use beat;
pub use cue;
pub use event;
pub use local;
pub use mem;
pub use protocol;

/// Software version of ClicKS common library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
