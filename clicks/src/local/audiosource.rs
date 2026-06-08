use crate::{
    local::beatstate::BeatState, local::playbackstate::PlaybackState, local::status::TimecodeState,
};

/// Wrapper type for the state of an audio source (audio channel)
#[derive(Clone, Debug, Copy)]
pub enum AudioSourceState {
    /// Metronome channel state
    BeatStatus(BeatState),
    /// Timecode channel state
    TimeStatus(TimecodeState),
    /// Playback channel state
    PlaybackStatus(PlaybackState),
}
