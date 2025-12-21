/// Enumerator for specifying the type of client->core request.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum RequestType {
    /// This request is a NotifySubscribers request.
    NotifySubscribers,
    /// This request is a Shutdown request.
    Shutdown,
    /// This request is a Initialize request.
    Initialize,
    /// This request is a RoutingChange request.
    RoutingChange,
    /// This request is a ControlCommand request.
    ControlCommand,
    /// This request is a Subscribe request.
    Subscribe,
    /// This request is a Unsubscribe request.
    Unsubscribe,
    /// This request is a SetConfiguration request.
    SetConfiguration,
    /// This request is a Ping request.
    Ping,
}
