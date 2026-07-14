use core::fmt;
use ks_common_generic::{smpte::Timecode, str::StaticString};

/// Conditional VLT requirement to perform a [EventDescription::JumpEvent].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Debug, PartialOrd, Ord, Eq, Copy, Default)]
pub enum JumpRequirement {
    /// VLT must be on
    #[default]
    JumpModeOn,
    /// VLT must be off
    JumpModeOff,
    /// Jump regardless of VLT
    None,
}

impl fmt::Display for JumpRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JumpRequirement::JumpModeOn => write!(f, "VLT On"),
            JumpRequirement::JumpModeOff => write!(f, "VLT Off"),
            JumpRequirement::None => write!(f, "None"),
        }
    }
}

/// How (if at all) to change VLT state, for example after a jump or on a request from client
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Debug, Default, PartialOrd, Ord, Eq, Copy)]
pub enum JumpModeChange {
    /// Set VLT on
    SetOn,
    /// Set VLT off
    SetOff,
    /// Toggle VLT: on -> off, off -> on
    Toggle,
    /// Do nothing: on -> on, off -> off
    #[default]
    None,
}

impl fmt::Display for JumpModeChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JumpModeChange::SetOn => write!(f, "Set VLT"),
            JumpModeChange::SetOff => write!(f, "Trip VLT"),
            JumpModeChange::Toggle => write!(f, "Toggle VLT"),
            JumpModeChange::None => write!(f, "None"),
        }
    }
}
impl JumpModeChange {
    /// What should the new VLT value be, considering self and the old VLT value
    pub fn vlt(&self, vlt: bool) -> bool {
        match self {
            JumpModeChange::SetOn => true,
            JumpModeChange::SetOff => false,
            JumpModeChange::Toggle => !vlt,
            JumpModeChange::None => vlt,
        }
    }
}

/// When pausing from a PauseEvent, what action to take to prepare for playback again
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Debug, PartialOrd, Ord, Eq, Copy, Default)]
pub enum PauseEventBehaviour {
    /// Pause and do nothing. Playback will resume exactly where stopped, which may have been in
    /// the middle of a beat.
    #[default]
    Hold,
    /// Move back to the beginning of the beat where the pause happened. Will always resume in time.
    RestartBeat,
    /// Move to the start of the first beat in the cue. Will always start in time.
    RestartCue,
    /// Move to the start of the first beat in the next cue after this one. Will always start in
    /// time.
    NextCue,
    /// Jump to the start of an arbitrary beat in this cue. Will always start in time.
    Jump {
        /// Beat idx to start at
        destination: u16,
    },
}

impl fmt::Display for PauseEventBehaviour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PauseEventBehaviour::Hold => {
                write!(f, "Hold")
            }
            PauseEventBehaviour::RestartBeat => {
                write!(f, "Restart beat")
            }
            PauseEventBehaviour::RestartCue => {
                write!(f, "Restart cue")
            }
            PauseEventBehaviour::NextCue => {
                write!(f, "Next cue")
            }
            PauseEventBehaviour::Jump { .. } => {
                write!(f, "Jump to beat")
            }
        }
    }
}

/// Event defines any event which happens on a specific beat in a cue.
/// All triggers, markers and similar are events.
///
/// Events can be unpopulated, in which case they have location = u16::MAX and event = None
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Event {
    /// Location (beat idx) of this event in the cue
    pub location: u16,
    /// Event descriptor with details
    pub event: Option<EventDescription>,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            location: u16::MAX,
            event: None,
        }
    }
}

impl Event {
    /// Create a new event from a location and description
    pub fn new(location: u16, description: EventDescription) -> Self {
        Self {
            location,
            event: Some(description),
        }
    }

    /// Create a new null event, without location or event desciption
    pub const fn null() -> Self {
        Self {
            location: u16::MAX,
            event: None,
        }
    }

    /// Is this event null (opposite of populated), i.e. is this event just an empty slot in the
    /// event table (true), or an actual event (false)?
    pub fn is_null(&self) -> bool {
        self.location == u16::MAX || self.event.is_none()
    }
}

/// EventDescription contains definitions for all event types, and the data they contain
/// All events are const-size to support uC communication, but must not be equal size to each
/// other. Guideline is about 128 bytes per event.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Debug, Ord, PartialOrd, Eq, Copy)]
pub enum EventDescription {
    /// When triggered: change the next beat pointer to this event's destination field.
    /// Can be conditional with [JumpRequirement] and can conditionally change VLT state with [JumpModeChange]
    JumpEvent {
        /// Beat idx to jump to
        #[cfg_attr(feature = "serde", serde(default))]
        destination: u16,
        /// Conditional VLT requirement required to jump
        #[cfg_attr(feature = "serde", serde(default))]
        requirement: JumpRequirement,
        /// When jumping, should VLT change?
        #[cfg_attr(feature = "serde", serde(default))]
        when_jumped: JumpModeChange,
        /// When passing without jumping, should VLT change?
        #[cfg_attr(feature = "serde", serde(default))]
        when_passed: JumpModeChange,
    },

    /// Marks a direct tempo change. Cosmetic during playback, as tempo comes from each beat's
    /// length property, but used for editing and recalculating tempo
    TempoChangeEvent {
        /// New tempo in BPM
        #[cfg_attr(feature = "serde", serde(default))]
        tempo: u16,
    },
    /// Marks a gradual tempo change. Cosmetic during playback, as tempo comes from each beat's
    /// length property, but used for editing and recalculating tempo
    GradualTempoChangeEvent {
        /// Old tempo (ramp start tempo) in BPM
        #[cfg_attr(feature = "serde", serde(default))]
        start_tempo: u16,
        /// New tempo (ramp end tempo) in BPM
        #[cfg_attr(feature = "serde", serde(default))]
        end_tempo: u16,
        /// Length of tempo ramp in beats
        #[cfg_attr(feature = "serde", serde(default))]
        length: u16,
    },

    /// When triggered: start playing a clip on a playback channel
    PlaybackEvent {
        /// Clip start offset in samples
        #[cfg_attr(feature = "serde", serde(default))]
        sample: i32,
        /// targeted playback channel
        #[cfg_attr(feature = "serde", serde(default))]
        channel_idx: u16,
        /// playback clip idx
        #[cfg_attr(feature = "serde", serde(default))]
        clip_idx: u16,
    },
    /// When triggered: stop playback on a channel
    PlaybackStopEvent {
        /// Targeted playback channel
        #[cfg_attr(feature = "serde", serde(default))]
        channel_idx: u16,
    },

    /// Marks an SMPTE LTC timestamp
    TimecodeEvent {
        /// Timecode Instant this LTC starts at
        #[cfg_attr(feature = "serde", serde(default))]
        time: Timecode,
    },

    /// Marks the point where SMPTE LTC should stop running
    TimecodeStopEvent,

    /// Cosmetic marker for rehearsal marks
    RehearsalMarkEvent {
        /// Rehearsal mark label
        /// Can be empty.
        #[cfg_attr(feature = "serde", serde(default))]
        label: StaticString<8>,
    },

    /// When triggered: pause transport, and execute the PauseEventBehaviour.
    PauseEvent {
        /// What to do after pausing
        #[cfg_attr(feature = "serde", serde(default))]
        behaviour: PauseEventBehaviour,
    },

    /// Marker for overriding beat lengths during beat length recalculation
    BeatLengthOverride {
        /// New length in us
        #[cfg_attr(feature = "serde", serde(default))]
        length: u32,
    },

    /// Marker for overriding beat counts during beat renumbering
    /// Renumbering will continue from this point
    BeatCountOverride {
        /// New beat count
        #[cfg_attr(feature = "serde", serde(default))]
        count: u8,
        /// New bar number
        #[cfg_attr(feature = "serde", serde(default))]
        bar_number: u8,
    },
}

impl EventDescription {
    /// Get human readable name of event type. Does not contain inner event data
    pub fn get_name(&self) -> &str {
        match self {
            EventDescription::JumpEvent { .. } => "Jump",
            EventDescription::TempoChangeEvent { .. } => "Tempo Change",
            EventDescription::GradualTempoChangeEvent { .. } => "Gradual Tempo Change",
            EventDescription::PlaybackEvent { .. } => "Playback",
            EventDescription::PlaybackStopEvent { .. } => "Playback Stop",
            EventDescription::TimecodeEvent { .. } => "Timecode",
            EventDescription::TimecodeStopEvent => "Timecode Stop",
            EventDescription::RehearsalMarkEvent { .. } => "Rehearsal Mark",
            EventDescription::PauseEvent { .. } => "Pause Event",
            EventDescription::BeatCountOverride { .. } => "Beat count override mark",
            EventDescription::BeatLengthOverride { .. } => "Beat length override mark",
        }
    }
}
