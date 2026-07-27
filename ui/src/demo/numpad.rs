use crate::{components::numpad::Numpad, demo::inline_primitive::demo_inline_primitive_displays};
use egui::{Key, Widget};

pub(crate) fn demo_numpad(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        let mut val = ui.memory(|r| r.data.get_temp::<f64>(ui.id()).unwrap_or(0.0));

        Numpad::new(&mut val)
            .with_sign()
            .with_decimal(Key::Period)
            .with_side_keys([Key::H, Key::M, Key::S, Key::F])
            .ui(ui);
        demo_inline_primitive_displays(ui, val);
        ui.memory_mut(|w| *w.data.get_temp_mut_or(ui.id(), 0.0) = val);
    });
}
