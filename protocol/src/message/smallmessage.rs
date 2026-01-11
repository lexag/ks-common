use crate::message::heartbeat::Heartbeat;
use event::EventDescription;
use local::status::{BeatState, SmallCueState, TransportState};
use mem::typeflags::MessageType;

/// Definition of small messages sent from core to client.
/// Variant max size is 128 bytes, and must be const-size
/// Supports no-std uC systems, and should contain enough information to display accurately in
/// realtime during playback.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Copy)]
pub enum SmallMessage {
    /// Transport state has changed. Sent once whenever jumping or starting/stopping playback, and
    /// multiple times per second during playback.
    TransportData(TransportState),
    /// Current beat has changed. Sent at the start of a new beat during playback
    BeatData(BeatState),
    /// Current cue has changed. Sent when loading a new cue.
    CueData(SmallCueState),
    /// A shutdown has been requested. Sent to all subscribers on a Shutdown request, telling them
    /// to unsubscribe and/or disconnect, and expect to not receive subsequent Heartbeats
    ShutdownOccured,
    /// Sent every few seconds to all clients, reporting core status and making sure the connection
    /// is alive
    Heartbeat(Heartbeat),
    /// A playback timeline event just occured
    EventOccured(EventDescription),
}

impl SmallMessage {
    /// Get the type flag of this message
    pub fn to_type(&self) -> MessageType {
        match self {
            Self::TransportData(..) => MessageType::TransportData,
            Self::BeatData(..) => MessageType::BeatData,
            Self::ShutdownOccured => MessageType::ShutdownOccured,
            Self::Heartbeat(..) => MessageType::Heartbeat,
            Self::EventOccured(..) => MessageType::EventOccured,
            Self::CueData(..) => MessageType::SmallCueData,
        }
    }
}
