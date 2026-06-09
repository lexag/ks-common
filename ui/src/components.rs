use core::fmt::Display;
use egui::IntoAtoms;

pub fn selector_list_index<T>(
    ui: &mut egui::Ui,
    options: &[T],
    selected_idx: Option<usize>,
    label: &str,
) -> Option<usize>
where
    T: Display,
{
    let mut clicked = None;
    ui.vertical(|ui| {
        ui.heading(label);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_min_height(500.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, option) in options.iter().enumerate() {
                    if ui
                        .selectable_label(Some(i) == selected_idx, option.to_string())
                        .clicked()
                    {
                        clicked = Some(i);
                    }
                }
            });
        });
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
