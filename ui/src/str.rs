use crate::{
    component_interface::InlineWidget,
    graphics::{SEGMENTED_CHAR_WIDTH, draw_segmented_display},
};
use ks_common_generic::str::StaticString;

const CHAR: f32 = SEGMENTED_CHAR_WIDTH;

impl<const L: usize> InlineWidget for StaticString<L> {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(ui, &[CHAR; L], self.str(), ui.visuals().text_color(), scale)
    }
}
