use crate::{component_interface::InlineWidget, graphics::draw_segmented_display, style};
use egui::{Sense, Vec2};
use ks_common_generic::smpte::{Timecode, TimecodeOffset};

const CHAR: f32 = 9.0;
const SEPR: f32 = 5.0;
const CHAR_WIDTHS: &[f32] = &[
    CHAR, CHAR, SEPR, CHAR, CHAR, SEPR, CHAR, CHAR, SEPR, CHAR, CHAR, SEPR, CHAR, CHAR,
];
const CHAR_HEIGHT: f32 = 20.0;
const MARGIN: f32 = 3.0;

impl InlineWidget for Timecode {
    fn draw(&mut self, ui: &mut egui::Ui) -> egui::Response {
        draw_segmented_display(
            ui,
            CHAR_WIDTHS,
            CHAR_HEIGHT,
            MARGIN,
            &format!("{} {}", self, self.frame_rate.fps),
            style::ACCENT_COLOR,
        )
    }
}

impl InlineWidget for Option<Timecode> {
    fn draw(&mut self, ui: &mut egui::Ui) -> egui::Response {
        match self {
            Some(tc) => tc.draw(ui),
            None => draw_segmented_display(
                ui,
                CHAR_WIDTHS,
                CHAR_HEIGHT,
                MARGIN,
                "   NO    TC   ",
                style::WARNING_COLOR,
            ),
        }
    }
}

impl InlineWidget for TimecodeOffset {
    fn draw(&mut self, ui: &mut egui::Ui) -> egui::Response {
        draw_segmented_display(
            ui,
            CHAR_WIDTHS,
            CHAR_HEIGHT,
            MARGIN,
            &format!(
                " {} {}",
                if self.is_negative { '-' } else { '+' },
                self.abs_time
            ),
            ui.visuals().text_color(),
        )
    }
}
