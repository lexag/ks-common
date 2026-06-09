use egui::{Align2, Color32, FontId, Pos2, Sense, Vec2};
use std::fmt::Display;

pub fn draw_segmented_display(
    ui: &mut egui::Ui,
    char_widths: &[f32],
    char_height: f32,
    margin: f32,
    text: &str,
    text_color: Color32,
) -> egui::Response {
    let widget_size: Vec2 = [
        char_widths.iter().sum::<f32>() + margin * 2.0,
        char_height + margin * 2.0,
    ]
    .into();

    let (widget_response, p) = ui.allocate_painter(widget_size, Sense::click_and_drag());

    let full_rect = widget_response.rect;
    let text_rect = full_rect.shrink(margin);

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
        let glyph_center: Pos2 = [cursor + width / 2.0, text_rect.center().y].into();
        p.text(
            glyph_center,
            Align2::CENTER_CENTER,
            character.to_string(),
            FontId::new(char_height, egui::FontFamily::Name("LTC".into())),
            text_color,
        );
        cursor += width;
    }

    widget_response
}
