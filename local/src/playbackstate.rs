/// State of audio playback channel
#[derive(Clone, Debug, Default, Copy)]
pub struct PlaybackState {
    /// Current clip idx
    pub clip_idx: u16,
    /// Current playback location in clip, in samples.
    pub current_sample: i32,
    /// Is currently playing?
    pub playing: bool,
    /// List of clip indices into a clip bank held by the playback manager
    pub clips: [u16; 16],
}
