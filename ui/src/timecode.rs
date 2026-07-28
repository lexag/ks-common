use crate::{
    graphics::{self, draw_segmented_display},
    interface::{ConfigurationWidget, InlineWidget},
    style,
};
use egui::{Key, Widget};
use ks_common_generic::smpte::{Timecode, TimecodeOffset};

const CHAR: f32 = graphics::SEGMENTED_CHAR_WIDTH;
const SEPR: f32 = graphics::SEGMENTED_SEPR_WIDTH;

impl InlineWidget for Timecode {
    fn draw(&mut self, ui: &mut egui::Ui, label: String) -> egui::Response {
        draw_segmented_display(
            ui,
            14,
            &format!("[{}] {}", self.frame_rate.fps, self),
            style::ACCENT_COLOR,
            label,
        )
    }
}

impl InlineWidget for Option<Timecode> {
    fn draw(&mut self, ui: &mut egui::Ui, label: String) -> egui::Response {
        match self {
            Some(tc) => tc.draw(ui, label),
            None => draw_segmented_display(ui, 14, "   NO    TC   ", style::WARNING_COLOR, label),
        }
    }
}

impl InlineWidget for TimecodeOffset {
    fn draw(&mut self, ui: &mut egui::Ui, label: String) -> egui::Response {
        draw_segmented_display(
            ui,
            14,
            &format!(
                "{} {}",
                if self.is_negative { '-' } else { '+' },
                self.abs_time
            ),
            ui.visuals().text_color(),
            label,
        )
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
