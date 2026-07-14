use crate::cue::Cue;

/// Status of the current cue
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default)]
pub struct CueState {
    /// Cue idx of this cue in the show
    pub cue_idx: u16,
    /// Cue data itself
    pub cue: Cue,
}
