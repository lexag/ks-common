use cue::CueMetadata;

/// Status of the current cue in a lightweight, const-size format
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Copy)]
pub struct SmallCueState {
    /// Cue idx of this cue in the show
    pub cue_idx: u16,
    /// Cue data itself
    pub cue_metadata: CueMetadata,
}
