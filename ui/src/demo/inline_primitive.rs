use crate::interface::InlineWidget;

use egui::Widget;

pub(crate) fn demo_inline_primitive(ui: &mut egui::Ui) {
    let mut val = ui.memory(|r| r.data.get_temp::<f64>(ui.id()).unwrap_or(0.0));
    egui::DragValue::new(&mut val).ui(ui);
    ui.memory_mut(|w| *w.data.get_temp_mut_or(ui.id(), 0.0) = val);

    demo_inline_primitive_displays(ui, val);
}

pub(crate) fn demo_inline_primitive_displays(ui: &mut egui::Ui, val: f64) {
    let mut val_u8: u8 = val as u8;
    let mut val_u16: u16 = val as u16;
    let mut val_u32: u32 = val as u32;
    let mut val_i16: i16 = val as i16;
    let mut val_i32: i32 = val as i32;
    let mut val_f32: f32 = val as f32;
    let mut val_f64: f64 = val;
    let mut val_char: char = val as u8 as char;
    let mut val_bool: bool = val > 0.0;

    ui.label("u8");
    val_u8.inline_widget(ui);
    ui.label("u16");
    val_u16.inline_widget(ui);
    ui.label("u32");
    val_u32.inline_widget(ui);

    ui.label("i16");
    val_i16.inline_widget(ui);
    ui.label("i32");
    val_i32.inline_widget(ui);

    ui.label("f32");
    val_f32.inline_widget(ui);
    ui.label("f64");
    val_f64.inline_widget(ui);

    ui.label("char");
    val_char.inline_widget(ui);
    ui.label("bool");
    val_bool.inline_widget(ui);
}
