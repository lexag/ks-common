use crate::{
    component_interface::{ConfigurationWidget, InlineWidget},
    graphics::draw_segmented_display,
    style,
};
use egui::{Sense, Vec2, Widget};
use ks_common_generic::smpte::{Timecode, TimecodeOffset};

const CHAR: f32 = 9.0;
const SEPR: f32 = 5.0;
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

impl ConfigurationWidget for TimecodeOffset {
    fn grid_contents(&mut self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::group(ui.style())
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .button(if self.is_negative { "-" } else { "+" })
                        .clicked()
                    {
                        self.is_negative = !self.is_negative;
                    };
                    let mut hours = self.abs_time.hours;
                    let mut minutes = self.abs_time.minutes;
                    let mut seconds = self.abs_time.seconds;
                    let mut frames = self.abs_time.frames;
                    for (val, suffix, max) in [
                        (&mut hours, "h", 23),
                        (&mut minutes, "m", 59),
                        (&mut seconds, "s", 59),
                        (&mut frames, "f", self.abs_time.frame_rate.fps),
                    ] {
                        egui::DragValue::new(val)
                            .suffix(suffix)
                            .range(0..=max as i32)
                            .speed(0.01)
                            .ui(ui);
                    }
                    self.abs_time = Timecode::from_raw_fields(
                        hours,
                        minutes,
                        seconds,
                        frames,
                        self.abs_time.frame_rate.fps,
                        self.abs_time.frame_rate.drop_frame,
                        0,
                    );
                });
            })
            .response
    }
}
