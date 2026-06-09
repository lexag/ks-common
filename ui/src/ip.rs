use crate::{
    component_interface::{ConfigurationWidget, InlineWidget},
    graphics::{self, draw_segmented_display},
    style,
};
use core::net::SocketAddrV4;
use egui::Widget;
use std::net::Ipv4Addr;

const CHAR: f32 = graphics::SEGMENTED_CHAR_WIDTH;
const SEPR: f32 = graphics::SEGMENTED_SEPR_WIDTH;

impl InlineWidget for SocketAddrV4 {
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        let octets = self.ip().octets();
        draw_segmented_display(
            ui,
            &[
                CHAR, CHAR, CHAR, SEPR, CHAR, CHAR, CHAR, SEPR, CHAR, CHAR, CHAR, SEPR, CHAR, CHAR,
                CHAR, SEPR, CHAR, CHAR, CHAR, CHAR, CHAR,
            ],
            &format!(
                "{:>3}.{:>3}.{:>3}.{:>3}:{:>5}",
                octets[0],
                octets[1],
                octets[2],
                octets[3],
                self.port()
            ),
            style::ACCENT_COLOR,
            scale,
        )
    }
}

impl ConfigurationWidget for SocketAddrV4 {
    fn grid_contents(&mut self, ui: &mut egui::Ui) {
        let mut octets = self.ip().octets();
        ui.label("Address");
        ui.horizontal(|ui| {
            egui::DragValue::new(&mut octets[0]).ui(ui);
            egui::DragValue::new(&mut octets[1]).ui(ui);
            egui::DragValue::new(&mut octets[2]).ui(ui);
            egui::DragValue::new(&mut octets[3]).ui(ui);
        });
        ui.end_row();
        let mut port = self.port();
        ui.label("Port");
        egui::DragValue::new(&mut port).ui(ui);

        self.set_ip(Ipv4Addr::from_octets(octets));
        self.set_port(port);
    }
}
