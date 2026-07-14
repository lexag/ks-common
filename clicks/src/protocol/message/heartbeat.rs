use ks_common_generic::str::StaticString;

/// A tiny packet sent from core to client once every couple of seconds to make sure both ends of
/// the connection are alive and to report some basic information about the core
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Copy)]
pub struct Heartbeat {
    /// The version of the ClicKS common library that the core is running on
    pub common_version: StaticString<8>,
    /// The ClicKS core version the core is running
    pub system_version: StaticString<8>,
    /// Core system time (utc without timezone)
    pub system_time: u64,
    /// CPU load of the audio thread, in percent
    pub cpu_use_audio: f32,
    /// Processing frequency of the main thread, in Hz
    pub process_freq_main: u32,
}
