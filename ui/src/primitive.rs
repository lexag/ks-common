use crate::{
    components,
    interface::{ConfigurationWidget, InlineWidget},
    style,
};

use egui::Widget;

impl InlineWidget for f32 {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(&self.to_string(), 8)
            .label(label)
            .ui(ui)
    }
}

impl InlineWidget for f64 {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(&self.to_string(), 8)
            .label(label)
            .ui(ui)
    }
}

impl InlineWidget for bool {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(if *self { "X" } else { "" }, 3)
            .align(egui::Align::Center)
            .color(style::ACCENT_COLOR)
            .label(label)
            .ui(ui)
    }

    fn on_click(&mut self) {
        *self = !*self;
    }
}

impl InlineWidget for u8 {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(&self.to_string(), 3)
            .align(egui::Align::Center)
            .label(label)
            .ui(ui)
    }
}

impl InlineWidget for u16 {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(&self.to_string(), 5)
            .align(egui::Align::Center)
            .label(label)
            .ui(ui)
    }
}

impl InlineWidget for u32 {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(&self.to_string(), 10)
            .label(label)
            .ui(ui)
    }
}

impl InlineWidget for i16 {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(&self.to_string(), 6)
            .label(label)
            .ui(ui)
    }
}

impl InlineWidget for i32 {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(&self.to_string(), 11)
            .label(label)
            .ui(ui)
    }
}

impl InlineWidget for char {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        components::TextDisplay::new(&self.to_string(), 3)
            .align(egui::Align::Center)
            .label(label)
            .ui(ui)
    }
}

// impl ConfigurationWidget for positive integers
macro_rules! impl_ConfigurationWidget_positive_int {
    (for $($t:ty),+) => {
        $(impl ConfigurationWidget for $t {
            fn grid_contents(&mut self, ui: &mut egui::Ui) {
                crate::components::Numpad::new(self).ui(ui);

            }
        })*
    }
}
// impl ConfigurationWidget for negative integers
macro_rules! impl_ConfigurationWidget_negative_int {
    (for $($t:ty),+) => {
        $(impl ConfigurationWidget for $t {
            fn grid_contents(&mut self, ui: &mut egui::Ui) {
                crate::components::Numpad::new(self).with_sign().ui(ui);

            }
        })*
    }
}
// impl ConfigurationWidget for floats
macro_rules! impl_ConfigurationWidget_float {
    (for $($t:ty),+) => {
        $(impl ConfigurationWidget for $t {
            fn grid_contents(&mut self, ui: &mut egui::Ui) {
                crate::components::Numpad::new(self).with_sign().with_decimal(egui::Key::Period).ui(ui);

            }
        })*
    }
}
impl_ConfigurationWidget_positive_int!(for u8, u16, u32, u64, u128);
impl_ConfigurationWidget_negative_int!(for i8, i16, i32, i64, i128);
impl_ConfigurationWidget_float!(for f32, f64);
