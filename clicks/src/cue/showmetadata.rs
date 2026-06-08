use ks_common_generic::str::StaticString;

/// Metadata for a Show instance. Like with [crate::cuemetadata::CueMetadata], anything that is human readable and
/// might be of interest to anyone without in-depth technical knowledge about the inner workings
/// of ClicKS should be in ShowMetadata in a human readable format.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct ShowMetadata {
    /// Name of this show. Usually the name of the production
    pub name: StaticString<32>,
    /// User-defined date field. Can be used for date of show programming or date of show
    /// performance.
    pub date: StaticString<32>,
}
