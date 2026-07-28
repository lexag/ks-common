use egui::Widget;
use std::fmt::Display;

use crate::components::Button;
use crate::components::SquarishGrid;
use crate::style;

pub fn selector_list_index<T>(
    ui: &mut egui::Ui,
    options: &[T],
    selected_idx: Option<usize>,
    label: &str,
) -> Option<usize>
where
    T: Display,
{
    let option_strings = options.iter().map(|s| s.to_string());

    let max_strlen = option_strings
        .clone()
        .map(|s| s.len())
        .max()
        .unwrap_or_default();

    let mut clicked = None;

    let mut grid = SquarishGrid::new(options.len());

    grid.show(ui, |ui, grid| {
        for (i, option) in option_strings.enumerate() {
            grid.add(ui, |ui| {
                let button = if max_strlen > 4 {
                    Button::wide(option)
                } else {
                    Button::new(option)
                }
                .indicator((Some(i) == selected_idx).then_some(style::ACCENT_COLOR));
                if button.ui(ui).clicked() {
                    clicked = Some(i);
                }
            });
        }
    });

    clicked
}

pub fn selector_list_value<T>(
    ui: &mut egui::Ui,
    options: &[T],
    selected_value: &T,
    label: &str,
) -> Option<T>
where
    T: Display + PartialEq + Clone,
{
    if let Some(i) = selector_list_index(
        ui,
        options,
        options.iter().position(|v| v == selected_value),
        label,
    ) {
        return Some(options[i].clone());
    }
    None
}
