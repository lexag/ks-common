use egui::{Align2, Color32, FontId, Pos2, Sense, Vec2};

pub const SEGMENTED_CHAR_WIDTH: f32 = 9.0;
pub const SEGMENTED_SEPR_WIDTH: f32 = 5.0;

pub fn draw_segmented_display(
    ui: &mut egui::Ui,
    char_widths: &[f32],
    text: &str,
    text_color: Color32,
    scale: f32,
) -> egui::Response {
    const MARGIN: f32 = 3.0;
    const CHAR_HEIGHT: f32 = 20.0;

    let mut widget_size: Vec2 = [
        char_widths.iter().sum::<f32>() + MARGIN * 2.0,
        CHAR_HEIGHT + MARGIN * 2.0,
    ]
    .into();

    widget_size *= scale;

    let (widget_response, p) = ui.allocate_painter(widget_size, Sense::click_and_drag());

    let full_rect = widget_response.rect;
    let text_rect = full_rect.shrink(MARGIN);

    p.rect(
        full_rect,
        ui.visuals().widgets.inactive.corner_radius,
        ui.visuals().widgets.inactive.bg_fill,
        ui.visuals().widgets.inactive.bg_stroke,
        egui::StrokeKind::Inside,
    );

    p.rect(
        text_rect,
        0.0,
        ui.visuals().extreme_bg_color,
        ui.visuals().widgets.inactive.bg_stroke,
        egui::StrokeKind::Outside,
    );

    let mut cursor = text_rect.left();
    for (character, width) in text.chars().zip(char_widths) {
        let glyph_center: Pos2 = [cursor + width * scale / 2.0, text_rect.center().y].into();
        p.text(
            glyph_center,
            Align2::CENTER_CENTER,
            character.to_string(),
            FontId::new(CHAR_HEIGHT * scale, egui::FontFamily::Name("LTC".into())),
            text_color,
        );
        cursor += width * scale;
    }

    widget_response
}
