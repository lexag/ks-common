use crate::{
    components::{Button, ToggleButton},
    style::{ACCENT_COLOR, ACTIVE_COLOR, ERROR_COLOR, WARNING_COLOR},
};
use egui::Widget;

pub(crate) fn demo_buttons(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        let mut val = ui.memory(|r| r.data.get_temp::<bool>(ui.id()).unwrap_or(false));
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
        ToggleButton::new(&mut val, material_icons::Icon::ArrowBack, ACTIVE_COLOR).ui(ui);
        ui.memory_mut(|w| *w.data.get_temp_mut_or(ui.id(), false) = val);
    });
}
