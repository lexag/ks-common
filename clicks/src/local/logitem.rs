use crate::local::config::{LogContext, LogKind};

extern crate std;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug)]
/// A debug (or information, warning, error) log entry
pub struct LogItem {
    /// Logged string; to print out, display or save, in its entirety.
    pub message: std::string::String,
    /// Category of this log entry
    pub kind: LogKind,
    /// Context for this log entry; which component it came from
    pub context: LogContext,
    /// UTC millisecond timestamp of this log entry
    pub time: u64,
}

impl LogItem {
    /// Construct a new log entry with automatic timestamp
    pub fn new(msg: std::string::String, context: LogContext, kind: LogKind) -> Self {
        Self {
            message: msg,
            kind,
            context,
            time: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}
