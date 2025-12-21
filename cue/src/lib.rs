#![warn(missing_docs)]
#![no_std]
//! Definitions of cue and show
#[cfg(feature = "std")]
mod cue;
mod cuemetadata;
#[cfg(feature = "std")]
mod show;
mod showbuilder;
mod showmetadata;

#[cfg(feature = "std")]
pub use cue::Cue;
#[cfg(feature = "std")]
pub use cue::CueSkeleton;
pub use cuemetadata::CueMetadata;

#[cfg(feature = "std")]
pub use show::Show;
#[cfg(feature = "std")]
pub use show::ShowSkeleton;
pub use showmetadata::ShowMetadata;

#[cfg(feature = "std")]
pub use showbuilder::ShowBuilder;
