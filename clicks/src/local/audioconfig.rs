use ks_common_generic::str::StaticString;

/// Wrapper type combining both JACK client and JACK server configurations
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct AudioConfiguration {
    /// Server configuration
    pub server: JACKServerConfiguration,
    /// Client configuration
    pub client: JACKClientConfiguration,
}

/// JACK server configuration. The JACK server is the audio thread handler running on the OS of the
/// ClicKS core machine.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct JACKServerConfiguration {
    /// Device id for the connected audio device, either external or internal
    pub device_id: StaticString<32>,
    /// System name, used for port searching
    pub system_name: StaticString<32>,
    /// Sample rate, in Hz
    pub sample_rate: u16,
    /// Period size, in samples
    pub period_size: u16,
}

/// JACK client configuration. The JACK client is *not* a ClicKS client, rather it is the core's
/// audio client that connects to the audio server. JACK server and client both run on the core
/// machine.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct JACKClientConfiguration {
    /// Name of this client
    pub name: StaticString<32>,
}
