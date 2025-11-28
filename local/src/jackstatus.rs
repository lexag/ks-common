use mem::str::StaticString;

/// Description of a system audio device
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, Copy)]
pub struct AudioDevice {
    /// Id string of this device
    pub id: StaticString<32>,
    /// Human readable name of this device
    pub name: StaticString<32>,
    /// IO-size (num_input_channels, num_output_channels)
    pub io_size: (usize, usize),
}

impl AudioDevice {
    /// Create an Audio Device object from an aplay str:
    /// ```txt
    /// card 1: Headphones [bcm2835 Headphones], device 0: bcm2835 Headphones [bcm2835 Headphones]
    /// ```
    pub fn from_aplay_str(str: &str) -> Option<AudioDevice> {
        let card_idx = &str[str.find("card")? + 5..str.find(':')?];
        let _device_idx = &str[(str.find("device")? + 7)..(str[8..].find(':')? + 8)];
        let mut id_str = StaticString::new(card_idx);
        id_str.content.rotate_right(3);
        id_str.set_char(0, b'h');
        id_str.set_char(1, b'w');
        id_str.set_char(2, b':');
        Some(Self {
            id: id_str,
            io_size: (0, 0),
            name: StaticString::new(str),
        })
    }
}

/// JACK audio status. Contains both some server specific and some client specific data, as the
/// line between client and server is blurred on an integrated system with its own JACK-server and
/// a single client.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, Copy)]
pub struct JACKStatus {
    /// Before starting the audio processing, the first 8 available devices to connect the JACK server to.
    /// After starting the audio processing, no audio devices are available and this value shows
    /// [None; 8]
    pub available_devices: [Option<AudioDevice>; 8],
    /// IO-size (num_input_channels, num_output_channels) of the selected audio device.
    /// Only available after audio processing starts.
    pub io_size: (usize, usize),
    /// Buffer size in samples
    /// Only available after audio processing starts.
    pub buffer_size: usize,
    /// Sample rate in Hz
    /// Only available after audio processing starts.
    pub sample_rate: usize,
    /// Frame size in samples
    /// Only available after audio processing starts.
    pub frame_size: usize,
    /// Active routing connections between the ClicKS JACK audio client and the JACK audio server.
    /// Each entry corresponds to one ClicKS audio channel, and each bit in the entry to one JACK
    /// server hardware out. I.e. if <code>connections\[0\] == 0x0003</code>, the metronome audio
    /// (ClicKS channel 1) is routed to hardware outputs 1 and 2.
    /// Only available after audio processing starts.
    pub connections: [u32; 32],
    /// Name of the JACK client
    pub client_name: StaticString<32>,
    /// Name of the JACK system output
    pub output_name: StaticString<32>,
    /// Whether audio processing is running
    pub running: bool,
}
