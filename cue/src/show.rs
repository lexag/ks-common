#[cfg(feature = "std")]
use crate::cue::{Cue, CueSkeleton};
use mem::str::StaticString;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::vec::Vec;

/// A Show represents a collection of Cues for semi-linear sequential playback
#[cfg(feature = "std")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Show {
    /// Metadata for this show
    pub metadata: ShowMetadata,
    /// Cue table for this show.
    pub cues: Vec<Cue>,
}

#[cfg(feature = "std")]
impl Show {
    /// Number of slots for cues this type contains.
    pub const NUM_CUES: usize = 64;
}

/// Metadata for a Show instance. Like with [crate::cue::CueMetadata], anything that is human readable and
/// might be of interest to anyone without in-depth technical knowledge about the inner workings
/// of ClicKS should be in ShowMetadata in a human readable format.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct ShowMetadata {
    /// Name of this show. Usually the name of the production
    pub name: StaticString<32>,
    /// User-defined date field. Can be used for date of show programming or date of show
    /// performance.
    pub date: StaticString<32>,
}

/// Lightweight shadow of [Show] for network and uC purposes, see [CueSkeleton]
#[cfg(feature = "std")]
#[derive(Debug, Clone, PartialEq)]
pub struct ShowSkeleton {
    /// Metadata for this show
    pub metadata: ShowMetadata,
    /// Cue table for this show.
    pub cues: Vec<CueSkeleton>,
}

#[cfg(feature = "std")]
impl ShowSkeleton {
    /// Create a new ShowSkeleton from a full show
    pub fn new(show: Show) -> Self {
        Self {
            metadata: show.metadata,
            cues: show.cues.into_iter().map(CueSkeleton::new).collect(),
        }
    }

    /// Create a full show from this skeleton
    pub fn to_show(self) -> Show {
        Show {
            metadata: self.metadata,
            cues: self.cues.into_iter().map(|c| c.to_cue()).collect(),
        }
    }
}
