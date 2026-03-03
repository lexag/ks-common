use core::fmt::{Display, Formatter, Result};

/// SMPTE timecode properties
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct TimecodeProperties {
    /// Frame numbers 0 and 1 are skipped during the first second of every minute, except multiples
    /// of 10 minutes. This converts 30 frames/second time code to the 29.97 frames/second NTSC
    /// standard. (from Wikipedia, "Linear timecode")
    pub drop_frame: bool,
    /// Set to 1 if the time code is synchronized to a color video signal. The frame number modulo
    /// 2 (for NTSC and SECAM) or modulo 4 (for PAL) should be preserved across cuts in order to
    /// avoid phase jumps in the chrominance subcarrier. (from Wikipedia, "Linear timecode")
    pub color_framing: bool,
    /// Binary group flag; user bit format
    pub user_bit_format: TimecodeUserBitFormat,
    /// Indicates that the time code is synchronized to an external clock. False indicates the time
    /// origin is arbitrary. In practice, this means "ignore the timestamp in an LTC event, and run
    /// LTC from device time instead".
    pub use_wall_time: bool,
    /// 32 user bits
    pub user_bits: [u8; 4],
    /// Frame number offset
    pub frame_offset: u8,
}

/// SMPTE BFG, binary group flag.
/// Indicates the format of user bits
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum TimecodeUserBitFormat {
    /// No (or unspecified) format
    #[default]
    Unspecified = 0,
    /// Date and timezone, according to SMPTE 309M
    DateTimezone = 1,
    /// Four 8-bit characters, transmitted little-endian
    EightBitLittleEndian = 2,
    /// Reserved and unused
    Reserved11 = 3,
}

/// A SMPTE LTC timestamp, including frame rate.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, Copy, Eq)]
pub struct TimecodeInstant {
    /// Frame rate in frames per second
    pub frame_rate: u8,
    /// Current number of hours
    pub h: i8,
    /// Current number of minutes
    pub m: i8,
    /// Current number of seconds
    pub s: i8,
    /// Current number of frames
    pub f: i8,
    /// Current progress through the current frame, 0-65536
    pub frame_progress: u16,
}

impl PartialEq for TimecodeInstant {
    fn eq(&self, other: &TimecodeInstant) -> bool {
        self.f == other.f && self.s == other.s && self.m == other.m && self.h == other.h
    }
}

impl Ord for TimecodeInstant {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let h_comp = self.h.cmp(&other.h);
        let m_comp = self.m.cmp(&other.m);
        let s_comp = self.s.cmp(&other.s);
        let f_comp = self.f.cmp(&other.f);
        if h_comp != core::cmp::Ordering::Equal {
            h_comp
        } else if m_comp != core::cmp::Ordering::Equal {
            m_comp
        } else if s_comp != core::cmp::Ordering::Equal {
            s_comp
        } else if f_comp != core::cmp::Ordering::Equal {
            f_comp
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for TimecodeInstant {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for TimecodeInstant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}:{:0>2}",
            self.h, self.m, self.s, self.f
        )
    }
}

impl TimecodeInstant {
    /// Create a 00:00:00:00 timecode instant with the given frame rate
    pub fn new(frame_rate: u8) -> TimecodeInstant {
        TimecodeInstant {
            frame_rate,
            ..Default::default()
        }
    }

    /// Add an amount of frame progress to the current timestamp.
    /// If this reaches the end of the frame, f increments and the remaining progress adds to the
    /// next frame.
    pub fn add_progress(&mut self, progress: u16) {
        let prog_of = self.frame_progress as u32 + progress as u32;
        self.frame_progress = (prog_of % 65536) as u16;
        if prog_of >= 65536 {
            self.f += 1
        }
        self.propagate();
    }

    /// Add an amount of microseconds to this timestamp.
    pub fn add_us(&mut self, time_us: u64) {
        let us_per_frame = 1_000_000 / self.frame_rate as u64;
        let frames = time_us / us_per_frame;
        let subframe_us = time_us % us_per_frame;
        let progress = subframe_us * 65536 / us_per_frame;
        self.h += (frames / self.frame_rate as u64 / 60 / 60) as i8;
        self.m += (frames / self.frame_rate as u64 / 60 % 60) as i8;
        self.s += (frames / self.frame_rate as u64 % 60) as i8;
        self.f += (frames % self.frame_rate as u64) as i8;
        self.add_progress(progress.try_into().unwrap_or_default());
    }
    /// Subtract an amount of microseconds from this timestamp.
    pub fn sub_us(&mut self, time_us: u64) {
        let mut tci = TimecodeInstant::new(self.frame_rate);
        tci.add_us(time_us);
        self.sub(tci);
    }

    /// Subtract another [TimecodeInstant] from this timestamp.
    pub fn sub(&mut self, other: TimecodeInstant) {
        self.f -= other.f;
        self.s -= other.s;
        self.m -= other.m;
        self.h -= other.h;
        self.propagate();
    }

    /// Set the current timestamp
    pub fn set_time(&mut self, h: usize, m: usize, s: usize, f: usize) {
        self.h = h as i8;
        self.m = m as i8;
        self.s = s as i8;
        self.f = f as i8;
        self.frame_progress = 0;
    }

    // propagate changes to f into the other values
    fn propagate(&mut self) {
        self.s += self.f / self.frame_rate as i8;
        self.f %= self.frame_rate as i8;
        self.f += self.frame_rate as i8;
        self.f %= self.frame_rate as i8;
        self.m += self.s / 60;
        self.s %= 60;
        self.h += self.m / 60;
        self.m %= 60;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_progress() {
        let time_const = TimecodeInstant::new(25);
        let mut time = time_const;
        time.add_progress(0);
        assert_eq!(time, time_const);
        time.add_progress(u16::MAX);
        time.add_progress(1);
        assert_eq!(time.frame_progress, 0);
        assert_eq!(time.f, 1);
        time.add_progress(1);
        assert_eq!(time.frame_progress, 1);
    }
    #[test]
    fn add_sub_identity() {
        let time_const = TimecodeInstant::new(25);
        for i in (0..36000 * 1000000).step_by(12345678) {
            let mut time = time_const;
            time.add_us(i);
            time.sub_us(i);
            assert_eq!(time, time_const, "Failed with {}us ({} s)", i, i / 1000000);
        }
    }

    #[test]
    fn add_us() {
        const US_PER_FRAME: u64 = 1_000_000 / 25;
        let mut time = TimecodeInstant::new(25);
        time.add_us(US_PER_FRAME);
        assert_eq!(time.f, 1);
        assert_eq!(time.frame_progress, 0);
        time.add_us(US_PER_FRAME * 26);
        assert_eq!(time.f, 2);
        assert_eq!(time.s, 1);
        assert_eq!(time.frame_progress, 0);
        time.add_us(US_PER_FRAME / 2);
        assert_eq!(time.f, 2);
        assert_eq!(time.frame_progress, 65535 / 2 + 1);

        time.set_time(0, 0, 0, 0);
        time.add_us(US_PER_FRAME * 25 * 3605);
        assert_eq!(time.h, 1);
        assert_eq!(time.m, 0);
        assert_eq!(time.s, 5);
        assert_eq!(time.f, 0);
    }
}
