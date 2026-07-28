use crate::{
    components::Button,
    style::{ACCENT_COLOR, ACTIVE_COLOR, WARNING_COLOR},
};
use egui::Widget;

pub(crate) fn demo_buttons(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        Button::new("hi").ui(ui);
        Button::new("ind a").indicator(None).ui(ui);
        Button::new("ind b").indicator(Some(ACCENT_COLOR)).ui(ui);
        Button::new("ind c").indicator(Some(ACTIVE_COLOR)).ui(ui);
        ui.add_enabled(false, Button::new("dis"));
        ui.add_enabled(false, Button::new("dis2").indicator(Some(WARNING_COLOR)));
    });
}
