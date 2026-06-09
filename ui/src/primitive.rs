use crate::{
    component_interface::InlineWidget,
    graphics::{self, draw_segmented_display},
    style,
};

const CHAR: f32 = graphics::SEGMENTED_CHAR_WIDTH;
const SEPR: f32 = graphics::SEGMENTED_SEPR_WIDTH;

impl InlineWidget for f32 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, SEPR, CHAR, CHAR, CHAR],
            &format!("{:+08.3}", self),
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
