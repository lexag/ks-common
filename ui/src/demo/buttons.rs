use crate::{
    components::Button,
    style::{ACCENT_COLOR, ACTIVE_COLOR, ERROR_COLOR, WARNING_COLOR},
};
use egui::Widget;

pub(crate) fn demo_buttons(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        Button::new("H").ui(ui);
        Button::new("ABC").ui(ui);
        Button::new("ind a").indicator(None).ui(ui);
        Button::new("ind b").indicator(Some(ACCENT_COLOR)).ui(ui);
        Button::new("ind c").indicator(Some(ACTIVE_COLOR)).ui(ui);
        ui.add_enabled(false, Button::new("dis"));
        ui.add_enabled(false, Button::new("dis2").indicator(Some(WARNING_COLOR)));
        Button::new("icon").icon(material_icons::Icon::Air).ui(ui);
        Button::new("icon2")
            .icon(material_icons::Icon::Water)
            .indicator(Some(ERROR_COLOR))
            .ui(ui);
    });
}
