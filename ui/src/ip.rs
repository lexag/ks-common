use crate::{
    components,
    interface::{ConfigurationWidget, InlineWidget},
    style,
};
use core::net::Ipv4Addr;
use core::net::SocketAddrV4;
use egui::Widget;

impl InlineWidget for SocketAddrV4 {
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        let octets = self.ip().octets();
        components::TextDisplay::new(&self.to_string(), 21)
            .label(&label)
            .ui(ui)
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
