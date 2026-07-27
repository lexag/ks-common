mod inline_primitive;
mod numpad;

pub fn demo_ui(ui: &mut egui::Ui) {
    numpad::demo_numpad(ui);
}
