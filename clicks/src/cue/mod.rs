#![warn(missing_docs)]
//! Definitions of cue and show
#[cfg(feature = "std")]
mod cue_inner;
mod cuemetadata;
#[cfg(feature = "std")]
mod show;
mod showbuilder;
mod showmetadata;

#[cfg(feature = "std")]
pub use cue_inner::Cue;
#[cfg(feature = "std")]
pub use cue_inner::CueSkeleton;
pub use cuemetadata::CueMetadata;

#[cfg(feature = "std")]
pub use show::Show;
#[cfg(feature = "std")]
pub use show::ShowSkeleton;
pub use showmetadata::ShowMetadata;

#[cfg(feature = "std")]
pub use showbuilder::ShowBuilder;
