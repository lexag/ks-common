#![warn(missing_docs)]
#![no_std]
//! Definitions (and supporting data formats) for Events, which happen on a specific beat in a cue
mod event;
pub use event::Event;
pub use event::EventDescription;
pub use event::JumpModeChange;
pub use event::JumpRequirement;
pub use event::PauseEventBehaviour;

#[cfg(feature = "std")]
mod eventcursor;
#[cfg(feature = "std")]
pub use eventcursor::EventCursor;

#[cfg(feature = "std")]
mod table;
#[cfg(feature = "std")]
pub use table::EventTable;
