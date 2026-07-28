use crate::{
    graphics::{SEGMENTED_CHAR_WIDTH, draw_segmented_display},
    interface::InlineWidget,
};
use ks_common_generic::str::StaticString;

const CHAR: f32 = SEGMENTED_CHAR_WIDTH;

impl<const L: usize> InlineWidget for StaticString<L> {
    fn draw(&mut self, ui: &mut egui::Ui, label: String) -> egui::Response {
        draw_segmented_display(ui, L, self.str(), ui.visuals().text_color(), label)
    }
}
