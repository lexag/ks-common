use crate::{
    components,
    interface::{ConfigurationWidget, InlineWidget},
};
use egui::{Align, Key, Widget};
use ks_common_generic::smpte::{Timecode, TimecodeOffset};

impl InlineWidget for Timecode {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(&format!("[{}] {}", self.frame_rate.fps, self), 14)
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
        let fps = self.frame_rate;
        crate::components::Numpad::new(self)
            .with_side_keys([Key::H, Key::M, Key::S, Key::F])
            .with_decimal(Key::Colon)
            .ui(ui);
        //self.frame_rate = fps;
    }
}

impl ConfigurationWidget for TimecodeOffset {
    fn grid_contents(&mut self, ui: &mut egui::Ui) {
        let fps = self.abs_time.frame_rate;
        crate::components::Numpad::new(self)
            .with_side_keys([Key::H, Key::M, Key::S, Key::F])
            .with_decimal(Key::Colon)
            .with_sign()
            .ui(ui);
        //self.abs_time.frame_rate = fps;
    }
}
