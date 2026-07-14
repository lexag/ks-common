//! Implementable display traits for rendering types into egui

use egui::Response;

/// Small (status-bar sized) visual representation of a type.
/// Can be text-based or visual.
pub trait InlineWidget {
    /// Render function for this trait
    fn draw(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response;

    /// Main function for this trait. Call this from outside, and it handles the rest.
    fn inline_widget(&mut self, ui: &mut egui::Ui) -> egui::Response {
        self.inline_widget_scaled(ui, 1.4)
    }

    /// Alternative function, allowing non-standard scale for this widget
    fn inline_widget_scaled(&mut self, ui: &mut egui::Ui, scale: f32) -> egui::Response {
        let widget = self.draw(ui, scale);
        if widget.clicked() {
            self.on_click()
        }
        if widget.double_clicked() {
            self.on_double_click()
        }
        if widget.dragged() {
            let delta = widget.drag_delta();
            self.on_drag(&delta)
        }
        if widget.hovered() {
            self.on_hover()
        }

        widget
    }

    /// Override this function to implement click responses
    fn on_click(&mut self) {}

    /// Override this function to implement double click responses
    fn on_double_click(&mut self) {}

    /// Override this function to implement drag responses
    fn on_drag(&mut self, delta: &egui::Vec2) {
        let _ = delta;
    }

    /// Override this function to implement hover responses
    fn on_hover(&mut self) {}
}

/// Larger (no limit) visual representation of the configuration settings for a type.
/// Should most likely be a grid of properties and their values.
pub trait ConfigurationWidget {
    /// Main function for this trait. Call this from outside, and it handles the rest.
    fn draw_configuration(&mut self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::group(ui.style())
            .show(ui, |ui| {
                egui::Grid::new(ui.next_auto_id()).show(ui, |ui| self.grid_contents(ui));
            })
            .response
    }

    /// Recommended impl function for this trait. Implement this and call draw_configuration, and
    /// it handles the rest.
    fn grid_contents(&mut self, ui: &mut egui::Ui);
}

pub trait InlineWidgetMenu {
    /// Main function for this trait. Call this from outside, and it handles the rest.
    fn inline_widget_menu(
        &mut self,
        ui: &mut egui::Ui,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) -> Response;
}

impl<T> InlineWidgetMenu for T
where
    T: InlineWidget,
{
    fn inline_widget_menu(
        &mut self,
        ui: &mut egui::Ui,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) -> Response {
        let response = self.inline_widget(ui);
        let _ = egui::Popup::menu(&response)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .info(egui::UiStackInfo::new(egui::UiKind::Menu))
            .show(add_contents);
        response
    }
}
