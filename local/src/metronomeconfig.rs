use num_traits::Float;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MetronomeWaveform {
    Sine,
    SquircleSine,
    Square,
    Triangle,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct MetronomeClick {
    frequency: u16,
    length: u16,
    wave: MetronomeWaveform,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct MetronomeConfiguration {
    click_primary: MetronomeClick,
    click_secondary: MetronomeClick,
    click_tertiary: MetronomeClick,
    click_quartenary: MetronomeClick,
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
    const PI: f32 = 3.1415926535;

    fn buffer(&self, sample_rate: u32) -> [f32; 96000] {
        let mut buf = [0f32; 96000];
        let func = match self.wave {
            MetronomeWaveform::Sine => |f: f32| <f32 as Float>::sin(f),
            MetronomeWaveform::SquircleSine => |f: f32| {
                let fs = <f32 as Float>::sin(f);
                let out = fs / (fs.abs() + 0.5).abs() * 1.5;
                out
            },
            MetronomeWaveform::Square => |f: f32| {
                let fs = <f32 as Float>::sin(f);
                let out = fs.signum();
                out
            },
            MetronomeWaveform::Triangle => |f: f32| {
                let mut out = f / 2.0 * Self::PI - 1.0;
                out = out % 1.0;
                out -= 0.5;
                out = out.abs();
                4.0 * out - 1.0
            },
        };
        for i in 0..self.length as u32 * sample_rate / 1000 {
            buf[i as usize] =
                (func)(i as f32 * Self::PI * self.frequency as f32 * 2.0 / sample_rate as f32)
                    * 0.1 as f32;
        }
        buf
    }
}
