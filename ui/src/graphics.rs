use crate::{components::Button, style};
use core::ops::{Add, Div, Mul};
use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Vec2};

pub const SEGMENTED_CHAR_WIDTH: f32 = 12.5;
pub const SEGMENTED_SEPR_WIDTH: f32 = 7.0;

fn char_width(char: char) -> f32 {
    if "!|,.;:'`´".contains(char) {
        SEGMENTED_SEPR_WIDTH
    } else {
        SEGMENTED_CHAR_WIDTH
    }
}

pub fn draw_segmented_display(
    ui: &mut egui::Ui,
    digits: usize,
    text: &str,
    text_color: Color32,
    label: String,
) -> egui::Response {
    const MARGIN: f32 = 3.0;
    const CHAR_HEIGHT: f32 = 30.0;

    let actual_text_width = text.chars().map(char_width).sum::<f32>();
    let max_text_width = digits as f32 * char_width('M');
    let mut field_size: Vec2 = [max_text_width + MARGIN * 2.0, CHAR_HEIGHT + MARGIN * 2.0].into();

    let widget_width = if field_size.x + 2.0 * MARGIN < Button::SQUARE_BUTTON_WIDTH {
        Button::SQUARE_BUTTON_WIDTH
    } else {
        (field_size.x - Button::SQUARE_BUTTON_WIDTH)
            .div(Button::SQUARE_BUTTON_WIDTH + style::spacing().item_spacing.x)
            .ceil()
            .mul(Button::SQUARE_BUTTON_WIDTH + style::spacing().item_spacing.x)
            .add(Button::SQUARE_BUTTON_WIDTH)
    };

    field_size.x = widget_width - MARGIN * 2.0;

    let widget_size = [widget_width, Button::SQUARE_BUTTON_WIDTH].into();

    let (widget_response, p) = ui.allocate_painter(widget_size, Sense::click_and_drag());

    let full_rect = widget_response.rect;
    let field_rect = Rect::from_center_size(full_rect.lerp_inside([0.5, 0.66]), field_size);
    let text_rect = field_rect.shrink(MARGIN);

    p.rect(
        full_rect,
        ui.visuals().widgets.inactive.corner_radius,
        style::auto_fg_fill(&widget_response),
        style::auto_bg_stroke(&widget_response),
        egui::StrokeKind::Inside,
    );

    p.rect(
        field_rect,
        0.0,
        ui.visuals().extreme_bg_color,
        ui.visuals().widgets.noninteractive.bg_stroke,
        egui::StrokeKind::Outside,
    );

    p.text(
        full_rect.lerp_inside([0.5, 0.2]),
        Align2::CENTER_CENTER,
        label,
        style::font_label(),
        style::auto_fg_stroke(&widget_response).color,
    );

    let mut cursor = text_rect.right() - actual_text_width;
    for character in text.chars() {
        let width = char_width(character);
        let glyph_center: Pos2 = [cursor + width / 2.0, text_rect.center().y].into();
        p.text(
            glyph_center,
            Align2::CENTER_CENTER,
            character.to_string(),
            FontId::new(CHAR_HEIGHT, egui::FontFamily::Name("LTC".into())),
            text_color,
        );
        cursor += width;
    }

    widget_response
}
