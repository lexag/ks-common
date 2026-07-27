use crate::{
    components::big_slider,
    graphics::{self, draw_segmented_display},
    interface::{ConfigurationWidget, InlineWidget},
    style,
};

const CHAR: f32 = graphics::SEGMENTED_CHAR_WIDTH;
const SEPR: f32 = graphics::SEGMENTED_SEPR_WIDTH;

impl InlineWidget for f32 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, SEPR, CHAR, CHAR, CHAR],
            &format!("{:+08.3}", self.min(999.999)),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for f64 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, SEPR, CHAR, CHAR, CHAR],
            &format!("{:+08.3}", self.min(999.999)),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for bool {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[SEPR, CHAR, SEPR],
            if *self { " X " } else { "   " },
            style::ACCENT_COLOR,
            scale,
        )
    }

    fn on_click(&mut self) {
        *self = !*self;
    }
}

impl InlineWidget for u8 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR],
            &format!("{: >3}", self),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl ConfigurationWidget for u8 {
    fn grid_contents(&mut self, ui: &mut egui::Ui) {
        if let Some(fraction) = big_slider(ui, (*self as f32) / 255.0) {
            *self = (fraction * 255.0) as Self;
        }
    }
}

impl InlineWidget for u16 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, CHAR],
            &format!("{: >5}", self),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for u32 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR],
            &format!("{: >10}", self),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for i16 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, CHAR, CHAR],
            &format!("{}{: >5}", if *self < 0 { '-' } else { ' ' }, self),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for i32 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[
                CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR,
            ],
            &format!("{}{: >5}", if *self < 0 { '-' } else { ' ' }, self),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for char {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR],
            &self.to_string(),
            ui.visuals().text_color(),
            scale,
        )
    }
}
