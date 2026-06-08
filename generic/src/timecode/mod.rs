#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::arithmetic_side_effects
)]

#[cfg(feature = "std")]
pub mod decoder;
#[cfg(feature = "std")]
pub mod encoder;
#[cfg(feature = "std")]
pub mod readwrite;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::string::String;

#[cfg(not(feature = "std"))]
use crate::str::StaticString;
use core::{fmt, hash::Hash};

/// SMPTE timecode frame rates
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::arbitrary_source_item_ordering)]
pub enum FrameRate {
    /// 23.976 fps (film transferred to NTSC, non-drop frame)
    Fps23976,
    /// 23.976 fps drop frame (drops 2 frames every 10 minutes)
    Fps23976DF,
    /// 24 fps (film)
    Fps24,
    /// 25 fps (PAL)
    Fps25,
    /// 29.97 fps (NTSC drop frame)
    Fps2997DF,
    /// 29.97 fps (NTSC non-drop frame)
    Fps2997NDF,
    /// 30 fps
    Fps30,
    /// 47.952 fps (cinema HFR, pulled-down from 48fps, non-drop frame)
    Fps47952,
    /// 47.952 fps drop frame (drops 4 frames every 10 minutes)
    Fps47952DF,
    /// 50 fps (PAL progressive)
    Fps50,
    /// 59.94 fps (NTSC progressive, non-drop frame)
    Fps5994,
    /// 59.94 fps drop frame (drops 4 frames every 10 minutes)
    Fps5994DF,
    /// 60 fps
    Fps60,
    /// 120 fps (high frame rate display / VR)
    Fps120,
}

impl FrameRate {
    /// Get the nominal frame rate as a float
    pub fn as_float(&self) -> f64 {
        match self {
            Self::Fps23976 | Self::Fps23976DF => 24000.0 / 1001.0,
            Self::Fps24 => 24.0,
            Self::Fps25 => 25.0,
            Self::Fps2997DF | Self::Fps2997NDF => 30000.0 / 1001.0,
            Self::Fps30 => 30.0,
            Self::Fps47952 | Self::Fps47952DF => 48000.0 / 1001.0,
            Self::Fps50 => 50.0,
            Self::Fps5994 | Self::Fps5994DF => 60000.0 / 1001.0,
            Self::Fps60 => 60.0,
            Self::Fps120 => 120.0,
        }
    }

    /// Get the exact frame rate as a rational (numerator, denominator)
    pub fn as_rational(&self) -> (u32, u32) {
        match self {
            Self::Fps23976 | Self::Fps23976DF => (24000, 1001),
            Self::Fps24 => (24, 1),
            Self::Fps25 => (25, 1),
            Self::Fps2997DF | Self::Fps2997NDF => (30000, 1001),
            Self::Fps30 => (30, 1),
            Self::Fps47952 | Self::Fps47952DF => (48000, 1001),
            Self::Fps50 => (50, 1),
            Self::Fps5994 | Self::Fps5994DF => (60000, 1001),
            Self::Fps60 => (60, 1),
            Self::Fps120 => (120, 1),
        }
    }

    /// The number of frames dropped per discontinuity point (every non-10th minute boundary).
    ///
    /// For 29.97 DF: 2 frames dropped per minute.
    /// For 23.976 DF: 2 frames dropped per minute (scaled from 29.97 × 24/30).
    /// For 47.952 DF: 4 frames dropped per minute (scaled from 29.97 × 48/30).
    /// For 59.94 DF: 4 frames dropped per minute (scaled from 29.97 × 60/30).
    pub fn drop_frames_per_minute(&self) -> u64 {
        match self {
            Self::Fps23976DF => 2,
            Self::Fps2997DF => 2,
            Self::Fps47952DF => 4,
            Self::Fps5994DF => 4,
            _ => 0,
        }
    }

    /// Get the number of frames per second (rounded)
    pub fn frames_per_second(&self) -> u32 {
        match self {
            Self::Fps23976 | Self::Fps23976DF => 24,
            Self::Fps24 => 24,
            Self::Fps25 => 25,
            Self::Fps2997DF | Self::Fps2997NDF => 30,
            Self::Fps30 => 30,
            Self::Fps47952 | Self::Fps47952DF => 48,
            Self::Fps50 => 50,
            Self::Fps5994 | Self::Fps5994DF => 60,
            Self::Fps60 => 60,
            Self::Fps120 => 120,
        }
    }

    /// Check if this is a drop frame rate
    pub fn is_drop_frame(&self) -> bool {
        matches!(
            self,
            Self::Fps2997DF | Self::Fps23976DF | Self::Fps5994DF | Self::Fps47952DF
        )
    }
}

/// Frame rate information for timecode (embedded in Timecode struct)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct FrameRateInfo {
    /// Drop frame flag
    pub drop_frame: bool,
    /// Frames per second (rounded)
    pub fps: u8,
}

/// Reconstruct a [`FrameRate`] enum from a [`FrameRateInfo`] embedded in a [`Timecode`].
///
/// This is a best-effort reconstruction: it cannot distinguish e.g. `Fps23976` from `Fps24`
/// (both have nominal fps=24) without the drop-frame flag, so it uses the drop-frame flag
/// and nominal fps to select the most common matching variant.
pub fn frame_rate_from_info(info: &FrameRateInfo) -> FrameRate {
    match (info.fps, info.drop_frame) {
        (24, true) => FrameRate::Fps23976DF,
        (24, false) => FrameRate::Fps23976, // Conservative: assume pull-down variant
        (25, _) => FrameRate::Fps25,
        (30, true) => FrameRate::Fps2997DF,
        (30, false) => FrameRate::Fps2997NDF,
        (48, true) => FrameRate::Fps47952DF,
        (48, false) => FrameRate::Fps47952,
        (50, _) => FrameRate::Fps50,
        (60, true) => FrameRate::Fps5994DF,
        (60, false) => FrameRate::Fps5994,
        (120, _) => FrameRate::Fps120,
        _ => FrameRate::Fps25, // Fallback
    }
}

/// SMPTE timecode structure
///
/// The `frame_count_cache` field stores the pre-computed total frame count
/// from midnight, avoiding recomputation on repeated calls to `to_frames()`.
/// It is excluded from equality comparison and serialization so it does not
/// affect timecode identity or wire format.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct Timecode {
    /// Hours (0-23)
    pub hours: u8,
    /// Minutes (0-59)
    pub minutes: u8,
    /// Seconds (0-59)
    pub seconds: u8,
    /// Frames (0 to frame_rate - 1)
    pub frames: u8,
    /// Frame rate
    pub frame_rate: FrameRateInfo,
    /// User bits (32 bits)
    pub user_bits: u32,
    /// Cached total frame count from midnight (computed at construction, excluded from Eq)
    frame_count_cache: u64,
}

impl PartialEq for Timecode {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours
            && self.minutes == other.minutes
            && self.seconds == other.seconds
            && self.frames == other.frames
            && self.frame_rate == other.frame_rate
            && self.user_bits == other.user_bits
    }
}

impl Eq for Timecode {}

impl Hash for Timecode {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.hours.hash(state);
        self.minutes.hash(state);
        self.seconds.hash(state);
        self.frames.hash(state);
        self.frame_rate.hash(state);
        self.user_bits.hash(state);
    }
}

impl PartialOrd for Timecode {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timecode {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.to_frames().cmp(&other.to_frames())
    }
}

impl Default for Timecode {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            frames: 0,
            frame_rate: FrameRateInfo {
                drop_frame: false,
                fps: 25,
            },
            user_bits: 0,
            frame_count_cache: 0,
        }
    }
}

impl Timecode {
    /// Compute total frames from midnight from the component fields.
    /// This is the canonical calculation used by the constructor and cache.
    fn compute_frames_from_fields(
        hours: u8,
        minutes: u8,
        seconds: u8,
        frames: u8,
        fps: u64,
        drop_frame: bool,
    ) -> u64 {
        let h = u64::from(hours);
        let m = u64::from(minutes);
        let s = u64::from(seconds);
        let f = u64::from(frames);

        let from_h = h.saturating_mul(3600).saturating_mul(fps);
        let from_m = m.saturating_mul(60).saturating_mul(fps);
        let from_s = s.saturating_mul(fps);
        let from_f = f;

        let mut total = from_h
            .saturating_add(from_m)
            .saturating_add(from_s)
            .saturating_add(from_f);

        if drop_frame {
            let drop_per_min = if fps >= 60 { 4u64 } else { 2u64 };
            let total_minutes = h.saturating_mul(60).saturating_add(m);
            let dropped_frames = drop_per_min
                .saturating_mul(total_minutes.saturating_sub(total_minutes.saturating_div(10)));
            total = total.saturating_sub(dropped_frames);
        }

        total
    }

    /// Create a new timecode
    pub fn new(
        hours: u8,
        minutes: u8,
        seconds: u8,
        frames: u8,
        frame_rate: FrameRate,
    ) -> Result<Self, TimecodeError> {
        let fps = frame_rate.frames_per_second() as u8;

        if hours > 23 {
            return Err(TimecodeError::InvalidHours);
        }
        if minutes > 59 {
            return Err(TimecodeError::InvalidMinutes);
        }
        if seconds > 59 {
            return Err(TimecodeError::InvalidSeconds);
        }
        if frames >= fps {
            return Err(TimecodeError::InvalidFrames);
        }

        // Validate drop frame rules
        if frame_rate.is_drop_frame() {
            let drop_count = frame_rate.drop_frames_per_minute() as u8;
            // drop_count frames are dropped at the start of each minute,
            // except minutes 0, 10, 20, 30, 40, 50.
            if seconds == 0 && frames < drop_count && !minutes.is_multiple_of(10) {
                return Err(TimecodeError::InvalidDropFrame);
            }
        }

        let drop_frame = frame_rate.is_drop_frame();
        let frame_count_cache = Self::compute_frames_from_fields(
            hours, minutes, seconds, frames, fps as u64, drop_frame,
        );

        Ok(Self {
            hours,
            minutes,
            seconds,
            frames,
            frame_rate: FrameRateInfo { fps, drop_frame },
            user_bits: 0,
            frame_count_cache,
        })
    }

    /// Parse a SMPTE timecode string.
    ///
    /// Accepts both "HH:MM:SS:FF" (non-drop frame, all colons) and
    /// "HH:MM:SS;FF" (drop frame, semicolon before frames).
    ///
    /// The `frame_rate` parameter determines the frame rate; the separator
    /// before the frame field determines whether drop-frame validation applies.
    ///
    /// # Errors
    ///
    /// Returns an error if the string format is invalid or component values
    /// are out of range.
    pub fn from_string(s: &str, frame_rate: FrameRate) -> Result<Self, TimecodeError> {
        let s = s.trim();
        // Minimum length: "00:00:00:00" = 11 chars
        if s.len() < 11 {
            return Err(TimecodeError::InvalidConfiguration);
        }

        // Split on colons and semicolons. Expect exactly 4 parts.
        let mut parts = s.split([':', ';']);

        let hours: u8 = parts
            .next()
            .ok_or(TimecodeError::InvalidConfiguration)?
            .parse()
            .map_err(|_| TimecodeError::InvalidHours)?;
        let minutes: u8 = parts
            .next()
            .ok_or(TimecodeError::InvalidConfiguration)?
            .parse()
            .map_err(|_| TimecodeError::InvalidMinutes)?;
        let seconds: u8 = parts
            .next()
            .ok_or(TimecodeError::InvalidConfiguration)?
            .parse()
            .map_err(|_| TimecodeError::InvalidSeconds)?;
        let frames: u8 = parts
            .next()
            .ok_or(TimecodeError::InvalidConfiguration)?
            .parse()
            .map_err(|_| TimecodeError::InvalidFrames)?;

        if parts.count() != 0 {
            return Err(TimecodeError::InvalidConfiguration);
        }

        Self::new(hours, minutes, seconds, frames, frame_rate)
    }

    /// Create a `Timecode` directly from raw fields without constructor validation.
    ///
    /// This is intended for internal use in parsers and codecs where the
    /// component values have already been validated by the caller.
    /// The `frame_count_cache` is computed automatically.
    pub fn from_raw_fields(
        hours: u8,
        minutes: u8,
        seconds: u8,
        frames: u8,
        fps: u8,
        drop_frame: bool,
        user_bits: u32,
    ) -> Self {
        let frame_count_cache = Self::compute_frames_from_fields(
            hours, minutes, seconds, frames, fps as u64, drop_frame,
        );
        Self {
            hours,
            minutes,
            seconds,
            frames,
            frame_rate: FrameRateInfo { fps, drop_frame },
            user_bits,
            frame_count_cache,
        }
    }

    /// Create timecode with user bits
    pub fn with_user_bits(mut self, user_bits: u32) -> Self {
        self.user_bits = user_bits;
        self
    }

    /// Convert to total frames since midnight.
    ///
    /// Returns the cached value computed at construction time — O(1).
    #[inline]
    pub fn to_frames(&self) -> u64 {
        self.frame_count_cache
    }

    /// Convert this timecode to elapsed wall-clock seconds as f64.
    ///
    /// For pull-down rates (23.976, 29.97, 47.952, 59.94) the exact rational
    /// frame rate is used so the result is frame-accurate.
    #[allow(clippy::cast_precision_loss)]
    pub fn to_seconds_f64(&self) -> f64 {
        let rate = frame_rate_from_info(&self.frame_rate);
        let (num, den) = rate.as_rational();
        // Use the exact rational to avoid floating-point drift at pull-down rates.
        self.frame_count_cache as f64 * den as f64 / num as f64
    }

    /// Create from total frames since midnight
    pub fn from_frames(frames: u64, frame_rate: FrameRate) -> Result<Self, TimecodeError> {
        let fps = frame_rate.frames_per_second() as u64;
        let mut remaining = frames;

        // Adjust for drop frame (generalised to support 2-frame and 4-frame drop rates)
        if frame_rate.is_drop_frame() {
            let drop_per_min = frame_rate.drop_frames_per_minute();
            let frames_per_minute = fps * 60 - drop_per_min;
            let frames_per_10_minutes = frames_per_minute * 9 + fps * 60;

            let ten_minute_blocks = remaining / frames_per_10_minutes;
            remaining += ten_minute_blocks * (drop_per_min * 9);

            let remaining_in_block = remaining % frames_per_10_minutes;
            if remaining_in_block >= fps * 60 {
                let extra_minutes = (remaining_in_block - fps * 60) / frames_per_minute;
                remaining += (extra_minutes + 1) * drop_per_min;
            }
        }

        let hours = (remaining / (fps * 3600)) as u8;
        remaining %= fps * 3600;
        let minutes = (remaining / (fps * 60)) as u8;
        remaining %= fps * 60;
        let seconds = (remaining / fps) as u8;
        let frame = (remaining % fps) as u8;

        Self::new(hours, minutes, seconds, frame, frame_rate)
    }

    /// Increment by one frame
    pub fn increment(&mut self) -> Result<(), TimecodeError> {
        self.frames += 1;

        if self.frames >= self.frame_rate.fps {
            self.frames = 0;
            self.seconds += 1;

            if self.seconds >= 60 {
                self.seconds = 0;
                self.minutes += 1;

                // Handle drop frame: skip frame numbers 0..drop_count at non-10th-minute boundaries
                if self.frame_rate.drop_frame && !self.minutes.is_multiple_of(10) {
                    let drop_count = if self.frame_rate.fps >= 60 { 4u8 } else { 2u8 };
                    self.frames = drop_count;
                }

                if self.minutes >= 60 {
                    self.minutes = 0;
                    self.hours += 1;

                    if self.hours >= 24 {
                        self.hours = 0;
                    }
                }
            }
        }

        // Recompute cache after mutation
        self.frame_count_cache = Self::compute_frames_from_fields(
            self.hours,
            self.minutes,
            self.seconds,
            self.frames,
            self.frame_rate.fps as u64,
            self.frame_rate.drop_frame,
        );

        Ok(())
    }

    /// Decrement by one frame
    pub fn decrement(&mut self) -> Result<(), TimecodeError> {
        if self.frames > 0 {
            self.frames -= 1;

            // Check if we're in a drop frame position
            let drop_count = if self.frame_rate.fps >= 60 { 4u8 } else { 2u8 };
            if self.frame_rate.drop_frame
                && self.seconds == 0
                && self.frames < drop_count
                && !self.minutes.is_multiple_of(10)
            {
                self.frames = self.frame_rate.fps - 1;
                if self.seconds > 0 {
                    self.seconds -= 1;
                } else {
                    self.seconds = 59;
                    if self.minutes > 0 {
                        self.minutes -= 1;
                    } else {
                        self.minutes = 59;
                        if self.hours > 0 {
                            self.hours -= 1;
                        } else {
                            self.hours = 23;
                        }
                    }
                }
            }
        } else if self.seconds > 0 {
            self.seconds -= 1;
            self.frames = self.frame_rate.fps - 1;
        } else {
            self.seconds = 59;
            self.frames = self.frame_rate.fps - 1;

            if self.minutes > 0 {
                self.minutes -= 1;
            } else {
                self.minutes = 59;
                if self.hours > 0 {
                    self.hours -= 1;
                } else {
                    self.hours = 23;
                }
            }
        }

        // Recompute cache after mutation
        self.frame_count_cache = Self::compute_frames_from_fields(
            self.hours,
            self.minutes,
            self.seconds,
            self.frames,
            self.frame_rate.fps as u64,
            self.frame_rate.drop_frame,
        );

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Arithmetic operators
// ---------------------------------------------------------------------------

impl core::ops::Add for Timecode {
    type Output = Result<Self, TimecodeError>;

    /// Add two timecodes by summing their total frame counts.
    ///
    /// The result uses the frame rate of `self`. The frame counts wrap at a
    /// 24-hour boundary.
    fn add(self, rhs: Self) -> Self::Output {
        let rate = frame_rate_from_info(&self.frame_rate);
        let fps = self.frame_rate.fps as u64;
        let frames_per_day = fps * 86_400;

        let sum = if frames_per_day > 0 {
            (self.frame_count_cache + rhs.frame_count_cache) % frames_per_day
        } else {
            self.frame_count_cache + rhs.frame_count_cache
        };

        Self::from_frames(sum, rate)
    }
}

impl core::ops::Sub for Timecode {
    type Output = Result<Self, TimecodeError>;

    /// Subtract `rhs` from `self` by frame count.
    ///
    /// The result uses the frame rate of `self`. Underflow wraps at a
    /// 24-hour boundary.
    fn sub(self, rhs: Self) -> Self::Output {
        let rate = frame_rate_from_info(&self.frame_rate);
        let fps = self.frame_rate.fps as u64;
        let frames_per_day = fps * 86_400;

        let result = if frames_per_day > 0 {
            if self.frame_count_cache >= rhs.frame_count_cache {
                self.frame_count_cache - rhs.frame_count_cache
            } else {
                // Wrap: borrow one 24-hour day
                frames_per_day - (rhs.frame_count_cache - self.frame_count_cache) % frames_per_day
            }
        } else {
            self.frame_count_cache.saturating_sub(rhs.frame_count_cache)
        };

        Self::from_frames(result, rate)
    }
}

impl core::ops::Add<u32> for Timecode {
    type Output = Result<Self, TimecodeError>;

    /// Add `rhs` frames to `self`, wrapping at a 24-hour boundary.
    ///
    /// The result uses the frame rate of `self`.
    fn add(self, rhs: u32) -> Self::Output {
        let rate = frame_rate_from_info(&self.frame_rate);
        let fps = self.frame_rate.fps as u64;
        let frames_per_day = fps * 86_400;

        let sum = if frames_per_day > 0 {
            (self.frame_count_cache + rhs as u64) % frames_per_day
        } else {
            self.frame_count_cache + rhs as u64
        };

        Self::from_frames(sum, rate)
    }
}

impl core::ops::Add<TimecodeOffset> for Timecode {
    type Output = Result<Self, TimecodeError>;

    fn add(self, rhs: TimecodeOffset) -> Self::Output {
        if rhs.is_negative {
            self - rhs.abs_time
        } else {
            self + rhs.abs_time
        }
    }
}

impl core::ops::Sub<TimecodeOffset> for Timecode {
    type Output = Result<Self, TimecodeError>;

    fn sub(self, rhs: TimecodeOffset) -> Self::Output {
        self + (-rhs)
    }
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

impl fmt::Display for Timecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let separator = if self.frame_rate.drop_frame { ';' } else { ':' };
        write!(
            f,
            "{:02}:{:02}:{:02}{}{:02}",
            self.hours, self.minutes, self.seconds, separator, self.frames
        )
    }
}

// ---------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------

/// Timecode reader trait
#[cfg(feature = "std")]
pub trait TimecodeReader {
    /// Read the next timecode from the source
    fn read_timecode(&mut self) -> Result<Option<Timecode>, TimecodeError>;

    /// Get the current frame rate
    fn frame_rate(&self) -> FrameRate;

    /// Check if the reader is synchronized
    fn is_synchronized(&self) -> bool;
}

/// Timecode writer trait
#[cfg(feature = "std")]
pub trait TimecodeWriter {
    /// Write a timecode to the output
    fn write_timecode(&mut self, timecode: &Timecode) -> Result<(), TimecodeError>;

    /// Get the current frame rate
    fn frame_rate(&self) -> FrameRate;

    /// Flush any buffered data
    fn flush(&mut self) -> Result<(), TimecodeError>;
}

// ---------------------------------------------------------------------------
// Offset type
// ---------------------------------------------------------------------------

/// Timecode offset; delta time
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimecodeOffset {
    /// Absolute time length and framerate of this offset
    pub abs_time: Timecode,
    /// Is this offset negative, i.e. goes backward (toward 0) from a point of time?
    pub is_negative: bool,
}

impl PartialOrd for TimecodeOffset {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimecodeOffset {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.is_negative == other.is_negative {
            let cmp = self.abs_time.cmp(&other.abs_time);
            if self.is_negative { cmp.reverse() } else { cmp }
        } else {
            if self.is_negative {
                core::cmp::Ordering::Less
            } else {
                core::cmp::Ordering::Greater
            }
        }
    }
}

impl TimecodeOffset {
    /// Create a new timecode offset
    pub fn new(abs_time: Timecode, is_negative: bool) -> Self {
        Self {
            abs_time,
            is_negative,
        }
    }

    /// Create a new timecode offset from raw field values
    pub fn from_raw_fields(
        negative: bool,
        hours: u8,
        minutes: u8,
        seconds: u8,
        frames: u8,
        frame_rate: FrameRate,
    ) -> Result<Self, TimecodeError> {
        Ok(Self {
            abs_time: Timecode::new(hours, minutes, seconds, frames, frame_rate)?,
            is_negative: negative,
        })
    }
}

impl core::ops::Add for TimecodeOffset {
    type Output = Result<Self, TimecodeError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_negative == rhs.is_negative {
            Ok(Self {
                abs_time: (self.abs_time + rhs.abs_time)?,
                is_negative: self.is_negative,
            })
        } else {
            Ok(Self {
                abs_time: (self.abs_time.max(rhs.abs_time) - self.abs_time.min(rhs.abs_time))?,
                is_negative: if self.abs_time > rhs.abs_time {
                    self.is_negative
                } else {
                    rhs.is_negative
                },
            })
        }
    }
}

impl core::ops::Neg for TimecodeOffset {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            abs_time: self.abs_time,
            is_negative: !self.is_negative,
        }
    }
}

impl core::ops::Sub for TimecodeOffset {
    type Output = Result<Self, TimecodeError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Timecode errors
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimecodeError {
    /// Invalid hours value
    InvalidHours,
    /// Invalid minutes value
    InvalidMinutes,
    /// Invalid seconds value
    InvalidSeconds,
    /// Invalid frames value
    InvalidFrames,
    /// Invalid drop frame timecode
    InvalidDropFrame,
    /// Sync word not found
    SyncNotFound,
    /// CRC error
    CrcError,
    /// Buffer too small
    BufferTooSmall,
    /// Invalid configuration
    InvalidConfiguration,
    /// IO error
    #[cfg(feature = "std")]
    IoError(String),
    /// IO error
    #[cfg(not(feature = "std"))]
    IoError(StaticString<32>),
    /// Not synchronized
    NotSynchronized,
}

impl fmt::Display for TimecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHours => write!(f, "Invalid hours value"),
            Self::InvalidMinutes => write!(f, "Invalid minutes value"),
            Self::InvalidSeconds => write!(f, "Invalid seconds value"),
            Self::InvalidFrames => write!(f, "Invalid frames value"),
            Self::InvalidDropFrame => write!(f, "Invalid drop frame timecode"),
            Self::SyncNotFound => write!(f, "Sync word not found"),
            Self::CrcError => write!(f, "CRC error"),
            Self::BufferTooSmall => write!(f, "Buffer too small"),
            Self::InvalidConfiguration => write!(f, "Invalid configuration"),
            #[cfg(feature = "std")]
            Self::IoError(e) => write!(f, "IO error: {}", e),
            #[cfg(not(feature = "std"))]
            Self::IoError(s) => write!(f, "IO error: {}", s.str()),
            Self::NotSynchronized => write!(f, "Not synchronized"),
        }
    }
}

impl core::error::Error for TimecodeError {}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::string::ToString;

    #[test]
    fn test_timecode_creation() {
        let tc = Timecode::new(1, 2, 3, 4, FrameRate::Fps25).expect("valid timecode");
        assert_eq!(tc.hours, 1);
        assert_eq!(tc.minutes, 2);
        assert_eq!(tc.seconds, 3);
        assert_eq!(tc.frames, 4);
    }

    #[test]
    fn test_timecode_display() {
        let tc = Timecode::new(1, 2, 3, 4, FrameRate::Fps25).expect("valid timecode");
        assert_eq!(tc.to_string(), "01:02:03:04");

        let tc_df = Timecode::new(1, 2, 3, 4, FrameRate::Fps2997DF).expect("valid timecode");
        assert_eq!(tc_df.to_string(), "01:02:03;04");
    }

    #[test]
    fn test_timecode_increment() {
        let mut tc = Timecode::new(0, 0, 0, 24, FrameRate::Fps25).expect("valid timecode");
        tc.increment().expect("increment should succeed");
        assert_eq!(tc.frames, 0);
        assert_eq!(tc.seconds, 1);
    }

    #[test]
    fn test_frame_rate() {
        assert_eq!(FrameRate::Fps25.as_float(), 25.0);
        assert!((FrameRate::Fps2997DF.as_float() - 29.97002997).abs() < 1e-6);
        assert!(FrameRate::Fps2997DF.is_drop_frame());
        assert!(!FrameRate::Fps2997NDF.is_drop_frame());
    }

    #[test]
    fn test_framerate_47952_and_120() {
        assert_eq!(FrameRate::Fps47952.frames_per_second(), 48);
        assert_eq!(FrameRate::Fps47952DF.frames_per_second(), 48);
        assert_eq!(FrameRate::Fps120.frames_per_second(), 120);
        assert!(!FrameRate::Fps47952.is_drop_frame());
        assert!(FrameRate::Fps47952DF.is_drop_frame());
        assert!(!FrameRate::Fps120.is_drop_frame());
        assert_eq!(FrameRate::Fps47952.as_rational(), (48000, 1001));
        assert_eq!(FrameRate::Fps120.as_rational(), (120, 1));
    }

    #[test]
    fn test_from_string_ndf() {
        let tc = Timecode::from_string("01:02:03:04", FrameRate::Fps25).expect("should parse");
        assert_eq!(tc.hours, 1);
        assert_eq!(tc.minutes, 2);
        assert_eq!(tc.seconds, 3);
        assert_eq!(tc.frames, 4);
    }

    #[test]
    fn test_from_string_df() {
        // Drop frame: semicolon before frames
        let tc = Timecode::from_string("01:02:03;04", FrameRate::Fps2997DF).expect("should parse");
        assert_eq!(tc.frames, 4);
        assert!(tc.frame_rate.drop_frame);
    }

    #[test]
    fn test_from_string_invalid_too_short() {
        assert!(Timecode::from_string("1:2:3:4", FrameRate::Fps25).is_err());
    }

    #[test]
    fn test_from_string_invalid_parts() {
        assert!(Timecode::from_string("01:02:03", FrameRate::Fps25).is_err());
    }

    #[test]
    fn test_to_seconds_f64_one_hour_25fps() {
        let tc = Timecode::new(1, 0, 0, 0, FrameRate::Fps25).expect("valid");
        let secs = tc.to_seconds_f64();
        assert!((secs - 3600.0).abs() < 1e-6);
    }

    #[test]
    fn test_to_seconds_f64_pull_down() {
        // 1 frame at 29.97 NDF = 1001/30000 seconds
        let tc = Timecode::new(0, 0, 0, 1, FrameRate::Fps2997NDF).expect("valid");
        let expected = 1001.0 / 30000.0;
        assert!((tc.to_seconds_f64() - expected).abs() < 1e-12);
    }

    #[test]
    fn test_ord_timecodes() {
        let tc1 = Timecode::new(0, 0, 0, 0, FrameRate::Fps25).expect("valid");
        let tc2 = Timecode::new(0, 0, 0, 1, FrameRate::Fps25).expect("valid");
        let tc3 = Timecode::new(1, 0, 0, 0, FrameRate::Fps25).expect("valid");
        assert!(tc1 < tc2);
        assert!(tc2 < tc3);
        assert!(tc1 < tc3);
        assert_eq!(tc1, tc1);
    }

    #[test]
    fn test_add_timecodes() {
        let tc1 = Timecode::new(0, 0, 1, 0, FrameRate::Fps25).expect("valid"); // 1s
        let tc2 = Timecode::new(0, 0, 2, 0, FrameRate::Fps25).expect("valid"); // 2s
        let result = (tc1 + tc2).expect("add should succeed");
        assert_eq!(result.seconds, 3);
        assert_eq!(result.frames, 0);
    }

    #[test]
    fn test_sub_timecodes() {
        let tc1 = Timecode::new(0, 0, 3, 0, FrameRate::Fps25).expect("valid"); // 3s
        let tc2 = Timecode::new(0, 0, 1, 0, FrameRate::Fps25).expect("valid"); // 1s
        let result = (tc1 - tc2).expect("sub should succeed");
        assert_eq!(result.seconds, 2);
        assert_eq!(result.frames, 0);
    }

    #[test]
    fn test_add_u32_frames() {
        // 0:00:00:00 + 25 frames = 0:00:01:00 at 25fps
        let tc = Timecode::new(0, 0, 0, 0, FrameRate::Fps25).expect("valid");
        let result = (tc + 25_u32).expect("add u32 should succeed");
        assert_eq!(result.seconds, 1);
        assert_eq!(result.frames, 0);

        // 23:59:59:24 + 1 frame wraps to 0:00:00:00
        let tc_near_end = Timecode::new(23, 59, 59, 24, FrameRate::Fps25).expect("valid");
        let wrapped = (tc_near_end + 1_u32).expect("wrap should succeed");
        assert_eq!(wrapped.hours, 0);
        assert_eq!(wrapped.minutes, 0);
        assert_eq!(wrapped.seconds, 0);
        assert_eq!(wrapped.frames, 0);
    }

    #[test]
    fn test_frame_count_cache_matches_recomputed() {
        let tc = Timecode::new(1, 23, 45, 12, FrameRate::Fps25).expect("valid");
        let expected: u64 = 3600 * 25 + 23 * 60 * 25 + 45 * 25 + 12;
        assert_eq!(tc.to_frames(), expected);
    }

    #[test]
    fn test_frame_count_cache_after_increment() {
        let mut tc = Timecode::new(0, 0, 0, 24, FrameRate::Fps25).expect("valid");
        let before = tc.to_frames();
        tc.increment().expect("ok");
        assert_eq!(tc.to_frames(), before + 1);
    }

    #[test]
    fn test_frame_rate_from_info() {
        let info = FrameRateInfo {
            fps: 25,
            drop_frame: false,
        };
        assert_eq!(frame_rate_from_info(&info), FrameRate::Fps25);

        let info_df = FrameRateInfo {
            fps: 30,
            drop_frame: true,
        };
        assert_eq!(frame_rate_from_info(&info_df), FrameRate::Fps2997DF);

        let info_120 = FrameRateInfo {
            fps: 120,
            drop_frame: false,
        };
        assert_eq!(frame_rate_from_info(&info_120), FrameRate::Fps120);
    }

    #[test]
    fn test_timecode_offset_add() {
        // Add one frame
        assert_eq!(
            TimecodeOffset::from_raw_fields(false, 0, 0, 4, 0, FrameRate::Fps25).expect("valid")
                + TimecodeOffset::from_raw_fields(false, 0, 0, 0, 1, FrameRate::Fps25)
                    .expect("valid"),
            TimecodeOffset::from_raw_fields(false, 0, 0, 4, 1, FrameRate::Fps25)
        );
        // Add negative one frame
        assert_eq!(
            TimecodeOffset::from_raw_fields(false, 0, 0, 4, 0, FrameRate::Fps25).expect("valid")
                + TimecodeOffset::from_raw_fields(true, 0, 0, 0, 1, FrameRate::Fps25)
                    .expect("valid"),
            TimecodeOffset::from_raw_fields(false, 0, 0, 3, 24, FrameRate::Fps25)
        );
        // Add negative multiple frames
        assert_eq!(
            TimecodeOffset::from_raw_fields(false, 0, 0, 4, 0, FrameRate::Fps25).expect("valid")
                + TimecodeOffset::from_raw_fields(true, 0, 0, 0, 15, FrameRate::Fps25)
                    .expect("valid"),
            TimecodeOffset::from_raw_fields(false, 0, 0, 3, 10, FrameRate::Fps25)
        );
    }

    #[test]
    fn test_timecode_offset_sub() {
        // Sub one frame
        assert_eq!(
            TimecodeOffset::from_raw_fields(false, 0, 0, 4, 0, FrameRate::Fps25).expect("valid")
                - TimecodeOffset::from_raw_fields(false, 0, 0, 0, 1, FrameRate::Fps25)
                    .expect("valid"),
            TimecodeOffset::from_raw_fields(false, 0, 0, 3, 24, FrameRate::Fps25)
        );
        // sub negative one frame
        assert_eq!(
            TimecodeOffset::from_raw_fields(false, 0, 0, 4, 0, FrameRate::Fps25).expect("valid")
                - TimecodeOffset::from_raw_fields(true, 0, 0, 0, 1, FrameRate::Fps25)
                    .expect("valid"),
            TimecodeOffset::from_raw_fields(false, 0, 0, 4, 1, FrameRate::Fps25)
        );
        // Sub negative multiple frames
        assert_eq!(
            TimecodeOffset::from_raw_fields(false, 0, 0, 4, 0, FrameRate::Fps25).expect("valid")
                - TimecodeOffset::from_raw_fields(true, 0, 0, 0, 15, FrameRate::Fps25)
                    .expect("valid"),
            TimecodeOffset::from_raw_fields(false, 0, 0, 4, 15, FrameRate::Fps25)
        );
    }

    #[test]
    fn test_timecode_offset() {
        // Sub one frame
        assert_eq!(
            Timecode::new(0, 0, 4, 0, FrameRate::Fps25).expect("valid")
                - TimecodeOffset::from_raw_fields(false, 0, 0, 0, 1, FrameRate::Fps25)
                    .expect("valid"),
            Timecode::new(0, 0, 3, 24, FrameRate::Fps25)
        );
        // sub negative one frame
        assert_eq!(
            Timecode::new(0, 0, 4, 0, FrameRate::Fps25).expect("valid")
                - TimecodeOffset::from_raw_fields(true, 0, 0, 0, 1, FrameRate::Fps25)
                    .expect("valid"),
            Timecode::new(0, 0, 4, 1, FrameRate::Fps25)
        );
        // Sub negative multiple frames
        assert_eq!(
            Timecode::new(0, 0, 4, 0, FrameRate::Fps25).expect("valid")
                - TimecodeOffset::from_raw_fields(true, 0, 0, 0, 15, FrameRate::Fps25)
                    .expect("valid"),
            Timecode::new(0, 0, 4, 15, FrameRate::Fps25)
        );
        // Sub wrapping
        assert_eq!(
            Timecode::new(0, 0, 4, 0, FrameRate::Fps25).expect("valid")
                - TimecodeOffset::from_raw_fields(false, 0, 3, 0, 0, FrameRate::Fps25)
                    .expect("valid"),
            Timecode::new(23, 57, 4, 0, FrameRate::Fps25)
        );
    }
}
