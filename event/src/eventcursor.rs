use crate::event::Event;
use crate::table::EventTable;

/// Assisting pointer type that indexes into an [EventTable] and points at a specific event.
/// Implements functionality to seek and step through events in order.
pub struct EventCursor<'a> {
    cursor: u8,
    table: &'a EventTable,
}

impl<'a> EventCursor<'a> {
    /// Create a new event cursor pointing to the first event in the given table
    pub fn new(events: &'a EventTable) -> EventCursor<'a> {
        Self {
            cursor: 0,
            table: events,
        }
    }

    /// Move cursor to point to the next event after or on the location index.
    ///
    /// Example:
    /// <code>
    /// Table [0, 2, 4, 6, 8]
    /// seek(0) -> cursor 0 (loc 0)
    /// seek(1) -> cursor 1 (loc 2)
    /// seek(2) -> cursor 1 (loc 2)
    /// </code>
    pub fn seek(&mut self, location: u16) {
        if self.location() > location {
            self.cursor = 0;
        }
        while self.location() < location && self.cursor < self.table.len() as u8 {
            self.cursor += 1;
        }
    }

    /// Beat idx that the currently pointed at event occurs
    pub fn location(&self) -> u16 {
        self.table.get(self.cursor).map_or(u16::MAX, |e| e.location)
    }

    /// Get the currently pointed at event
    pub fn get(&mut self) -> Option<Event> {
        self.table.get(self.cursor)
    }

    /// Get the currently pointed at event, and step forward.
    /// Returns None if the event is unpopulated (null)
    /// Used in a while loop to step through events
    pub fn get_next(&mut self) -> Option<Event> {
        let e = self.get();
        self.step();
        e
    }

    /// True if cursor is at or before the given location
    /// <code>
    /// while cursor.at_or_before(location) {
    ///     cursor.get_next();
    ///     // Will iterate all events up to and including the current beat (location)
    ///     // and leave the cursor pointing to the next unhandled event after this beat
    /// }
    /// </code>
    pub fn at_or_before(&self, location: u16) -> bool {
        self.location() <= location
    }

    /// Move cursor one step forward. Stops at 256
    pub fn step(&mut self) {
        if self.cursor == u8::MAX {
            return;
        }
        self.cursor += 1;
    }

    /// Move cursor to idx in the event table.
    /// This is not event location, simply indexing into the table
    pub fn goto(&mut self, idx: u8) {
        self.cursor = idx;
    }

    /// Get the currently pointed at index
    pub fn cursor_idx(&self) -> u8 {
        self.cursor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dummy_table() -> EventTable {
        let mut c = EventTable::empty();
        for i in 0..64 {
            c.set(
                i,
                Event {
                    location: i as u16 * 2,
                    event: Some(crate::event::EventDescription::TimecodeEvent {
                        time: mem::smpte::TimecodeInstant {
                            frame_rate: 25,
                            h: 0,
                            m: 0,
                            s: 0,
                            f: 0,
                            frame_progress: 0,
                        },
                    }),
                },
            );
        }
        c
    }

    #[test]
    fn test_seek() {
        let table = make_dummy_table();
        let mut cursor = EventCursor::new(&table);
        cursor.seek(0);
        assert_eq!(cursor.cursor, 0);

        cursor.seek(16);
        assert_eq!(cursor.cursor, 8);
        cursor.seek(16);
        assert_eq!(cursor.cursor, 8);

        cursor.seek(2);
        assert_eq!(cursor.cursor, 1);

        cursor.seek(15);
        assert_eq!(cursor.cursor, 8);

        let table = EventTable::empty();
        cursor = EventCursor::new(&table);
        cursor.seek(37);
        assert_eq!(cursor.cursor, 0);
    }

    #[test]
    fn test_step() {
        let table = make_dummy_table();
        let mut cursor = EventCursor::new(&table);
        cursor.seek(0);
        assert_eq!(cursor.cursor, 0);

        cursor.step();
        assert_eq!(cursor.cursor, 1);
        cursor.step();
        cursor.step();
        assert_eq!(cursor.cursor, 3);
        cursor.goto(255);
        assert_eq!(cursor.cursor, 255);
        cursor.step();
        assert_eq!(cursor.cursor, 255);
    }
}
