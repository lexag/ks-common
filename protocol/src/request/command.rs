use core::fmt;
use core::result;

use event::JumpModeChange;

/// Actions contained in a ControlAction request, representing actions on the core that can be
/// executed safely during realtime playback.
/// A ControlAction must not take enough time to disturb the audio thread, and should in general be
/// lightweight actions.
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ControlAction {
    /// Start transport playback running.
    TransportStart,
    /// Stop transport playback running.
    TransportStop,
    /// Move transport to beginning of cue.
    TransportZero,
    /// Seek to a specific beat index.
    /// Seek does not need to obey beat timing, and should be executed immediately, but should land
    /// in time with the playback, if playback is running, such that the tempo is not disturbed in
    /// a constant tempo cue.
    TransportSeekBeat(u16),
    /// Jump to a specific beat index.
    /// Jump must obey beat timing, and can only set the next beat pointer to the destination and
    /// await a jump.
    TransportJumpBeat(u16),
    /// Load a cue indexed in the loaded show by the given index.
    LoadCueByIndex(u8),
    /// Load a cue from the audio processor's internal cue index register. Generally unused by
    /// clients, and only called internally.
    LoadCueFromSelfIndex,
    /// Increment the audio processor's internal cue index register, and call LoadCueFromSelfIndex
    LoadNextCue,
    /// Decrement the audio processor's internal cue index register, and call LoadCueFromSelfIndex
    LoadPreviousCue,
    /// Internal action for inter-thread status report. Causes the audio processor to dump messages
    /// containing various system status information out to the main thread and onto the network.
    DumpStatus,
    /// Set the gain of a channel by index and gain. 0dB is default.
    SetChannelGain(u8, f32),
    /// Set the mute state of a channel by index
    SetChannelMute(u8, bool),
    /// Change VLT setting with a JumpModeChange
    ChangeJumpMode(JumpModeChange),
    /// Change the playrate.
    /// A playrate different from 100% (0x0064) will currently disable audio and LTC playback, and
    /// only run the metronome
    ChangePlayrate(u16),
}

impl fmt::Display for ControlAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        match self {
            ControlAction::TransportStart => write!(f, "TransportStart"),
            ControlAction::TransportStop => write!(f, "TransportStop"),
            ControlAction::TransportZero => write!(f, "TransportZero"),
            ControlAction::TransportSeekBeat(..) => write!(f, "TransportSeekBeat"),
            ControlAction::TransportJumpBeat(..) => write!(f, "TransportJumpBeat"),
            ControlAction::LoadCueByIndex(..) => write!(f, "LoadCueByIndex"),
            ControlAction::LoadCueFromSelfIndex => write!(f, "LoadCueFromSelfIndex"),
            ControlAction::LoadNextCue => write!(f, "LoadNextCue"),
            ControlAction::LoadPreviousCue => write!(f, "LoadPreviousCue"),
            ControlAction::DumpStatus => write!(f, "DumpStatus"),
            ControlAction::SetChannelGain(..) => write!(f, "SetChannelGain"),
            ControlAction::SetChannelMute(..) => write!(f, "SetChannelMute"),
            ControlAction::ChangeJumpMode(..) => write!(f, "ChangeJumpMode"),
            ControlAction::ChangePlayrate(..) => write!(f, "ChangePlayrate"),
        }
    }
}
