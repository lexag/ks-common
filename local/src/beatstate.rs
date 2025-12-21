use beat::Beat;
use event::JumpModeChange;

/// Current timing state of the metronome
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Copy)]
pub struct BeatState {
    /// Current beat idx
    pub beat_idx: u16,
    /// Next beat idx
    pub next_beat_idx: u16,
    /// Current beat data
    pub beat: Beat,
    /// Requested change of VLT. An audio source can't change the VLT by itself, so it must be
    /// forwarded to a module that can.
    pub requested_vlt_action: JumpModeChange,
}
