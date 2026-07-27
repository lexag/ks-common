use crate::style;

use egui::Vec2;

use crate::components::SELECTOR_LIST_MIN_HEIGHT;
use crate::components::SELECTOR_LIST_WIDTH;

use egui::Sense;

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
