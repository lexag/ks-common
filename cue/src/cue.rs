use beat::Beat;
use event::{Event, EventCursor, EventDescription, EventTable, JumpModeChange, JumpRequirement};

extern crate std;
use crate::CueMetadata;
use std::vec::Vec;

/// A Cue represents a musical or technical "cue", in the meaning semi-linear timeline progression
/// with a clearly defined start and end, which may be followed or preceded by other cues.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Cue {
    /// Metadata for this cue
    pub metadata: CueMetadata,
    /// Table of beats in this cue
    pub beats: Vec<Beat>,
    /// Table of events that will/can occur during this cue
    pub events: EventTable,
}

/// Shadow-type of Cue, without a Beat-table. Used for lightweight network communication with
/// clients that do not care about knowing all beat details for the cue, but may still need
/// information about events and metadata
#[derive(Clone, Debug, PartialEq)]
pub struct CueSkeleton {
    /// Metadata for this cue
    pub metadata: CueMetadata,
    /// Table of events that will/can occur during this cue
    pub events: EventTable,
}

impl CueSkeleton {
    /// Create a new CueSkeleton from a full cue
    pub fn new(cue: Cue) -> Self {
        Self {
            metadata: cue.metadata,
            events: cue.events,
        }
    }

    /// Create a full cue from this skeleton
    pub fn to_cue(self) -> Cue {
        Cue {
            metadata: self.metadata,
            events: self.events,
            ..Default::default()
        }
    }
}

impl Default for Cue {
    fn default() -> Cue {
        Cue::empty()
    }
}

impl Cue {
    /// Create an empty cue containing no beats.
    /// The cue is valid for playback
    pub const fn empty() -> Cue {
        Cue {
            events: EventTable::empty(),
            beats: Vec::new(),
            metadata: CueMetadata::const_default(),
        }
    }

    /// Create an example Cue with 100 populated beats in 4/4 at 120 BPM, and a PlaybackEvent on
    /// the first beat.
    pub fn example() -> Cue {
        let mut br = Cue::empty();
        for i in 0..100 {
            br.beats.push(Beat {
                count: i as u8 % 4 + 1,
                bar_number: i as u8 / 4 + 1,
                length: 500_000,
            });
        }
        br.events.set(
            0,
            Event {
                location: 0,
                event: Some(EventDescription::PlaybackEvent {
                    channel_idx: 0,
                    clip_idx: 0,
                    sample: 0,
                }),
            },
        );
        br
    }

    /// Create an example Cue with 8 populated beats that loops 4 beats.
    pub fn example_loop() -> Cue {
        let mut br = Cue::empty();
        for i in 0..8 {
            br.beats.push(Beat {
                count: i as u8 % 4 + 1,
                bar_number: i as u8 / 4 + 1,
                length: 500_000,
            });
        }
        br.events.set(
            0,
            Event {
                location: 0,
                event: Some(EventDescription::PlaybackEvent {
                    channel_idx: 0,
                    clip_idx: 0,
                    sample: 0,
                }),
            },
        );
        br.events.set(
            1,
            Event {
                location: 3,
                event: Some(EventDescription::JumpEvent {
                    destination: 0,
                    requirement: JumpRequirement::None,
                    when_jumped: JumpModeChange::None,
                    when_passed: JumpModeChange::None,
                }),
            },
        );
        br
    }

    /// Get a beat by its index in this cue.
    /// Returns None if idx is more than the length of this cue, or if the indexed beat is not
    /// populated.
    pub fn get_beat(&self, idx: u16) -> Option<Beat> {
        if self.beats.len() <= idx as usize || self.beats[idx as usize].length == 0 {
            return None;
        }
        Some(self.beats[idx as usize])
    }

    /// Get a copy of the beat table
    pub fn get_beats(&self) -> Vec<Beat> {
        self.beats.clone()
    }

    /// Reorder all this Cue's beats' bar numbers and beat numbers, starting from m1b1-m1b2-etc
    /// Increments bar number when running into a downbeat (beat number = 1) or when the bar number
    /// changes
    ///
    /// Thus, it will reorder this:
    /// bar : 1 1 1 1 3 3 3 3 3 3 3 3
    /// beat: 1 2 2 3 4 2 3 4 1 2 5 4
    /// into this:
    /// bar : 1 1 1 1 2 2 2 2 3 3 3 3
    /// beat: 1 2 3 4 1 2 3 4 1 2 3 4
    /// but will leave this alone, as there is no indication of where to break the bar:
    /// bar : 1 1 1 1 1 1 1 1 1
    /// beat: 1 2 3 4 5 6 7 8 9
    pub fn reorder_numbers(&mut self) {
        if self.beats.is_empty() {
            return;
        }
        let mut bar = if self.beats[0].bar_number == 0 { 0 } else { 1 };
        let mut count = 1u16;
        let mut prev_bar = bar;
        for beat in &mut self.beats {
            if beat.is_null() {
                break;
            }

            if prev_bar != beat.bar_number || (beat.count == 1 && prev_bar > 1) {
                count = 1;
                bar += 1;
            }

            prev_bar = beat.bar_number;

            beat.bar_number = bar;
            beat.count = count as u8;

            count += 1;
        }
    }

    /// Recalculate beat lengths according to tempo calculations:
    /// - Starts at 120 BPM by default
    /// - If any TempoChangeEvent occurs, the new tempo is used
    /// - If any GradualTempoChangeEvent occurs, the tempo ramps linearly from old to new
    /// - Else, the tempo is the same as last beat.
    pub fn recalculate_tempo_changes(&mut self) {
        let mut beat_length: u32 = 1000000 * 60 / 120;
        let mut beats_left_in_change: u16 = 0;
        let mut accelerator: f32 = 0.0;

        let mut new_beats = self.beats.clone();
        let mut cursor = EventCursor::new(&self.events);

        for beat in &mut new_beats {
            if beat.is_null() {
                break;
            }

            if let Some(EventDescription::TempoChangeEvent { tempo }) =
                cursor.get().unwrap_or_default().event
            {
                cursor.step();
                beat_length = 1000000 * 60 / tempo as u32;
                accelerator = 0.0;
            }
            if let Some(EventDescription::GradualTempoChangeEvent {
                start_tempo,
                end_tempo,
                length,
            }) = cursor.get().unwrap_or_default().event
            {
                cursor.step();
                beat_length = 1000000 * 60 / start_tempo as u32;
                accelerator = (60000000.0 / end_tempo as f32 - 60000000.0 / start_tempo as f32)
                    / length as f32;
                beats_left_in_change = length;
            }
            beat.length = beat_length;
            beat_length = (beat_length as f32 + accelerator) as u32;
            beats_left_in_change = beats_left_in_change.saturating_sub(1);
            if beats_left_in_change == 0 {
                accelerator = 0.0;
            }
        }

        self.beats = new_beats;
    }

    /// Is this cue an empty cue slot in the show, or is it populated with actual cue data?
    pub fn is_null(&self) -> bool {
        self.beats[0].is_null()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_beat() {
        let c = Cue::example();
        assert!(c.get_beat(56).is_some());
        assert!(c.get_beat(170).is_none());
        assert!(c.get_beat(267).is_none());
    }

    #[test]
    fn test_reorder_numbers() {
        fn make_dummy_beat(bar: u8, beat: u8) -> Beat {
            Beat {
                count: beat,
                bar_number: bar,
                length: 500_000,
            }
        }
        let mut c = Cue::empty();
        c.beats.push(make_dummy_beat(1, 1));
        c.beats.push(make_dummy_beat(1, 2));
        c.beats.push(make_dummy_beat(1, 2));
        c.beats.push(make_dummy_beat(1, 3));
        c.beats.push(make_dummy_beat(3, 4));
        c.beats.push(make_dummy_beat(3, 2));
        c.beats.push(make_dummy_beat(3, 3));
        c.beats.push(make_dummy_beat(3, 4));
        c.beats.push(make_dummy_beat(3, 1));
        c.beats.push(make_dummy_beat(3, 2));
        c.beats.push(make_dummy_beat(3, 5));
        c.beats.push(make_dummy_beat(3, 4));

        c.reorder_numbers();

        assert_eq!(c.beats[0], make_dummy_beat(1, 1));
        assert_eq!(c.beats[1], make_dummy_beat(1, 2));
        assert_eq!(c.beats[2], make_dummy_beat(1, 3));
        assert_eq!(c.beats[3], make_dummy_beat(1, 4));
        assert_eq!(c.beats[4], make_dummy_beat(2, 1));
        assert_eq!(c.beats[5], make_dummy_beat(2, 2));
        assert_eq!(c.beats[6], make_dummy_beat(2, 3));
        assert_eq!(c.beats[7], make_dummy_beat(2, 4));
        assert_eq!(c.beats[8], make_dummy_beat(3, 1));
        assert_eq!(c.beats[9], make_dummy_beat(3, 2));
        assert_eq!(c.beats[10], make_dummy_beat(3, 3));
        assert_eq!(c.beats[11], make_dummy_beat(3, 4));
    }

    #[test]
    fn test_recalculate_tempo() {
        let mut c = Cue::example();
        c.events.set(
            0,
            Event::new(0, EventDescription::TempoChangeEvent { tempo: 125 }),
        );
        c.recalculate_tempo_changes();

        assert_eq!(c.beats[0].length, 480000);
        assert_eq!(c.beats[1].length, 480000);
        assert_eq!(c.beats[3].length, 480000);
    }
}
