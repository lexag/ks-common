use mem::network::SubscriberInfo;

/// State of audio playback handler
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct PlaybackHandlerStatus {
    clips: Vec<Vec<u16>>,
}
