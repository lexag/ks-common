use crate::request::command::ControlAction;
use local::config::SystemConfigurationChange;
use mem::{
    network::{ConnectionInfo, SubscriberInfo},
    typeflags::RequestType,
};

/// Requests that are given to the core by a client.
/// Requests may be non-realtime-safe, apart from ControlAction, which has its own subactions that
/// must all be realtime safe during playback.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Request {
    /// Inform all subscribers with all relevant system status messages.
    /// Used to get a complete update on status when connecting a new subscriber.
    NotifySubscribers,
    /// Safely shutdown the core and disconnect all clients
    Shutdown,
    /// Start the audio processor, and load all show data. This causes some configuration variables
    /// to be locked.
    Initialize,
    /// Change routing (from, to, connect?) between a ClicKS channel and a JACK hardware out.
    /// This only has effect when the audio processor is running.
    ChangeRouting(u8, u8, bool),
    /// Execute a ControlAction.
    /// This only has effect when the audio processor is running.
    ControlAction(ControlAction),
    /// Subscribe to an information flow.
    Subscribe(SubscriberInfo),
    /// Unsubscribe from an information flow.
    Unsubscribe(ConnectionInfo),
    /// Change a section of system configuration.
    /// Some sections can only be changed before starting the audio processor.
    ChangeConfiguration(SystemConfigurationChange),
    /// Notify the core that this client still is alive and connected.
    /// The core will forget clients which it has not heard from for some time.
    Ping,
}

impl Request {
    /// Get the type flag for this request.
    pub fn to_type(&self) -> RequestType {
        match self {
            Self::NotifySubscribers => RequestType::NotifySubscribers,
            Self::Shutdown => RequestType::Shutdown,
            Self::Initialize => RequestType::Initialize,
            Self::ChangeRouting(..) => RequestType::RoutingChange,
            Self::ControlAction(..) => RequestType::ControlCommand,
            Self::Subscribe(..) => RequestType::Subscribe,
            Self::Unsubscribe(..) => RequestType::Unsubscribe,
            Self::ChangeConfiguration(..) => RequestType::SetConfiguration,
            Self::Ping => RequestType::Ping,
        }
    }
}
