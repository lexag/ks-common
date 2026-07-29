use crate::graphics;
use crate::style;
use egui::FontId;
use egui::vec2;
use egui::{Align, Align2, Color32, Widget};

pub struct TextDisplay<'a> {
    text: &'a str,
    max_len: usize,
    label: Option<&'a str>,
    align: Option<Align>,
    color: Option<Color32>,
}

impl<'a> TextDisplay<'a> {
    pub fn new(text: &'a str, max_len: usize) -> Self {
        Self {
            text,
            max_len,
            label: None,
            align: None,
            color: None,
        }
    }

    pub fn align(self, align: Align) -> Self {
        self.align_o(Some(align))
    }
    pub fn color(self, color: Color32) -> Self {
        self.color_o(Some(color))
    }
    pub fn label(self, label: &'a str) -> Self {
        self.label_o(Some(label))
    }

    pub fn align_o(mut self, align: Option<Align>) -> Self {
        self.align = align;
        self
    }
    pub fn color_o(mut self, color: Option<Color32>) -> Self {
        self.color = color;
        self
    }
    pub fn label_o(mut self, label: Option<&'a str>) -> Self {
        self.label = label;
        self
    }
}

impl<'a> Widget for TextDisplay<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let actual_text_width = self.text.chars().map(graphics::char_width).sum::<f32>();
        let max_text_width = self.max_len as f32 * graphics::char_width('M');

        let (resp, p) =
            graphics::allocate_display_box(ui, self.label.unwrap_or_default(), max_text_width);

        let text_rect = p.clip_rect();
        let align2 = Align2([self.align.unwrap_or(Align::Min), Align::Center]);
        let actual_text_rect = align2.anchor_size(
            align2.pos_in_rect(&text_rect),
            vec2(actual_text_width, text_rect.height()),
        );
        let mut cursor = actual_text_rect.left_center();

        for character in self.text.chars() {
            let width = graphics::char_width(character);
            p.text(
                cursor,
                Align2::LEFT_CENTER,
                character.to_string(),
                FontId::new(graphics::CHAR_HEIGHT, egui::FontFamily::Name("LTC".into())),
                self.color
                    .unwrap_or_else(|| style::widgets().noninteractive.fg_stroke.color),
            );
            cursor += vec2(width, 0.0);
        }

        resp
    }
}
