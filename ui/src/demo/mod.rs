#![allow(dead_code)]

mod autoenum;
mod buttons;
mod inline_primitive;
mod inline_primitive_menu;
mod numpad;
mod timecode;

pub fn demo_ui(ui: &mut egui::Ui) {
    egui::ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal_top(|ui| {
            inline_primitive_menu::demo_inline_primitive_menu(ui);
            timecode::demo_timecode(ui);
            buttons::demo_buttons(ui);
            autoenum::demo_autoenum(ui);
        });
    });
}
