//! Linear Timecode (LTC) reading and writing
//!
//! LTC encodes timecode as an audio signal using biphase mark code (BMC).
//! The audio signal contains 80 bits per frame:
//! - 64 bits for timecode and user data
//! - 16 bits for sync word (0x3FFD)
//!
//! # Biphase Mark Code
//! - Bit 0: One transition in the middle of the bit cell
//! - Bit 1: Two transitions (at the beginning and middle)
//! - Frequencies: ~1920 Hz (bit 0 at 30fps) to ~2400 Hz (bit 1 at 30fps)
//!
//! # Signal Characteristics
//! - Typically recorded at line level (-10 dBV to +4 dBu)
//! - Can be read in forward or reverse
//! - Can be read at varying speeds (0.1x to 10x nominal)

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::vec::Vec;

use crate::timecode::{
    FrameRate, Timecode, TimecodeError, TimecodeReader, TimecodeWriter, decoder,
    encoder::LtcEncoder,
};

/// LTC reader configuration
#[derive(Debug, Clone)]
pub struct LtcReaderConfig {
    /// Sample rate of the input audio
    pub sample_rate: u32,
    /// Expected frame rate
    pub frame_rate: FrameRate,
    /// Minimum signal amplitude (0.0 to 1.0)
    pub min_amplitude: f32,
    /// Maximum speed variation (1.0 = nominal, 2.0 = 2x speed)
    pub max_speed: f32,
}

impl Default for LtcReaderConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            frame_rate: FrameRate::Fps25,
            min_amplitude: 0.1,
            max_speed: 2.0,
        }
    }
}

/// LTC reader
pub struct LtcReader {
    decoder: decoder::LtcDecoder,
}

impl LtcReader {
    /// Create a new LTC reader with configuration
    pub fn new(config: LtcReaderConfig) -> Self {
        Self {
            decoder: decoder::LtcDecoder::new(
                config.sample_rate,
                config.frame_rate,
                config.min_amplitude,
            ),
        }
    }

    /// Process audio samples and attempt to decode timecode
    pub fn process_samples(&mut self, samples: &[f32]) -> Result<Option<Timecode>, TimecodeError> {
        self.decoder.process_samples(samples)
    }

    /// Reset the decoder state
    pub fn reset(&mut self) {
        self.decoder.reset();
    }

    /// Get the current sync confidence (0.0 to 1.0)
    pub fn sync_confidence(&self) -> f32 {
        self.decoder.sync_confidence()
    }
}

impl TimecodeReader for LtcReader {
    /// Return the most recently decoded timecode.
    ///
    /// Audio samples must be submitted via [`LtcReader::process_samples`]
    /// before this method can return `Some(timecode)`.  Calling
    /// `read_timecode` without first feeding samples will always return
    /// `Ok(None)` because no frames have been decoded yet.
    ///
    /// This design keeps the `TimecodeReader` trait pull-based: callers that
    /// own the sample source feed chunks via `process_samples`, then poll
    /// `read_timecode` to retrieve completed frames.
    fn read_timecode(&mut self) -> Result<Option<Timecode>, TimecodeError> {
        Ok(self.decoder.last_decoded_timecode())
    }

    fn frame_rate(&self) -> FrameRate {
        self.decoder.frame_rate()
    }

    fn is_synchronized(&self) -> bool {
        self.decoder.is_synchronized()
    }

    fn read_timecode_confidence(&mut self) -> Result<(Timecode, f32), TimecodeError> {
        self.read_timecode()?.map_or_else(
            || Ok((Timecode::default(), 0.0)),
            |t| Ok((t, self.decoder.sync_confidence())),
        )
    }
}

/// LTC writer configuration
#[derive(Debug, Clone)]
pub struct LtcWriterConfig {
    /// Sample rate of the output audio
    pub sample_rate: u32,
    /// Frame rate to encode
    pub frame_rate: FrameRate,
    /// Output signal amplitude (0.0 to 1.0)
    pub amplitude: f32,
}

impl Default for LtcWriterConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            frame_rate: FrameRate::Fps25,
            amplitude: 0.5,
        }
    }
}

/// LTC writer
pub struct LtcWriter {
    encoder: LtcEncoder,
}

impl LtcWriter {
    /// Create a new LTC writer with configuration
    pub fn new(config: LtcWriterConfig) -> Self {
        Self {
            encoder: LtcEncoder::new(config.sample_rate, config.frame_rate, config.amplitude),
        }
    }

    /// Encode a timecode frame to audio samples
    pub fn encode_frame(&mut self, timecode: &Timecode) -> Result<Vec<f32>, TimecodeError> {
        self.encoder.encode_frame(timecode)
    }

    /// Reset the encoder state
    pub fn reset(&mut self) {
        self.encoder.reset();
    }
}

impl TimecodeWriter for LtcWriter {
    fn write_timecode(&mut self, timecode: &Timecode) -> Result<(), TimecodeError> {
        let _samples = self.encode_frame(timecode)?;
        // In a real implementation, samples would be written to an audio output
        Ok(())
    }

    fn frame_rate(&self) -> FrameRate {
        self.encoder.frame_rate()
    }

    fn flush(&mut self) -> Result<(), TimecodeError> {
        Ok(())
    }
}

/// LTC bit patterns and constants
pub(crate) mod constants {
    /// SMPTE sync word (0x3FFD in binary: 11 1111 1111 1101)
    pub const SYNC_WORD: u16 = 0x3FFD;

    /// Number of bits per LTC frame
    pub const BITS_PER_FRAME: usize = 80;

    /// Number of data bits (excluding sync word)
    pub const DATA_BITS: usize = 64;

    /// Sync word bit length
    pub const SYNC_BITS: usize = 16;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ltc_reader_creation() {
        let config = LtcReaderConfig::default();
        let _reader = LtcReader::new(config);
    }

    #[test]
    fn test_ltc_writer_creation() {
        let config = LtcWriterConfig::default();
        let _writer = LtcWriter::new(config);
    }

    #[test]
    fn test_sync_word() {
        assert_eq!(constants::SYNC_WORD, 0x3FFD);
        assert_eq!(constants::BITS_PER_FRAME, 80);
    }

    #[test]
    fn write_read_cycle() {
        extern crate std;
        let mut combinations = std::vec::Vec::new();
        for sample_rate in [44100, 48000, 96000] {
            for frame_rate in [
                FrameRate::Fps24,
                FrameRate::Fps25,
                FrameRate::Fps2997DF,
                FrameRate::Fps30,
            ] {
                combinations.push((sample_rate, frame_rate));
            }
        }

        for (sample_rate, frame_rate) in combinations {
            let mut writer = LtcWriter::new(LtcWriterConfig {
                sample_rate,
                frame_rate,
                amplitude: 0.5,
            });
            let mut reader = LtcReader::new(LtcReaderConfig {
                sample_rate,
                frame_rate,
                min_amplitude: 1e-3,
                max_speed: 2.0,
            });

            let mut timecode = Timecode::new(1, 2, 3, 4, frame_rate).expect("valid");
            let mut samples = Vec::new();
            for _ in 0..500 {
                samples.extend(writer.encode_frame(&timecode).expect("write"));
                timecode.increment().unwrap();
            }

            let portions = samples.chunks(sample_rate as usize / 80);
            let mut num_handled = 0;
            let mut num_failed = 0;
            for portion in portions {
                let _ = reader.process_samples(portion).expect("read");
                if let Ok(Some(_)) = reader.read_timecode() {
                } else {
                    num_failed += 1
                }
                num_handled += 1;
            }
            assert!(
                num_failed < 10,
                "failed {} reads of {}",
                num_failed,
                num_handled
            );
        }
    }
}
