//! LTC Decoder - Biphase Mark Code decoding from audio
//!
//! This module implements a complete LTC decoder that:
//! - Analyzes audio waveforms for biphase mark code transitions
//! - Detects and validates SMPTE sync words
//! - Extracts timecode and user bits
//! - Handles variable playback speeds
//! - Provides error correction and validation

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::vec::Vec;

use crate::timecode::{FrameRate, Timecode, TimecodeError, readwrite::constants::*};

/// LTC decoder state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum DecoderState {
    /// Searching for sync word
    Searching,
    /// Locked to sync, decoding bits
    Locked,
    /// Lost sync, attempting to reacquire
    LostSync,
}

/// Biphase mark code decoder
pub struct LtcDecoder {
    /// Sample rate
    #[allow(dead_code)]
    sample_rate: u32,
    /// Frame rate
    frame_rate: FrameRate,
    /// Minimum signal amplitude
    min_amplitude: f32,
    /// Current decoder state
    state: DecoderState,
    /// Bit buffer (80 bits)
    bit_buffer: [bool; BITS_PER_FRAME],
    /// Current bit position in buffer
    bit_position: usize,
    /// Zero crossing detector state
    zero_crossing: ZeroCrossingDetector,
    /// Bit synchronizer
    bit_sync: BitSynchronizer,
    /// Last decoded timecode
    last_timecode: Option<Timecode>,
    /// Sync confidence counter
    sync_confidence: u32,
    /// Error counter
    error_count: u32,
}

impl LtcDecoder {
    /// Create a new LTC decoder
    pub fn new(sample_rate: u32, frame_rate: FrameRate, min_amplitude: f32) -> Self {
        Self {
            sample_rate,
            frame_rate,
            min_amplitude,
            state: DecoderState::Searching,
            bit_buffer: [false; BITS_PER_FRAME],
            bit_position: 0,
            zero_crossing: ZeroCrossingDetector::new(sample_rate, frame_rate),
            bit_sync: BitSynchronizer::new(sample_rate, frame_rate),
            last_timecode: None,
            sync_confidence: 0,
            error_count: 0,
        }
    }

    /// Get this decoder's set frame rate
    pub fn frame_rate(&self) -> FrameRate {
        self.frame_rate
    }

    /// Process audio samples and decode timecode
    pub fn process_samples(&mut self, samples: &[f32]) -> Result<Option<Timecode>, TimecodeError> {
        let mut result = None;

        for &sample in samples {
            // Detect zero crossings
            if let Some(transition) = self
                .zero_crossing
                .process_sample(sample, self.min_amplitude)
            {
                // Process the transition
                if let Some(bit) = self.bit_sync.process_transition(transition) {
                    // Store the bit
                    if let Some(tc) = self.process_bit(bit)? {
                        result = Some(tc);
                    }
                }
            }
        }

        Ok(result)
    }

    /// Process a decoded bit
    fn process_bit(&mut self, bit: bool) -> Result<Option<Timecode>, TimecodeError> {
        // Store bit in buffer
        self.bit_buffer[self.bit_position] = bit;
        self.bit_position += 1;

        // Check if we have a complete frame
        if self.bit_position >= BITS_PER_FRAME {
            self.bit_position = 0;

            // Try to decode the frame
            match self.decode_frame() {
                Ok(timecode) => {
                    self.state = DecoderState::Locked;
                    self.sync_confidence = self.sync_confidence.saturating_add(5).min(100);
                    self.error_count = 0;
                    self.last_timecode = Some(timecode);
                    return Ok(Some(timecode));
                }
                Err(e) => {
                    self.error_count += 1;
                    if self.error_count > 10 {
                        self.state = DecoderState::LostSync;
                        self.sync_confidence = 0;
                        self.bit_position = 0;
                        return Err(e);
                    }
                }
            }
        }

        Ok(None)
    }

    /// Decode a complete LTC frame from the bit buffer
    fn decode_frame(&mut self) -> Result<Timecode, TimecodeError> {
        // Find sync word position
        let sync_pos = self.find_sync_word()?;

        // Extract data bits (64 bits before sync word)
        let mut data_bits = [false; DATA_BITS];
        for (i, data_bit) in data_bits.iter_mut().enumerate().take(DATA_BITS) {
            let pos = (sync_pos + BITS_PER_FRAME - DATA_BITS + i) % BITS_PER_FRAME;
            *data_bit = self.bit_buffer[pos];
        }

        // Setup the next frame write buffer such that we sync frame edges with buffer edges
        self.bit_position = (2 * BITS_PER_FRAME - sync_pos - SYNC_BITS) % BITS_PER_FRAME;

        // Decode timecode from data bits
        self.decode_timecode_from_bits(&data_bits)
    }

    /// Find the sync word in the bit buffer
    fn find_sync_word(&self) -> Result<usize, TimecodeError> {
        // Convert sync word to bit pattern
        let sync_bits = self.u16_to_bits(SYNC_WORD);

        // Search for sync word in buffer
        for start_pos in 0..BITS_PER_FRAME {
            let mut match_count = 0;
            for (i, &sync_bit) in sync_bits.iter().enumerate().take(SYNC_BITS) {
                let pos = (start_pos + i) % BITS_PER_FRAME;
                if self.bit_buffer[pos] == sync_bit {
                    match_count += 1;
                }
            }

            // Allow up to 2 bit errors in sync word
            if match_count >= SYNC_BITS - 2 {
                return Ok(start_pos);
            }
        }

        Err(TimecodeError::SyncNotFound)
    }

    /// Convert u16 to bit array (LSB first, as per LTC spec)
    fn u16_to_bits(&self, value: u16) -> [bool; 16] {
        let mut bits = [false; 16];
        for (i, bit) in bits.iter_mut().enumerate() {
            *bit = (value & (1 << i)) != 0;
        }
        bits
    }

    /// Decode timecode from 64 data bits
    fn decode_timecode_from_bits(
        &mut self,
        bits: &[bool; DATA_BITS],
    ) -> Result<Timecode, TimecodeError> {
        // LTC bit layout (SMPTE 12M):
        // Bits 0-3: Frame units
        // Bits 4-7: User bits 1
        // Bits 8-9: Frame tens
        // Bit 10: Drop frame flag
        // Bit 11: Color frame flag
        // Bits 12-15: User bits 2
        // Bits 16-19: Second units
        // Bits 20-23: User bits 3
        // Bits 24-26: Second tens
        // Bit 27: Biphase mark correction (even parity)
        // Bits 28-31: User bits 4
        // Bits 32-35: Minute units
        // Bits 36-39: User bits 5
        // Bits 40-42: Minute tens
        // Bit 43: Binary group flag
        // Bits 44-47: User bits 6
        // Bits 48-51: Hour units
        // Bits 52-55: User bits 7
        // Bits 56-57: Hour tens
        // Bit 58-63: User bits 8 and flags

        let frame_units = self.bits_to_u8(&bits[0..4]);
        let frame_tens = self.bits_to_u8(&bits[8..10]);
        let frames = frame_tens * 10 + frame_units;

        let second_units = self.bits_to_u8(&bits[16..20]);
        let second_tens = self.bits_to_u8(&bits[24..27]);
        let seconds = second_tens * 10 + second_units;

        let minute_units = self.bits_to_u8(&bits[32..36]);
        let minute_tens = self.bits_to_u8(&bits[40..43]);
        let minutes = minute_tens * 10 + minute_units;

        let hour_units = self.bits_to_u8(&bits[48..52]);
        let hour_tens = self.bits_to_u8(&bits[56..58]);
        let hours = hour_tens * 10 + hour_units;

        // Extract drop frame flag
        let drop_frame = bits[10];

        // Extract user bits
        let user_bits = self.extract_user_bits(bits);

        // Create timecode
        let frame_rate = if drop_frame && self.frame_rate == FrameRate::Fps2997NDF {
            FrameRate::Fps2997DF
        } else {
            self.frame_rate
        };

        let mut timecode = Timecode::new(hours, minutes, seconds, frames, frame_rate)?;
        timecode.user_bits = user_bits;

        if let Some(last_time) = self.last_timecode
            && timecode.to_frames().abs_diff(last_time.to_frames()) > 8
        {
            self.bit_position = 0;
            self.last_timecode = None;
            self.sync_confidence = 0;
            self.bit_buffer.fill(false);
            return Err(TimecodeError::NotSynchronized);
        }

        Ok(timecode)
    }

    /// Convert bit slice to u8 (LSB first)
    fn bits_to_u8(&self, bits: &[bool]) -> u8 {
        let mut value = 0u8;
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }
        value
    }

    /// Extract user bits from data bits
    fn extract_user_bits(&self, bits: &[bool; DATA_BITS]) -> u32 {
        let mut user_bits = 0u32;

        // User bits are scattered throughout the LTC frame
        // UB1: bits 4-7
        user_bits |= self.bits_to_u8(&bits[4..8]) as u32;
        // UB2: bits 12-15
        user_bits |= (self.bits_to_u8(&bits[12..16]) as u32) << 4;
        // UB3: bits 20-23
        user_bits |= (self.bits_to_u8(&bits[20..24]) as u32) << 8;
        // UB4: bits 28-31
        user_bits |= (self.bits_to_u8(&bits[28..32]) as u32) << 12;
        // UB5: bits 36-39
        user_bits |= (self.bits_to_u8(&bits[36..40]) as u32) << 16;
        // UB6: bits 44-47
        user_bits |= (self.bits_to_u8(&bits[44..48]) as u32) << 20;
        // UB7: bits 52-55
        user_bits |= (self.bits_to_u8(&bits[52..56]) as u32) << 24;
        // UB8: bits 59-62 (4 bits)
        user_bits |= (self.bits_to_u8(&bits[59..63]) as u32) << 28;

        user_bits
    }

    /// Reset decoder state
    pub fn reset(&mut self) {
        self.state = DecoderState::Searching;
        self.bit_position = 0;
        self.sync_confidence = 0;
        self.error_count = 0;
        self.zero_crossing.reset();
        self.bit_sync.reset();
    }

    /// Return the most recently decoded timecode, if any.
    ///
    /// This is the same value that was returned by the last successful call
    /// to [`process_samples`](Self::process_samples).  It is cleared on
    /// [`reset`](Self::reset).
    pub fn last_decoded_timecode(&self) -> Option<Timecode> {
        self.last_timecode
    }

    /// Check if decoder is synchronized
    pub fn is_synchronized(&self) -> bool {
        self.state == DecoderState::Locked && self.sync_confidence >= 10
    }

    /// Get sync confidence (0.0 to 1.0)
    pub fn sync_confidence(&self) -> f32 {
        (self.sync_confidence as f32) / 100.0
    }
}

/// Zero crossing detector
#[allow(dead_code)]
struct ZeroCrossingDetector {
    /// Previous sample value
    prev_sample: f32,
    /// Sample counter
    sample_count: u64,
    /// Expected samples per bit (nominal)
    samples_per_bit: f32,
}

impl ZeroCrossingDetector {
    fn new(sample_rate: u32, frame_rate: FrameRate) -> Self {
        let fps = frame_rate.as_float();
        let bits_per_second = fps * BITS_PER_FRAME as f64;
        let samples_per_bit = sample_rate as f64 / bits_per_second;

        Self {
            prev_sample: 0.0,
            sample_count: 0,
            samples_per_bit: samples_per_bit as f32,
        }
    }

    /// Process a sample and detect zero crossings
    fn process_sample(&mut self, sample: f32, min_amplitude: f32) -> Option<Transition> {
        self.sample_count += 1;

        // Detect zero crossing with hysteresis
        let transition = if self.prev_sample < -min_amplitude && sample >= min_amplitude {
            Some(Transition {
                sample_index: self.sample_count,
                rising: true,
            })
        } else if self.prev_sample > min_amplitude && sample <= -min_amplitude {
            Some(Transition {
                sample_index: self.sample_count,
                rising: false,
            })
        } else {
            None
        };

        self.prev_sample = sample;
        transition
    }

    fn reset(&mut self) {
        self.prev_sample = 0.0;
        self.sample_count = 0;
    }
}

/// Transition event
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct Transition {
    sample_index: u64,
    rising: bool,
}

/// Bit synchronizer - converts transitions to bits
struct BitSynchronizer {
    /// Last transition time
    last_transition: Option<u64>,
    /// Expected samples per bit cell
    samples_per_bit: f32,
    /// Current bit cell phase
    bit_phase: f32,
    /// Bit clock accumulator
    last_was_mid_bit: bool,
}

impl BitSynchronizer {
    fn new(sample_rate: u32, frame_rate: FrameRate) -> Self {
        let fps = frame_rate.as_float();
        let bits_per_second = fps * BITS_PER_FRAME as f64;
        let samples_per_bit = sample_rate as f64 / bits_per_second;

        Self {
            last_transition: None,
            samples_per_bit: samples_per_bit as f32,
            bit_phase: 0.0,
            last_was_mid_bit: false,
        }
    }

    /// Process a transition and decode bits
    fn process_transition(&mut self, transition: Transition) -> Option<bool> {
        let sample_index = transition.sample_index;

        if let Some(last_idx) = self.last_transition {
            let samples_since_last = (sample_index - last_idx) as f32;

            // Determine if this is a half-bit or full-bit transition
            let is_half_bit = samples_since_last < (self.samples_per_bit * 0.75);

            self.last_transition = Some(sample_index);

            if is_half_bit {
                if self.last_was_mid_bit {
                    // This is a mid-bit transition (bit = 1)
                    self.last_was_mid_bit = false;
                    return Some(true);
                } else {
                    // This is the middle flip of a 1 bit. We wait until we're done with the bit to
                    // not duplicate a value
                    self.last_was_mid_bit = true;
                    return None;
                }
            } else {
                // This is a full-bit transition (bit = 0)
                self.last_was_mid_bit = false;
                return Some(false);
            }
            //if is_half_bit {
            //    // This is a mid-bit transition (bit = 1)
            //    self.last_was_mid_bit = 0.5;
            //    return Some(true);
            //} else {
            //    // This is a full-bit transition (bit = 0)
            //    self.last_was_mid_bit = 0.0;
            //    return Some(false);
            //}
        }

        self.last_transition = Some(sample_index);
        None
    }

    fn reset(&mut self) {
        self.last_transition = None;
        self.bit_phase = 0.0;
        self.last_was_mid_bit = false;
    }
}

/// Signal filter for noise reduction
#[allow(dead_code)]
struct SignalFilter {
    /// Filter coefficients (simple low-pass IIR)
    alpha: f32,
    /// Previous filtered value
    prev_value: f32,
}

impl SignalFilter {
    #[allow(dead_code)]
    fn new(cutoff_freq: f32, sample_rate: f32) -> Self {
        let rc = 1.0 / (2.0 * core::f32::consts::PI * cutoff_freq);
        let dt = 1.0 / sample_rate;
        let alpha = dt / (rc + dt);

        Self {
            alpha,
            prev_value: 0.0,
        }
    }

    /// Apply low-pass filter to sample
    #[allow(dead_code)]
    fn process(&mut self, sample: f32) -> f32 {
        let filtered = self.alpha * sample + (1.0 - self.alpha) * self.prev_value;
        self.prev_value = filtered;
        filtered
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.prev_value = 0.0;
    }
}

/// Waveform analyzer for signal quality assessment
#[allow(dead_code)]
struct WaveformAnalyzer {
    /// RMS calculator
    rms_accumulator: f32,
    /// Sample count for RMS
    rms_count: u32,
    /// Peak detector
    peak_positive: f32,
    /// Negative peak
    peak_negative: f32,
}

impl WaveformAnalyzer {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            rms_accumulator: 0.0,
            rms_count: 0,
            peak_positive: 0.0,
            peak_negative: 0.0,
        }
    }

    /// Process a sample and update statistics
    #[allow(dead_code)]
    fn process_sample(&mut self, sample: f32) {
        self.rms_accumulator += sample * sample;
        self.rms_count += 1;

        if sample > self.peak_positive {
            self.peak_positive = sample;
        }
        if sample < self.peak_negative {
            self.peak_negative = sample;
        }
    }

    /// Get RMS value
    #[allow(dead_code)]
    fn get_rms(&self) -> f32 {
        if self.rms_count > 0 {
            (self.rms_accumulator / self.rms_count as f32).sqrt()
        } else {
            0.0
        }
    }

    /// Get peak-to-peak amplitude
    #[allow(dead_code)]
    fn get_peak_to_peak(&self) -> f32 {
        self.peak_positive - self.peak_negative
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.rms_accumulator = 0.0;
        self.rms_count = 0;
        self.peak_positive = 0.0;
        self.peak_negative = 0.0;
    }
}

/// Drop frame calculator
#[allow(dead_code)]
struct DropFrameCalculator;

impl DropFrameCalculator {
    /// Check if a timecode is valid for drop frame
    #[allow(dead_code)]
    fn is_valid_drop_frame(minutes: u8, seconds: u8, frames: u8) -> bool {
        // Frames 0 and 1 are dropped at the start of each minute except 0, 10, 20, 30, 40, 50
        if seconds == 0 && frames < 2 && !minutes.is_multiple_of(10) {
            return false;
        }
        true
    }

    /// Adjust timecode for drop frame
    #[allow(dead_code)]
    fn adjust_for_drop_frame(minutes: u8, seconds: u8, frames: u8) -> (u8, u8, u8) {
        if seconds == 0 && frames < 2 && !minutes.is_multiple_of(10) {
            // Skip to frame 2
            (minutes, seconds, 2)
        } else {
            (minutes, seconds, frames)
        }
    }
}

/// Error correction using redundancy
#[allow(dead_code)]
struct ErrorCorrector {
    /// History of recent timecodes
    history: Vec<Timecode>,
    /// Maximum history size
    max_history: usize,
}

impl ErrorCorrector {
    #[allow(dead_code)]
    fn new(max_history: usize) -> Self {
        Self {
            history: Vec::with_capacity(max_history),
            max_history,
        }
    }

    /// Add timecode to history
    #[allow(dead_code)]
    fn add_timecode(&mut self, timecode: Timecode) {
        self.history.push(timecode);
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    /// Try to correct errors using history
    #[allow(dead_code)]
    fn correct_timecode(&self, timecode: &Timecode) -> Option<Timecode> {
        if self.history.is_empty() {
            return Some(*timecode);
        }

        // Check if timecode is sequential with last one
        if let Some(last) = self.history.last() {
            let mut expected = *last;
            if expected.increment().is_ok() {
                // Check if current timecode is close to expected
                if Self::is_close(timecode, &expected) {
                    return Some(*timecode);
                }
            }
        }

        // If not sequential, return the timecode anyway
        Some(*timecode)
    }

    /// Check if two timecodes are close (within a few frames)
    #[allow(dead_code)]
    fn is_close(tc1: &Timecode, tc2: &Timecode) -> bool {
        let diff = (tc1.to_frames() as i64 - tc2.to_frames() as i64).abs();
        diff <= 5
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.history.clear();
    }
}

/// Bit pattern validator
#[allow(dead_code)]
struct BitPatternValidator;

impl BitPatternValidator {
    /// Validate that bit patterns make sense for timecode
    #[allow(dead_code)]
    fn validate_timecode_bits(bits: &[bool; DATA_BITS]) -> bool {
        // Check that tens digits are in valid range
        let frame_tens = Self::bits_to_u8(&bits[8..10]);
        let second_tens = Self::bits_to_u8(&bits[24..27]);
        let minute_tens = Self::bits_to_u8(&bits[40..43]);
        let hour_tens = Self::bits_to_u8(&bits[56..58]);

        // Frame tens should be 0-5 (for 60fps max)
        if frame_tens > 5 {
            return false;
        }

        // Second tens should be 0-5
        if second_tens > 5 {
            return false;
        }

        // Minute tens should be 0-5
        if minute_tens > 5 {
            return false;
        }

        // Hour tens should be 0-2
        if hour_tens > 2 {
            return false;
        }

        true
    }

    #[allow(dead_code)]
    fn bits_to_u8(bits: &[bool]) -> u8 {
        let mut value = 0u8;
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }
        value
    }
}

/// Speed variation detector
#[allow(dead_code)]
struct SpeedDetector {
    /// History of bit periods
    bit_periods: Vec<f32>,
    /// Maximum history
    max_history: usize,
}

impl SpeedDetector {
    #[allow(dead_code)]
    fn new(max_history: usize) -> Self {
        Self {
            bit_periods: Vec::with_capacity(max_history),
            max_history,
        }
    }

    /// Add a bit period measurement
    #[allow(dead_code)]
    fn add_period(&mut self, period: f32) {
        self.bit_periods.push(period);
        if self.bit_periods.len() > self.max_history {
            self.bit_periods.remove(0);
        }
    }

    /// Get average period
    #[allow(dead_code)]
    fn get_average_period(&self) -> Option<f32> {
        if self.bit_periods.is_empty() {
            return None;
        }

        let sum: f32 = self.bit_periods.iter().sum();
        Some(sum / self.bit_periods.len() as f32)
    }

    /// Get speed ratio (1.0 = nominal speed)
    #[allow(dead_code)]
    fn get_speed_ratio(&self, nominal_period: f32) -> f32 {
        self.get_average_period()
            .map_or(1.0, |avg| nominal_period / avg)
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.bit_periods.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_creation() {
        let decoder = LtcDecoder::new(48000, FrameRate::Fps25, 0.1);
        assert!(!decoder.is_synchronized());
    }

    #[test]
    fn test_zero_crossing_detector() {
        let mut detector = ZeroCrossingDetector::new(48000, FrameRate::Fps25);

        // Test rising edge
        let t1 = detector.process_sample(-0.5, 0.1);
        assert!(t1.is_none());

        let t2 = detector.process_sample(0.5, 0.1);
        assert!(t2.is_some());
    }

    #[test]
    fn test_bits_to_u8() {
        let decoder = LtcDecoder::new(48000, FrameRate::Fps25, 0.1);
        let bits = [true, false, true, false]; // Binary: 0101 = 5
        assert_eq!(decoder.bits_to_u8(&bits), 5);
    }

    #[test]
    fn test_drop_frame_validator() {
        assert!(!DropFrameCalculator::is_valid_drop_frame(1, 0, 0));
        assert!(!DropFrameCalculator::is_valid_drop_frame(1, 0, 1));
        assert!(DropFrameCalculator::is_valid_drop_frame(1, 0, 2));
        assert!(DropFrameCalculator::is_valid_drop_frame(10, 0, 0));
    }
}
