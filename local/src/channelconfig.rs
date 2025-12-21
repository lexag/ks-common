use mem::str::StaticString;

/// Cosmetic selector value for whether a channel is mono or stereo, in which case which side
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub enum ChannelAssignment {
    /// This channel is the left half of a stereo channel pair
    L,
    /// This channel is the right half of a stereo channel pair
    R,
    /// This channel is a mono channel
    #[default]
    Mono,
}

/// Configuration values for audio channels
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct ChannelConfiguration {
    /// Name of this channel
    pub name: StaticString<32>,
    /// Mono or stereo assignment
    pub channel_assignment: ChannelAssignment,
    /// Digital gain value of this channel in dB. 0 is unchanged volume.
    pub gain: f32,
}
