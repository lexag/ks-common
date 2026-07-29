use crate::{components, interface::InlineWidget};
use egui::Widget;
use ks_common_generic::str::StaticString;

impl<const L: usize> InlineWidget for StaticString<L> {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(self.str(), L)
            .label(label)
            .ui(ui)
    }
}
