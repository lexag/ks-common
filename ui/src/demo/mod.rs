#![allow(dead_code)]

mod inline_primitive;
mod inline_primitive_menu;
mod numpad;

pub fn demo_ui(ui: &mut egui::Ui) {
    egui::ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal_top(|ui| {
            numpad::demo_numpad(ui);
            inline_primitive_menu::demo_inline_primitive_menu(ui);
        });
    });
}
