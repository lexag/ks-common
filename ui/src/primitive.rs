use crate::{component_interface::InlineWidget, graphics::draw_segmented_display};

impl InlineWidget for f32 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        const CHAR: f32 = 9.0;
        const SEPR: f32 = 5.0;
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, SEPR, CHAR, CHAR, CHAR],
            &format!("{:+08.3}", self),
            ui.visuals().text_color(),
            scale,
        )
    }
}
