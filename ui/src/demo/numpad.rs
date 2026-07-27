use crate::components::numpad::Numpad;
use egui::Widget;

pub(crate) fn demo_numpad(ui: &mut egui::Ui) {
    Numpad::new(&mut 0).with_sign().with_decimal(',').ui(ui);
}
