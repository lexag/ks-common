#![warn(missing_docs)]
//! Definitions (and supporting data formats) for Events, which happen on a specific beat in a cue
mod event_inner;
pub use event_inner::Event;
pub use event_inner::EventDescription;
pub use event_inner::JumpModeChange;
pub use event_inner::JumpRequirement;
pub use event_inner::PauseEventBehaviour;

#[cfg(feature = "std")]
mod eventcursor;
#[cfg(feature = "std")]
pub use eventcursor::EventCursor;

#[cfg(feature = "std")]
mod table;
#[cfg(feature = "std")]
pub use table::EventTable;
