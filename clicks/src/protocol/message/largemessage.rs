use crate::cue::Show;
#[cfg(feature = "std")]
use crate::local::{
    config::{LogItem, SystemConfiguration},
    status::{CueState, JACKStatus, NetworkStatus, PlaybackHandlerStatus},
};
use ks_common_generic::typeflags::MessageType;

/// Definition of large messages sent from core to client
/// These should contain detailed state information, and be enough to get a newly connected client
/// up to speed.
#[allow(clippy::large_enum_variant)]
#[cfg(feature = "std")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum LargeMessage {
    /// Current cue has changed. Sent whenever a new cue is loaded.
    CueData(CueState),
    /// Complete show data
    ShowData(Show),
    /// Network status has changed. Sent when a subscriber joins or leaves the network
    NetworkChanged(NetworkStatus),
    /// JACK state changed, sent on startup to report available audio devices, and then again when
    /// an audio device is selected.
    JACKStateChanged(JACKStatus),
    /// System configuration has changed. Sent (to all subscribers) as a response to a
    /// ChangeSystemConfiguration request. Also sent on startup
    ConfigurationChanged(SystemConfiguration),
    /// A log entry was written
    Log(LogItem),
    /// Playback handler loaded new clips
    PlaybackHandlerChanged(PlaybackHandlerStatus),
}

#[cfg(feature = "std")]
impl LargeMessage {
    /// Get the type flag of this message
    pub fn to_type(&self) -> MessageType {
        match self {
            Self::CueData(..) => MessageType::CueData,
            Self::Log(..) => MessageType::Log,
            Self::ShowData(..) => MessageType::ShowData,
            Self::NetworkChanged(..) => MessageType::NetworkChanged,
            Self::JACKStateChanged(..) => MessageType::JACKStateChanged,
            Self::ConfigurationChanged(..) => MessageType::ConfigurationChanged,
            Self::PlaybackHandlerChanged(..) => MessageType::PlaybackHandlerChanged,
        }
    }
}
