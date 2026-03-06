use mem::str::StaticString;

/// Cue metadata, for all information (mostly strings) regarding a cue that is not specifically
/// playback-data such as beats and events. As a rule, everything that someone not familiar with
/// ClicKS inner working may want to know should be in metadata
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Clone, Debug, Default, PartialEq, Copy)]
pub struct CueMetadata {
    /// Name of the cue, usually a song name or description of what happens on stage
    pub name: StaticString<16>,
    /// Human readable identifier, such as an index, letter, or an alternative cue name
    pub human_ident: StaticString<8>,
}

impl CueMetadata {
    /// Compile time constant empty default function
    pub const fn const_default() -> Self {
        Self {
            name: StaticString::empty(),
            human_ident: StaticString::empty(),
        }
    }
}
