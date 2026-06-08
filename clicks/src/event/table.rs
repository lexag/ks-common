use crate::event::Event;
extern crate std;
use std::vec::Vec;

/// Table of events that occur in a specific cue.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct EventTable {
    table: std::vec::Vec<Event>,
}

impl EventTable {
    /// Create an empty EventTable.
    pub const fn empty() -> Self {
        Self { table: Vec::new() }
    }

    /// Get the event at idx, returning a null-event if idx is out of bounds.
    pub fn get(&self, idx: u8) -> Option<Event> {
        let e = self.table.get(idx as usize)?;
        if e.is_null() { None } else { Some(*e) }
    }

    /// Get a mut ref to the event at idx, returning a null-event if idx is out of bounds.
    pub fn get_mut(&mut self, idx: u8) -> Option<&mut Event> {
        let e = self.table.get_mut(idx as usize)?;
        if e.is_null() { None } else { Some(e) }
    }

    /// Set the event value at idx.
    /// Returns boolean success.
    pub fn set(&mut self, idx: u8, event: Event) -> bool {
        if idx > self.table.len() as u8 {
            false
        } else if idx == self.table.len() as u8 {
            self.table.push(event);
            true
        } else {
            self.table[idx as usize] = event;
            true
        }
    }

    /// Sort this table in order of event location.
    /// Unpopulated events (null-events) will be at the end of the table
    pub fn sort(&mut self) {
        self.table.sort_by_key(|e: &Event| e.location);
    }

    /// Add an event to this table.
    /// The table will be sorted to place the new event in order.
    /// This may break existing [crate::eventcursor::EventCursor]s
    pub fn push(&mut self, event: Event) -> bool {
        self.table.push(event);
        self.sort();
        true
    }

    /// Remove an event from the table by index.
    /// The event is returned and the table is resorted to fill the gap
    pub fn pop(&mut self, idx: u8) -> Option<Event> {
        if idx as usize >= self.table.len() {
            return None;
        }
        let e = self.table.remove(idx as usize);
        self.sort();
        Some(e)
    }

    /// Get number of events in this table
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Is this table empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get all events located at the given beat
    pub fn get_at_location(&self, location: u16) -> Vec<Event> {
        self.table
            .iter()
            .filter(|&e| e.location == location)
            .copied()
            .collect()
    }

    /// Shift events at beat >= X later by Y beats
    pub fn shift_events(&mut self, start_location: u16, change: i16) {
        let breakpoint_idx = self.idx_of_location(start_location);
        for event in &mut self.table[breakpoint_idx as usize..] {
            event.location = (event.location as i16).saturating_add(change).max(0) as u16
        }
    }

    /// Get index in table of a beat location, or of the closest larger beat
    /// location if no event occurs on the given location
    fn idx_of_location(&self, location: u16) -> u8 {
        let mut idx = 0;
        while let Some(event) = self.get(idx)
            && event.location < location
        {
            idx += 1;
        }
        idx
    }

    /// Get an immutable iterator over all events in this table
    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.table.iter()
    }

    /// Get a mutable iterator over all events in this table
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Event> {
        self.table.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventDescription;

    #[test]
    fn test_sort() {
        let mut t = EventTable::empty();
        t.push(Event::new(12, EventDescription::TimecodeStopEvent));
        t.push(Event::new(10, EventDescription::TimecodeStopEvent));
        t.push(Event::new(4, EventDescription::TimecodeStopEvent));
        t.push(Event::new(16, EventDescription::TimecodeStopEvent));
        assert_eq!(t.get(0).expect("").location, 4);
        assert_eq!(t.get(1).expect("").location, 10);
        assert_eq!(t.get(2).expect("").location, 12);
        assert_eq!(t.get(3).expect("").location, 16);
        assert_eq!(t.get(4), None);
    }
}
