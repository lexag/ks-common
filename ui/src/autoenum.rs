use crate::{
    components::{self, selector_list_value},
    interface::{ConfigurationWidget, InlineWidget, InlineWidgetMenu},
};
use core::fmt::Display;
use egui::Color32;
use egui::Widget;

pub trait InlineWidgetAutoEnum {
    fn options() -> Vec<Self>
    where
        Self: Sized + Display;

    fn color(&self) -> Option<Color32> {
        None
    }

    fn text(&self) -> Option<String> {
        None
    }

    fn autoenum_inline_widget(&mut self, ui: &mut egui::Ui, label: &str)
    where
        Self: InlineWidget,
    {
        self.inline_widget(ui, label);
    }

    fn autoenum_inline_widget_menu(&mut self, ui: &mut egui::Ui, label: &str)
    where
        Self: InlineWidgetMenu + Clone + ConfigurationWidget,
    {
        self.clone().inline_widget_menu(ui, label, |ui| {
            self.draw_configuration(ui);
        });
    }
}

impl<T> InlineWidget for T
where
    T: InlineWidgetAutoEnum + Display,
{
    fn draw(&mut self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        let options = T::options();
        let text = self.text().unwrap_or(self.to_string());
        let lengths = options
            .iter()
            .map(|s| s.text().unwrap_or(s.to_string()).len());
        let max_width = if options.is_empty() {
            0
        } else {
            lengths.clone().max().expect("Not empty")
        };

        components::TextDisplay::new(&text, max_width)
            .color_o(self.color())
            .align(egui::Align::Center)
            .label(label)
            .ui(ui)
    }
}

impl<T> ConfigurationWidget for T
where
    T: InlineWidgetAutoEnum + Clone + PartialEq + Display,
{
    fn draw_configuration(&mut self, ui: &mut egui::Ui) -> egui::Response {
        if let Some(selection) = selector_list_value(ui, &T::options(), self) {
            *self = selection;
        }
        // FIXME: this is not the right response, but we've kind of painted ourselves into a corner
        // by requiring Response returns for some but not all UI elements, and so in this case we
        // have none to return.
        ui.response()
    }

    fn grid_contents(&mut self, _ui: &mut egui::Ui) {
        unimplemented!()
    }
}
