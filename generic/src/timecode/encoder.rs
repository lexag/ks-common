//! LTC Encoder - Biphase Mark Code encoding to audio
//!
//! This module implements a complete LTC encoder that:
//! - Encodes timecode and user bits to 80-bit LTC frames
//! - Generates biphase mark code audio waveforms
//! - Inserts SMPTE sync words
//! - Handles drop frame encoding
//! - Supports variable amplitude and sample rates

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::vec::Vec;

use crate::timecode::{FrameRate, Timecode, TimecodeError, readwrite::constants::*};

/// LTC encoder
pub struct LtcEncoder {
    /// Sample rate
    #[allow(dead_code)]
    sample_rate: u32,
    /// Frame rate
    #[allow(dead_code)]
    frame_rate: FrameRate,
    /// Output amplitude (0.0 to 1.0)
    amplitude: f32,
    /// Samples per bit (for nominal speed)
    samples_per_bit: f32,
    /// Current phase (0.0 to 1.0)
    #[allow(dead_code)]
    phase: f32,
    /// Current waveform polarity
    polarity: bool,
}

impl LtcEncoder {
    /// Create a new LTC encoder
    pub fn new(sample_rate: u32, frame_rate: FrameRate, amplitude: f32) -> Self {
        let fps = frame_rate.as_float();
        let bits_per_second = fps * BITS_PER_FRAME as f64;
        let samples_per_bit = sample_rate as f64 / bits_per_second;

        Self {
            sample_rate,
            frame_rate,
            amplitude: amplitude.clamp(0.0, 1.0),
            samples_per_bit: samples_per_bit as f32,
            phase: 0.0,
            polarity: false,
        }
    }

    /// Get this encoder's set frame rate
    pub fn frame_rate(&self) -> FrameRate {
        self.frame_rate
    }

    /// Encode a timecode frame to audio samples
    pub fn encode_frame(&mut self, timecode: &Timecode) -> Result<Vec<f32>, TimecodeError> {
        // Create bit array
        let bits = self.timecode_to_bits(timecode)?;

        // Encode bits to audio
        let mut samples = self.bits_to_audio(&bits);

        samples = self.low_pass(samples);

        Ok(samples)
    }

    /// Cut down hard edges on bit borders
    fn low_pass(&self, buf: Vec<f32>) -> Vec<f32> {
        //for (i, s) in buf.iter().enumerate() {
        //    println!("buf {i:03} {s}")
        //}
        let mut out = std::vec![0.0; buf.len()];
        const LP_WIDTH: usize = 3;
        for (idx, item) in out.iter_mut().enumerate().take(buf.len()) {
            let mut cumsum = 0.0;
            for offs_idx in idx..idx + LP_WIDTH {
                cumsum += if offs_idx < buf.len() {
                    buf[offs_idx]
                } else {
                    -buf[buf.len() - 10]
                }
            }
            cumsum /= LP_WIDTH as f32;
            *item = cumsum;
        }
        out
    }

    /// Convert timecode to 80-bit LTC frame
    fn timecode_to_bits(
        &self,
        timecode: &Timecode,
    ) -> Result<[bool; BITS_PER_FRAME], TimecodeError> {
        let mut bits = [false; BITS_PER_FRAME];

        // Decompose timecode
        let frame_units = timecode.frames % 10;
        let frame_tens = timecode.frames / 10;
        let second_units = timecode.seconds % 10;
        let second_tens = timecode.seconds / 10;
        let minute_units = timecode.minutes % 10;
        let minute_tens = timecode.minutes / 10;
        let hour_units = timecode.hours % 10;
        let hour_tens = timecode.hours / 10;

        // Encode frame units (bits 0-3)
        self.encode_bcd(&mut bits, 0, frame_units);

        // User bits 1 (bits 4-7)
        self.encode_nibble(&mut bits, 4, (timecode.user_bits & 0xF) as u8);

        // Frame tens (bits 8-9)
        self.encode_bcd(&mut bits, 8, frame_tens);

        // Drop frame flag (bit 10)
        bits[10] = timecode.frame_rate.drop_frame;

        // Color frame flag (bit 11) - assume 0
        bits[11] = false;

        // User bits 2 (bits 12-15)
        self.encode_nibble(&mut bits, 12, ((timecode.user_bits >> 4) & 0xF) as u8);

        // Second units (bits 16-19)
        self.encode_bcd(&mut bits, 16, second_units);

        // User bits 3 (bits 20-23)
        self.encode_nibble(&mut bits, 20, ((timecode.user_bits >> 8) & 0xF) as u8);

        // Second tens (bits 24-26)
        self.encode_bcd(&mut bits, 24, second_tens);

        // Even parity (bit 27)
        bits[27] = self.calculate_even_parity(&bits[0..27]);

        // User bits 4 (bits 28-31)
        self.encode_nibble(&mut bits, 28, ((timecode.user_bits >> 12) & 0xF) as u8);

        // Minute units (bits 32-35)
        self.encode_bcd(&mut bits, 32, minute_units);

        // User bits 5 (bits 36-39)
        self.encode_nibble(&mut bits, 36, ((timecode.user_bits >> 16) & 0xF) as u8);

        // Minute tens (bits 40-42)
        self.encode_bcd(&mut bits, 40, minute_tens);

        // Binary group flag (bit 43)
        bits[43] = false;

        // User bits 6 (bits 44-47)
        self.encode_nibble(&mut bits, 44, ((timecode.user_bits >> 20) & 0xF) as u8);

        // Hour units (bits 48-51)
        self.encode_bcd(&mut bits, 48, hour_units);

        // User bits 7 (bits 52-55)
        self.encode_nibble(&mut bits, 52, ((timecode.user_bits >> 24) & 0xF) as u8);

        // Hour tens (bits 56-57)
        self.encode_bcd(&mut bits, 56, hour_tens);

        // Reserved bits (58)
        bits[58] = false;

        // User bits 8 (bits 59-62)
        self.encode_nibble(&mut bits, 59, ((timecode.user_bits >> 28) & 0xF) as u8);

        // Reserved bit (63)
        bits[63] = false;

        // Sync word (bits 64-79)
        self.encode_sync_word(&mut bits);

        Ok(bits)
    }

    /// Encode a BCD digit (4 bits, but may use fewer)
    fn encode_bcd(&self, bits: &mut [bool; BITS_PER_FRAME], start: usize, value: u8) {
        for i in 0..4 {
            if start + i < BITS_PER_FRAME {
                bits[start + i] = (value & (1 << i)) != 0;
            }
        }
    }

    /// Encode a 4-bit nibble
    fn encode_nibble(&self, bits: &mut [bool; BITS_PER_FRAME], start: usize, value: u8) {
        for i in 0..4 {
            if start + i < BITS_PER_FRAME {
                bits[start + i] = (value & (1 << i)) != 0;
            }
        }
    }

    /// Calculate even parity
    fn calculate_even_parity(&self, bits: &[bool]) -> bool {
        let count = bits.iter().filter(|&&b| b).count();
        count % 2 != 0
    }

    /// Encode sync word (0x3FFD)
    fn encode_sync_word(&self, bits: &mut [bool; BITS_PER_FRAME]) {
        let sync_word = SYNC_WORD.reverse_bits();
        for i in 0..SYNC_BITS {
            bits[DATA_BITS + i] = (sync_word & (1 << i)) != 0;
        }
    }

    /// Convert bit array to audio samples using biphase mark code
    fn bits_to_audio(&mut self, bits: &[bool; BITS_PER_FRAME]) -> Vec<f32> {
        let total_samples = (self.samples_per_bit * BITS_PER_FRAME as f32) as usize;
        let mut samples = Vec::with_capacity(total_samples);

        for &bit in bits.iter() {
            // Generate audio for one bit using biphase mark code
            let bit_samples = self.encode_bit_bmc(bit);
            samples.extend_from_slice(&bit_samples);
        }

        samples
    }

    /// Encode a single bit using biphase mark code
    fn encode_bit_bmc(&mut self, bit: bool) -> Vec<f32> {
        let samples_per_bit = self.samples_per_bit as usize;
        let mut samples = Vec::with_capacity(samples_per_bit);

        if bit {
            // Bit 1: Transition at start and middle
            // First half
            for _ in 0..(samples_per_bit / 2) {
                samples.push(if self.polarity {
                    self.amplitude
                } else {
                    -self.amplitude
                });
            }
            self.polarity = !self.polarity;

            // Second half
            for _ in (samples_per_bit / 2)..samples_per_bit {
                samples.push(if self.polarity {
                    self.amplitude
                } else {
                    -self.amplitude
                });
            }
        } else {
            // Bit 0: Transition only at start
            for _ in 0..samples_per_bit {
                samples.push(if self.polarity {
                    self.amplitude
                } else {
                    -self.amplitude
                });
            }
        }
        self.polarity = !self.polarity;

        samples
    }

    /// Reset encoder state
    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.polarity = false;
    }
}

/// Waveform shaper for improved signal quality
#[allow(dead_code)]
struct WaveformShaper {
    /// Rise time (in samples)
    rise_time: usize,
    /// Current transition progress
    transition_progress: usize,
    /// Target level
    target_level: f32,
    /// Current level
    current_level: f32,
}

impl WaveformShaper {
    #[allow(dead_code)]
    fn new(sample_rate: u32, rise_time_us: f32) -> Self {
        let rise_time = ((rise_time_us / 1_000_000.0) * sample_rate as f32) as usize;

        Self {
            rise_time: rise_time.max(1),
            transition_progress: 0,
            target_level: 0.0,
            current_level: 0.0,
        }
    }

    /// Set target level for transition
    #[allow(dead_code)]
    fn set_target(&mut self, level: f32) {
        if (level - self.target_level).abs() > 0.001 {
            self.target_level = level;
            self.transition_progress = 0;
        }
    }

    /// Get next shaped sample
    #[allow(dead_code)]
    fn next_sample(&mut self) -> f32 {
        if self.transition_progress < self.rise_time {
            // Linear interpolation during transition
            let progress = self.transition_progress as f32 / self.rise_time as f32;
            self.current_level =
                self.current_level * (1.0 - progress) + self.target_level * progress;
            self.transition_progress += 1;
        } else {
            self.current_level = self.target_level;
        }

        self.current_level
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.transition_progress = 0;
        self.current_level = 0.0;
    }
}

/// Pre-emphasis filter for tape recording
#[allow(dead_code)]
struct PreEmphasisFilter {
    /// Filter coefficient
    alpha: f32,
    /// Previous input
    prev_input: f32,
    /// Previous output
    prev_output: f32,
}

impl PreEmphasisFilter {
    #[allow(dead_code)]
    fn new(time_constant_us: f32, sample_rate: u32) -> Self {
        let tc = time_constant_us / 1_000_000.0;
        let dt = 1.0 / sample_rate as f32;
        let alpha = tc / (tc + dt);

        Self {
            alpha,
            prev_input: 0.0,
            prev_output: 0.0,
        }
    }

    /// Apply pre-emphasis to sample
    #[allow(dead_code)]
    fn process(&mut self, input: f32) -> f32 {
        let output = self.alpha * (self.prev_output + input - self.prev_input);
        self.prev_input = input;
        self.prev_output = output;
        output
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.prev_input = 0.0;
        self.prev_output = 0.0;
    }
}

/// DC offset remover
#[allow(dead_code)]
struct DcBlocker {
    /// Filter coefficient
    alpha: f32,
    /// Previous input
    prev_input: f32,
    /// Previous output
    prev_output: f32,
}

impl DcBlocker {
    #[allow(dead_code)]
    fn new(cutoff_hz: f32, sample_rate: u32) -> Self {
        let rc = 1.0 / (2.0 * core::f32::consts::PI * cutoff_hz);
        let dt = 1.0 / sample_rate as f32;
        let alpha = rc / (rc + dt);

        Self {
            alpha,
            prev_input: 0.0,
            prev_output: 0.0,
        }
    }

    /// Remove DC offset from sample
    #[allow(dead_code)]
    fn process(&mut self, input: f32) -> f32 {
        let output = self.alpha * (self.prev_output + input - self.prev_input);
        self.prev_input = input;
        self.prev_output = output;
        output
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.prev_input = 0.0;
        self.prev_output = 0.0;
    }
}

/// Amplitude limiter
#[allow(dead_code)]
struct Limiter {
    /// Threshold (0.0 to 1.0)
    threshold: f32,
    /// Attack time (in samples)
    attack_samples: usize,
    /// Release time (in samples)
    release_samples: usize,
    /// Current gain reduction
    gain_reduction: f32,
}

impl Limiter {
    #[allow(dead_code)]
    fn new(threshold: f32, attack_ms: f32, release_ms: f32, sample_rate: u32) -> Self {
        let attack_samples = ((attack_ms / 1000.0) * sample_rate as f32) as usize;
        let release_samples = ((release_ms / 1000.0) * sample_rate as f32) as usize;

        Self {
            threshold,
            attack_samples: attack_samples.max(1),
            release_samples: release_samples.max(1),
            gain_reduction: 1.0,
        }
    }

    /// Apply limiting to sample
    #[allow(dead_code)]
    fn process(&mut self, input: f32) -> f32 {
        let abs_input = input.abs();

        if abs_input > self.threshold {
            // Attack: reduce gain quickly
            let target_gain = self.threshold / abs_input;
            let attack_coefficient = 1.0 / self.attack_samples as f32;
            self.gain_reduction += (target_gain - self.gain_reduction) * attack_coefficient;
        } else {
            // Release: increase gain slowly
            let release_coefficient = 1.0 / self.release_samples as f32;
            self.gain_reduction += (1.0 - self.gain_reduction) * release_coefficient;
        }

        input * self.gain_reduction
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.gain_reduction = 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_creation() {
        let encoder = LtcEncoder::new(48000, FrameRate::Fps25, 0.5);
        assert_eq!(encoder.amplitude(), 0.5);
    }

    #[test]
    fn test_encode_frame() {
        let mut encoder = LtcEncoder::new(48000, FrameRate::Fps25, 0.5);
        let timecode = Timecode::new(1, 2, 3, 4, FrameRate::Fps25).expect("valid timecode");
        let samples = encoder
            .encode_frame(&timecode)
            .expect("encode should succeed");
        std::println!("{:?}", samples);
    }

    #[test]
    fn test_encode_to_bits() {
        let encoder = LtcEncoder::new(48000, FrameRate::Fps25, 0.5);
        let timecode = Timecode::new(1, 2, 3, 4, FrameRate::Fps25).expect("valid timecode");
        let Ok(bits) = encoder.timecode_to_bits(&timecode) else {
            panic!("to bits failed");
        };
        const CORRECT: [bool; 80] = [
            false, false, true, false, // frame units
            false, false, false, false, // user bits 1
            false, false, // frame tens
            false, false, // drop frame and color frame flags
            false, false, false, false, // user bits 2
            //
            true, true, false, false, // seconds units
            false, false, false, false, // user bits 3
            false, false, false, // seconds tens
            true,  // polarity
            false, false, false, false, // user bits 4
            //
            false, true, false, false, // minutes units
            false, false, false, false, // user bits 5
            false, false, false, // minutes tens
            false, // flag
            false, false, false, false, // user bits 6
            //
            true, false, false, false, // hours units
            false, false, false, false, // user bits 7
            false, false, // hours tens
            false, false, // flags
            false, false, false, false, // user bits 8
            //
            false, false, true, true, true, true, true, true, true, true, true, true, true, true,
            false, true, // sync word
        ];
        for i in 0..80 {
            assert_eq!(bits[i], CORRECT[i], "at bit {}", i);
        }
    }

    #[test]
    fn test_even_parity() {
        let encoder = LtcEncoder::new(48000, FrameRate::Fps25, 0.5);
        let bits = [true, false, true]; // 2 true bits = even
        assert!(!encoder.calculate_even_parity(&bits));

        let bits = [true, false, false]; // 1 true bit = odd
        assert!(encoder.calculate_even_parity(&bits));
    }

    #[cfg(feature = "full-test")]
    #[test]
    fn smpte_ltc_eq() {
        use super::*;

        const NUM_BLOCKS: usize = 1000;
        const SAMPLE_RATE: usize = 48000;
        const SMPTE_FRAME_RATE: usize = 25;
        const SAMPLES_PER_FRAME: usize = SAMPLE_RATE / SMPTE_FRAME_RATE;
        const SAMPLES_PER_BIT: usize = SAMPLES_PER_FRAME / 80;

        let mut encoder = LtcEncoder::new(SAMPLE_RATE as u32, FrameRate::Fps25, 0.5);

        let mut timecode = Timecode::new(0, 0, 0, 0, FrameRate::Fps25).expect("valid");
        let mut generated_samples = Vec::new();
        for _ in 0..NUM_BLOCKS {
            generated_samples.extend(encoder.encode_frame(&timecode).expect("write"));
            timecode.increment().unwrap();
        }

        assert_ne!(generated_samples.iter().map(|s| s.abs()).sum::<f32>(), 0.0);

        let reader = hound::WavReader::open("tests/data/smpte_25fps.wav");

        assert!(reader.is_ok());
        let mut reader = reader.unwrap();
        let rsamples: Vec<i16> = reader.samples::<i16>().map(|o| o.unwrap_or(0)).collect();

        let rpeak = rsamples.iter().max().unwrap_or(&0);

        let cscale_factor = *rpeak as f32 / i16::MAX as f32;

        let csamples: Vec<i16> = generated_samples
            .into_iter()
            .map(|v| (v * 32768.0 * cscale_factor * 2.0) as i16)
            .collect();

        const EQUAL_THRESHOLD: u16 = 2;
        for (i, (a, b)) in csamples.iter().zip(rsamples.iter()).enumerate() {
            if i % SAMPLES_PER_FRAME / SAMPLES_PER_BIT == 27
                || i % SAMPLES_PER_FRAME / SAMPLES_PER_BIT == 59
                || i % SAMPLES_PER_FRAME / SAMPLES_PER_BIT == 43
            {
                continue;
            }
            if a.abs().abs_diff(b.abs()) >= EQUAL_THRESHOLD {
                std::println!("Failed at sample {}", i);
                std::println!("splglb\tsplloc\tframe\tbit\tthis\trea");

                for j in i.saturating_sub(3)..i {
                    std::println!(
                        "{j}\t{}\t{}\t{}\t{}\t{}\tO",
                        j % SAMPLES_PER_FRAME,
                        j / SAMPLES_PER_FRAME,
                        j % SAMPLES_PER_FRAME / SAMPLES_PER_BIT,
                        csamples[j],
                        rsamples[j]
                    );
                }
                std::println!(
                    "{i}\t{}\t{}\t{}\t{}\t{}\t<--",
                    i % SAMPLES_PER_FRAME,
                    i / SAMPLES_PER_FRAME,
                    i % SAMPLES_PER_FRAME / SAMPLES_PER_BIT,
                    a,
                    b
                );
                for j in i + 1..i + 8 {
                    std::println!(
                        "{j}\t{}\t{}\t{}\t{}\t{}\t{}",
                        j % SAMPLES_PER_FRAME,
                        j / SAMPLES_PER_FRAME,
                        j % SAMPLES_PER_FRAME / SAMPLES_PER_BIT,
                        csamples[j],
                        rsamples[j],
                        if csamples[j].abs_diff(rsamples[j]) < EQUAL_THRESHOLD {
                            "O"
                        } else {
                            "X"
                        }
                    );
                }
                assert!(a.abs_diff(*b) < EQUAL_THRESHOLD);
            }
        }
    }
}
