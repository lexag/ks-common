use mem::network::SubscriberInfo;
extern crate std;
use std::vec::Vec;

/// Wrapper for network status values
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct NetworkStatus {
    /// List of network subscribers
    pub subscribers: Vec<SubscriberInfo>,
}
