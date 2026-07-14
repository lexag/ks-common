#[allow(missing_docs)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MetronomeWaveform {
    Sine,
    SquircleSine,
    Square,
    Triangle,
}

/// Defines the parameters for a single metronome "blip"
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct MetronomeClick {
    /// Frequency in Hz
    pub frequency: u16,
    /// Length in ms
    pub length: u16,
    /// Waveform
    pub wave: MetronomeWaveform,
}

/// Configuration for metronome settings; 4 different levels of click settings
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct MetronomeConfiguration {
    /// Primary click; generally beat 1
    pub click_primary: MetronomeClick,
    /// Secondary click: generally beat 2, 3, 4 etc.
    pub click_secondary: MetronomeClick,
    /// Tertiary click: generally offbeats
    pub click_tertiary: MetronomeClick,
    /// Quartenary click: generally 16ths or triplets
    pub click_quartenary: MetronomeClick,
}

impl Default for MetronomeConfiguration {
    fn default() -> Self {
        Self {
            click_primary: MetronomeClick {
                frequency: 800,
                length: 4,
                wave: MetronomeWaveform::SquircleSine,
            },
            click_secondary: MetronomeClick {
                frequency: 400,
                length: 4,
                wave: MetronomeWaveform::SquircleSine,
            },
            click_tertiary: MetronomeClick {
                frequency: 200,
                length: 4,
                wave: MetronomeWaveform::SquircleSine,
            },
            click_quartenary: MetronomeClick {
                frequency: 1600,
                length: 4,
                wave: MetronomeWaveform::SquircleSine,
            },
        }
    }
}

impl MetronomeClick {
    /// Generate a wave buffer for this click
    #[cfg(feature = "std")]
    pub fn buffer(&self, sample_rate: u32) -> [f32; 96000] {
        let mut buf = [0f32; 96000];
        let func = match self.wave {
            MetronomeWaveform::Sine => |f: f32| f32::sin(f),
            MetronomeWaveform::SquircleSine => |f: f32| {
                let fs = f32::sin(f);
                fs / (fs.abs() + 0.5).abs() * 1.5
            },
            MetronomeWaveform::Square => |f: f32| {
                let fs = f32::sin(f);
                fs.signum()
            },
            MetronomeWaveform::Triangle => |f: f32| {
                let mut out = f / 2.0 * core::f32::consts::PI - 1.0;
                out %= 1.0;
                out -= 0.5;
                out = out.abs();
                4.0 * out - 1.0
            },
        };
        for i in 0..self.length as u32 * sample_rate / 1000 {
            buf[i as usize] = (func)(
                i as f32 * core::f32::consts::PI * self.frequency as f32 * 2.0 / sample_rate as f32,
            ) * 0.1_f32;
        }
        buf
    }
}
