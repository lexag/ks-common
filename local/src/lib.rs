#![warn(missing_docs)]
#![no_std]
//! Definitions pertaining to local core storage that other clients might still have use for, such
//! as configuration and status
mod audioconfig;
mod audiosource;
mod beatstate;
mod channelconfig;
#[cfg(feature = "std")]
mod combinedstatus;
#[cfg(feature = "std")]
mod cuestate;
mod jackstatus;
mod loggingconfig;
#[cfg(feature = "std")]
mod networkstatus;
mod playbackstate;
mod smallcuestate;
mod systemconfig;
mod transportstate;

pub mod config {
    //! All local storage related to system configuration and settings.
    //! Config values are persistent and, while they can change during runtime, should never change
    //! during live playback
    pub use super::audioconfig::AudioConfiguration;
    pub use super::audioconfig::JACKClientConfiguration;
    pub use super::audioconfig::JACKServerConfiguration;
    pub use super::channelconfig::ChannelAssignment;
    pub use super::channelconfig::ChannelConfiguration;
    pub use super::loggingconfig::LogContext;
    pub use super::loggingconfig::LogKind;
    pub use super::loggingconfig::LoggerConfiguration;
    pub use super::systemconfig::SystemConfiguration;
    pub use super::systemconfig::SystemConfigurationChange;
}

pub mod status {
    //! All local storage related to live runtime status
    //! Status values are non-persistent, and change (in some cases very frequently) during live
    //! playback
    pub use super::audiosource::AudioSourceState;
    pub use super::beatstate::BeatState;
    #[cfg(feature = "std")]
    pub use super::combinedstatus::CombinedStatus;
    #[cfg(feature = "std")]
    pub use super::cuestate::CueState;
    pub use super::jackstatus::AudioDevice;
    pub use super::jackstatus::JACKStatus;
    #[cfg(feature = "std")]
    pub use super::networkstatus::NetworkStatus;
    pub use super::playbackstate::PlaybackState;
    pub use super::smallcuestate::SmallCueState;
    pub use super::transportstate::TransportState;
}
