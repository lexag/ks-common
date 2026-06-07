use ks_common_generic::smpte::TimecodeInstant;

/// Status of current SMPTE timecode frame
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Copy, Default)]
pub struct TimecodeState {
    /// Is timecode currently running, i.e. is timestamp changing?
    pub running: bool,
    /// current LTC timestamp
    pub ltc: TimecodeInstant,
}
