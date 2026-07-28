use crate::interface::AutoInlineWidgetMenu;

use egui::Widget;

pub(crate) fn demo_inline_primitive_menu(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        let mut val = ui.memory(|r| r.data.get_temp::<f64>(ui.id()).unwrap_or(0.0));
        //egui::DragValue::new(&mut val).ui(ui);
        ui.memory_mut(|w| *w.data.get_temp_mut_or(ui.id(), 0.0) = val);

        let mut val_u8: u8 = val as u8;
        let mut val_u16: u16 = val as u16;
        let mut val_u32: u32 = val as u32;
        let mut val_i16: i16 = val as i16;
        let mut val_i32: i32 = val as i32;
        let mut val_f32: f32 = val as f32;
        let mut val_f64: f64 = val;

        val_u8.auto_inline_widget_menu(ui, "u8");
        val_u16.auto_inline_widget_menu(ui, "u16");
        val_u32.auto_inline_widget_menu(ui, "u32");

        val_i16.auto_inline_widget_menu(ui, "i16");
        val_i32.auto_inline_widget_menu(ui, "i32");

        val_f32.auto_inline_widget_menu(ui, "f32");
        val_f64.auto_inline_widget_menu(ui, "f64");
        ui.memory_mut(|w| *w.data.get_temp_mut_or(ui.id(), 0.0) = val_f64);
    });
}
