extern crate std;
use std::vec::Vec;

/// State of audio playback handler
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct PlaybackHandlerStatus {
    /// List indexed by channel of list of playback clips currently loaded
    pub clips: Vec<Vec<u16>>,
}
