use crate::{
    graphics::{self, draw_segmented_display},
    interface::{ConfigurationWidget, InlineWidget},
    style,
};
use egui::{Key, Widget};
use ks_common_generic::smpte::{Timecode, TimecodeOffset};

const CHAR: f32 = graphics::SEGMENTED_CHAR_WIDTH;
const SEPR: f32 = graphics::SEGMENTED_SEPR_WIDTH;
const CHAR_WIDTHS: &[f32] = &[
    CHAR, CHAR, SEPR, CHAR, CHAR, SEPR, CHAR, CHAR, SEPR, CHAR, CHAR, SEPR, CHAR, CHAR,
];

impl InlineWidget for Timecode {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            CHAR_WIDTHS,
            &format!("{} {}", self, self.frame_rate.fps),
            style::ACCENT_COLOR,
            scale,
        )
    }
}

impl InlineWidget for Option<Timecode> {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        match self {
            Some(tc) => tc.draw(ui, scale),
            None => draw_segmented_display(
                ui,
                CHAR_WIDTHS,
                "   NO    TC   ",
                style::WARNING_COLOR,
                scale,
            ),
        }
    }
}

impl InlineWidget for TimecodeOffset {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            CHAR_WIDTHS,
            &format!(
                " {} {}",
                if self.is_negative { '-' } else { '+' },
                self.abs_time
            ),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl ConfigurationWidget for Timecode {
    fn grid_contents(&mut self, ui: &mut egui::Ui) {
        let fps = self.frame_rate;
        crate::components::numpad::Numpad::new(self)
            .with_side_keys([Key::H, Key::M, Key::S, Key::F])
            .with_decimal(Key::Colon)
            .ui(ui);
        //self.frame_rate = fps;
    }
}

impl ConfigurationWidget for TimecodeOffset {
    fn grid_contents(&mut self, ui: &mut egui::Ui) {
        let fps = self.abs_time.frame_rate;
        crate::components::numpad::Numpad::new(self)
            .with_side_keys([Key::H, Key::M, Key::S, Key::F])
            .with_decimal(Key::Colon)
            .with_sign()
            .ui(ui);
        //self.abs_time.frame_rate = fps;
    }
}
