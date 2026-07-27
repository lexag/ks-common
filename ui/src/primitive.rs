use crate::{
    graphics::{self, draw_segmented_display},
    interface::{ConfigurationWidget, InlineWidget},
    style,
};

use egui::Widget;

const CHAR: f32 = graphics::SEGMENTED_CHAR_WIDTH;
const SEPR: f32 = graphics::SEGMENTED_SEPR_WIDTH;

impl InlineWidget for f32 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, SEPR, CHAR, CHAR, CHAR],
            &format!("{:+08.3}", self.min(999.999)),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for f64 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, SEPR, CHAR, CHAR, CHAR],
            &format!("{:+08.3}", self.min(999.999)),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for bool {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[SEPR, CHAR, SEPR],
            if *self { " X " } else { "   " },
            style::ACCENT_COLOR,
            scale,
        )
    }

    fn on_click(&mut self) {
        *self = !*self;
    }
}

impl InlineWidget for u8 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR],
            &format!("{: >3}", self),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for u16 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, CHAR],
            &format!("{: >5}", self),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for u32 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR],
            &format!("{: >10}", self),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for i16 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[CHAR, CHAR, CHAR, CHAR, CHAR, CHAR],
            &format!("{}{: >5}", if *self < 0 { '-' } else { ' ' }, self.abs()),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for i32 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[
                CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR, CHAR,
            ],
            &format!("{}{: >10}", if *self < 0 { '-' } else { ' ' }, self.abs()),
            ui.visuals().text_color(),
            scale,
        )
    }
}

impl InlineWidget for char {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        draw_segmented_display(
            ui,
            &[SEPR, CHAR, SEPR],
            &format!(" {} ", self),
            ui.visuals().text_color(),
            scale,
        )
    }
}

// impl ConfigurationWidget for positive integers
macro_rules! impl_ConfigurationWidget_positive_int {
    (for $($t:ty),+) => {
        $(impl ConfigurationWidget for $t {
            fn draw_configuration(&mut self, ui: &mut egui::Ui) -> egui::Response {
                crate::components::numpad::Numpad::new(self).ui(ui)

            }
    fn grid_contents(&mut self, _: &mut egui::Ui) { unimplemented!()}
        })*
    }
}
// impl ConfigurationWidget for negative integers
macro_rules! impl_ConfigurationWidget_negative_int {
    (for $($t:ty),+) => {
        $(impl ConfigurationWidget for $t {
            fn draw_configuration(&mut self, ui: &mut egui::Ui) -> egui::Response {
                crate::components::numpad::Numpad::new(self).with_sign().ui(ui)

            }
    fn grid_contents(&mut self, _: &mut egui::Ui) { unimplemented!()}
        })*
    }
}
// impl ConfigurationWidget for floats
macro_rules! impl_ConfigurationWidget_float {
    (for $($t:ty),+) => {
        $(impl ConfigurationWidget for $t {
            fn draw_configuration(&mut self, ui: &mut egui::Ui) -> egui::Response {
                crate::components::numpad::Numpad::new(self).with_sign().with_decimal('.').ui(ui)

            }
    fn grid_contents(&mut self, _: &mut egui::Ui) { unimplemented!()}
        })*
    }
}
impl_ConfigurationWidget_positive_int!(for u8, u16, u32, u64, u128);
impl_ConfigurationWidget_negative_int!(for i8, i16, i32, i64, i128);
impl_ConfigurationWidget_float!(for f32, f64);
