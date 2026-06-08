/// Status of current transport location and state
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Copy)]
pub struct TransportState {
    /// Is transport currently running, i.e. is location changing
    pub running: bool,
    /// VLT state
    pub vlt: bool,
    /// Playrate in percent
    pub playrate_percent: u16,
}

impl Default for TransportState {
    fn default() -> Self {
        Self {
            running: false,
            vlt: false,
            playrate_percent: 100,
        }
    }
}
