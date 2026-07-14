use crate::style;
use core::fmt::Display;
use egui::{Sense, Vec2, Widget};

const SELECTOR_LIST_WIDTH: f32 = 250.0;
const SELECTOR_LIST_MIN_HEIGHT: f32 = 500.0;
const SELECTOR_LIST_MIN_ITEM_HEIGHT: f32 = 64.0;

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
            ui.set_min_height(SELECTOR_LIST_MIN_HEIGHT);
            ui.set_width(SELECTOR_LIST_WIDTH);
            egui::ScrollArea::vertical().id_salt(label).show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    for (i, option) in options.iter().enumerate() {
                        let label =
                            egui::Button::selectable(Some(i) == selected_idx, option.to_string())
                                .wrap()
                                .min_size(
                                    [SELECTOR_LIST_WIDTH, SELECTOR_LIST_MIN_ITEM_HEIGHT].into(),
                                )
                                .ui(ui);
                        if label.clicked() {
                            clicked = Some(i);
                        }
                    }
                });
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

pub fn big_slider(ui: &mut egui::Ui, progress: f32) -> Option<f32> {
    let size: Vec2 = [SELECTOR_LIST_WIDTH * 0.5, SELECTOR_LIST_MIN_HEIGHT].into();
    let (resp, p) = ui.allocate_painter(size, Sense::drag());
    let full_rect = resp.rect;

    p.rect(
        full_rect,
        ui.visuals().widgets.inactive.corner_radius,
        ui.visuals().widgets.inactive.bg_fill,
        ui.visuals().widgets.inactive.bg_stroke,
        egui::StrokeKind::Outside,
    );

    let (_, fill_rect) = full_rect.split_top_bottom_at_fraction(1.0 - progress);
    p.rect_filled(
        fill_rect,
        ui.visuals().widgets.inactive.corner_radius,
        style::ACCENT_COLOR,
    );

    if resp.dragged()
        && let Some(pos) = ui.input(|i| i.pointer.hover_pos())
    {
        let percent = -(pos.y - full_rect.bottom()) / full_rect.height();
        return Some(percent);
    }
    None
}
