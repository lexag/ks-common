/// State of audio playback channel
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Copy)]
pub struct PlaybackState {
    /// Channel index
    pub channel: u8,
    /// Current clip idx
    pub clip_idx: u16,
    /// Current playback location in clip, in samples.
    pub current_sample: i32,
    /// Is currently playing?
    pub playing: bool,
    /// List of clip indices into a clip bank held by the playback manager
    pub clips: [u16; 16],
    /// Clip length in samples
    pub clip_length: u32,
}

impl PlaybackState {
    /// Initialize a PlaybackState with the given channel index
    pub fn new(channel: u8) -> Self {
        Self {
            channel,
            ..Default::default()
        }
    }
}
