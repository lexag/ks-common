use crate::{
    interface::SubstitutedAutoInlineWidgetMenu,
    traits::{AutoInlineWidgetMenu, ConfigurationWidget},
};

#[derive(Default)]
struct ComplexType {
    integer: u8,
    other_value: f32,
}

impl SubstitutedAutoInlineWidgetMenu<u8> for ComplexType {
    fn substitute(&self) -> u8 {
        self.integer
    }
}

impl ConfigurationWidget for ComplexType {
    fn grid_contents(&mut self, ui: &mut egui::Ui) {
        self.integer.auto_inline_widget_menu(ui, "integer");
        self.other_value.auto_inline_widget_menu(ui, "other_value");
    }
}

const VAL: ComplexType = ComplexType {
    integer: 42,
    other_value: 123.456,
};

pub fn demo_substitute(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.set_max_width(350.0);
        VAL.auto_inline_widget_menu(ui, "noclone");
    });
}
