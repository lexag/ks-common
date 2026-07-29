use crate::{components::Button, style};
use core::ops::{Add, Div, Mul, Sub};
use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Vec2};

pub const SEGMENTED_CHAR_WIDTH: f32 = 10.0;
pub const SEGMENTED_SEPR_WIDTH: f32 = 7.0;

pub fn char_width(char: char) -> f32 {
    SEGMENTED_CHAR_WIDTH
    //if "!|,.;:'`´[](){}".contains(char) {
    //    SEGMENTED_SEPR_WIDTH
    //} else {
    //    SEGMENTED_CHAR_WIDTH
    //}
}

pub const MARGIN: f32 = 3.0;
pub const CHAR_HEIGHT: f32 = 28.0;

/// Allocate a ui element with a label and a dark display box to draw text or other content inside
/// resp.rect contains the entire element
/// p.clip_rect() is limited to the content area of the display
pub fn allocate_display_box(
    ui: &mut egui::Ui,
    label: &str,
    max_text_width: f32,
) -> (egui::Response, egui::Painter) {
    let mut field_size: Vec2 = [max_text_width + MARGIN * 2.0, CHAR_HEIGHT + MARGIN * 2.0].into();
    let (resp, mut p) = allocate_element_with_surface(ui, field_size);

    let full_rect = p.clip_rect();
    field_size.x = full_rect.width();
    let field_rect = Rect::from_center_size(full_rect.lerp_inside([0.5, 0.66]), field_size);
    let text_rect = field_rect.shrink(MARGIN);

    // field
    p.rect(
        field_rect,
        0.0,
        ui.visuals().extreme_bg_color,
        ui.visuals().widgets.noninteractive.bg_stroke,
        egui::StrokeKind::Outside,
    );

    // label
    p.text(
        full_rect.lerp_inside([0.5, 0.2]),
        Align2::CENTER_CENTER,
        label,
        style::font_label(),
        style::auto_fg_stroke(&resp).color,
    );
    p.shrink_clip_rect(text_rect);
    (resp, p)
}

/// Allocate a ui element with drawn background in the default style
///
/// p is clipped to an inner content Rect.
/// resp.rect is the entire widget rect, including margins/gutters
fn allocate_element_with_surface(
    ui: &mut egui::Ui,
    min_size: Vec2,
) -> (egui::Response, egui::Painter) {
    allocate_element_with_surface_fill(ui, min_size, None)
}

/// Allocate a ui element with drawn background in the provided fill color
///
/// p is clipped to an inner content Rect.
/// resp.rect is the entire widget rect, including margins/gutters
fn allocate_element_with_surface_fill(
    ui: &mut egui::Ui,
    min_size: Vec2,
    fill: Option<Color32>,
) -> (egui::Response, egui::Painter) {
    let size = round_size_up(min_size);
    //println!("Asked for {}, got {}", min_size, size);

    let (resp, mut p) = ui.allocate_painter(size, Sense::click_and_drag());

    let content_rect = resp.rect.shrink(MARGIN);
    //println!(
    //    "widget shold be {},is {}, content is {}, cliprect is {}",
    //    size,
    //    resp.rect.size(),
    //    content_rect.size(),
    //    p.clip_rect().size()
    //);

    // background
    p.rect(
        resp.rect,
        ui.visuals().widgets.inactive.corner_radius,
        fill.unwrap_or_else(|| style::auto_fg_fill(&resp)),
        style::auto_bg_stroke(&resp),
        egui::StrokeKind::Inside,
    );
    p.set_clip_rect(content_rect);
    (resp, p)
}

fn round_size_up(min_size: Vec2) -> Vec2 {
    const SQUARE_SIZE: Vec2 = Vec2::splat(Button::SQUARE_BUTTON_WIDTH);
    let grow_step = Vec2::splat(Button::SQUARE_BUTTON_WIDTH) + style::spacing().item_spacing;
    min_size
        .add(Vec2::splat(2.0 * MARGIN))
        .sub(SQUARE_SIZE)
        .max(Vec2::ZERO)
        .div(grow_step)
        .ceil()
        .mul(grow_step)
        .add(SQUARE_SIZE)
}
