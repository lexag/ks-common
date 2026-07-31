use crate::{
    components,
    interface::{ConfigurationWidget, InlineWidget},
    traits::InlineWidgetAutoEnum,
};
use egui::{Align, Key, Widget};
use ks_common_generic::smpte::{FrameRate, FrameRateInfo, Timecode, TimecodeOffset};

impl InlineWidget for Timecode {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(&format!("{}", self), 11)
            .label(label)
            .align(Align::Max)
            .ui(ui)
    }
}

impl InlineWidget for Option<Timecode> {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        match self {
            Some(tc) => tc.draw(ui, label),
            None => components::TextDisplay::new("NO    TC", 14)
                .align(Align::Center)
                .label(label)
                .color(ui.visuals().warn_fg_color)
                .ui(ui),
        }
    }
}

impl InlineWidget for TimecodeOffset {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(
            &format!(
                "{} {}",
                if self.is_negative { '-' } else { '+' },
                self.abs_time
            ),
            14,
        )
        .label(label)
        .align(Align::Max)
        .ui(ui)
    }
}

impl ConfigurationWidget for Timecode {
    fn grid_contents(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let fps = self.frame_rate;
            crate::components::Numpad::new(self)
                .with_side_keys([Key::H, Key::M, Key::S, Key::F])
                .with_decimal(Key::Colon)
                .ui(ui);
            let mut fr: FrameRate = fps.into();
            fr.autoenum_inline_widget_menu(ui, "Frame rate");
            self.frame_rate = FrameRateInfo {
                drop_frame: fr.is_drop_frame(),
                fps: fr.frames_per_second() as u8,
            };
        });
    }
}

impl ConfigurationWidget for TimecodeOffset {
    fn grid_contents(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let fps = self.abs_time.frame_rate;
            crate::components::Numpad::new(self)
                .with_side_keys([Key::H, Key::M, Key::S, Key::F])
                .with_decimal(Key::Colon)
                .with_sign()
                .ui(ui);
            let mut fr: FrameRate = fps.into();
            fr.autoenum_inline_widget_menu(ui, "Frame rate");
            self.abs_time.frame_rate = FrameRateInfo {
                drop_frame: fr.is_drop_frame(),
                fps: fr.frames_per_second() as u8,
            };
        });
    }
}

impl InlineWidgetAutoEnum for FrameRate {
    fn options() -> Vec<Self>
    where
        Self: Sized + core::fmt::Display,
    {
        vec![
            Self::Fps23976,
            Self::Fps23976DF,
            Self::Fps24,
            Self::Fps25,
            Self::Fps2997DF,
            Self::Fps2997NDF,
            Self::Fps30,
            Self::Fps47952,
            Self::Fps47952DF,
            Self::Fps50,
            Self::Fps5994,
            Self::Fps5994DF,
            Self::Fps60,
            Self::Fps120,
        ]
    }
}
