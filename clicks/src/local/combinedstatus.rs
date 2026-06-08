use crate::cue::Show;
use crate::{
    local::audiosource::AudioSourceState,
    local::beatstate::BeatState,
    local::cuestate::CueState,
    local::jackstatus::JACKStatus,
    local::networkstatus::NetworkStatus,
    local::playbackstate::PlaybackState,
    local::status::{PlaybackHandlerStatus, TimecodeState},
    local::transportstate::TransportState,
};

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
    /// Playback handler clip status
    pub playback_status: PlaybackHandlerStatus,
}

impl Default for CombinedStatus {
    fn default() -> Self {
        Self {
            sources: [
                AudioSourceState::BeatStatus(BeatState::default()),
                AudioSourceState::TimeStatus(TimecodeState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::new(0)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(1)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(2)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(3)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(4)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(5)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(6)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(7)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(8)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(9)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(10)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(11)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(12)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(13)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(14)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(15)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(16)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(17)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(18)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(19)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(20)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(21)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(22)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(23)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(24)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(25)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(26)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(27)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(28)),
                AudioSourceState::PlaybackStatus(PlaybackState::new(29)),
            ],
            transport: TransportState::default(),
            cue: CueState::default(),
            show: Show::default(),
            network_status: NetworkStatus::default(),
            jack_status: JACKStatus::default(),
            playback_status: PlaybackHandlerStatus::default(),
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
    pub fn time_state(&self) -> TimecodeState {
        if self.sources.is_empty() {
            return TimecodeState::default();
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
