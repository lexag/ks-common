#![allow(dead_code)]

mod inline_primitive;
mod inline_primitive_menu;
mod numpad;

pub fn demo_ui(ui: &mut egui::Ui) {
    inline_primitive_menu::demo_inline_primitive_menu(ui);
}
