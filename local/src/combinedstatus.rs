use crate::{
    audiosource::AudioSourceState, beatstate::BeatState, cuestate::CueState,
    jackstatus::JACKStatus, networkstatus::NetworkStatus, playbackstate::PlaybackState,
    transportstate::TransportState,
};
use cue::Show;
use mem::smpte::TimecodeInstant;

/// Wrapper type for the core audio processing status.
#[derive(Clone, Debug)]
pub struct CombinedStatus {
    /// State of the 32 audio sources (metronome, timecode, and 30 playback channels)
    pub sources: [AudioSourceState; 32],
    /// Transport state
    pub transport: TransportState,
    /// Cue state
    pub cue: CueState,
    /// The loaded show in its entirety
    pub show: Show,
    /// Network status
    pub network_status: NetworkStatus,
    /// JACK audio server-client status
    pub jack_status: JACKStatus,
}

impl Default for CombinedStatus {
    fn default() -> Self {
        Self {
            // FIXME: uglyyyyyyyyyyyyyyyy
            sources: [
                AudioSourceState::BeatStatus(BeatState::default()),
                AudioSourceState::TimeStatus(TimecodeInstant::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
            ],
            transport: TransportState::default(),
            cue: CueState::default(),
            show: Show::default(),
            network_status: NetworkStatus::default(),
            jack_status: JACKStatus::default(),
        }
    }
}

impl CombinedStatus {
    /// Get the metrome beat state from channel 1
    pub fn beat_state(&self) -> BeatState {
        if self.sources.is_empty() {
            return BeatState::default();
        }
        if let AudioSourceState::BeatStatus(state) = &self.sources[0] {
            *state
        } else {
            panic!(
                "Metronome is not in slot 0. Slot 0 contains {:?}",
                &self.sources[0]
            )
        }
    }
    /// Get the timecode time state from channel 2
    pub fn time_state(&self) -> TimecodeInstant {
        if self.sources.is_empty() {
            return TimecodeInstant::default();
        }
        if let AudioSourceState::TimeStatus(state) = &self.sources[1] {
            *state
        } else {
            panic!(
                "Timecode is not in slot 1. Slot 1 contains {:?}",
                &self.sources[1]
            )
        }
    }
}
