use crate::style::{self, CORNER_RADIUS, auto_bg_stroke, auto_fg_fill};
use egui::{Align2, Color32, Rect, Sense, Vec2, Widget, vec2};

pub struct Button<'a> {
    text: &'a str,
    override_fill: Option<Color32>,
    indicator: Option<Color32>,
}

impl<'a> Button<'a> {
    pub(crate) const SQUARE_BUTTON_WIDTH: f32 = 64.0;

    const SIZE: Vec2 = Vec2::splat(Self::SQUARE_BUTTON_WIDTH);

    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            override_fill: None,
            indicator: None,
        }
    }

    pub fn fill(mut self, col: Color32) -> Self {
        self.override_fill = Some(col);
        self
    }

    /// Add a colored status indicator on the button. None means indicator is present but not lit.
    /// Some(col) means indicator is present and lit with col.
    pub fn indicator(mut self, col: Option<Color32>) -> Self {
        self.indicator = col.or(Some(Color32::BLACK));
        self
    }
}

impl<'a> Widget for Button<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (resp, p) = ui.allocate_painter(Self::SIZE, Sense::click());
        let body_rect = resp.rect;

        // Main button body
        p.rect(
            body_rect,
            CORNER_RADIUS,
            self.override_fill.unwrap_or_else(|| auto_fg_fill(&resp)),
            auto_bg_stroke(&resp),
            egui::StrokeKind::Inside,
        );

        p.text(
            body_rect.center()
                + if self.indicator.is_some() {
                    vec2(0.0, -4.0)
                } else {
                    Vec2::ZERO
                },
            Align2::CENTER_CENTER,
            self.text,
            style::font_button(),
            ui.visuals().widgets.inactive.fg_stroke.color,
        );

        // colored indicator
        if let Some(mut indicator_color) = self.indicator {
            if !resp.enabled() {
                indicator_color = indicator_color.gamma_multiply(ui.visuals().disabled_alpha);
            }

            let indicator_center = body_rect.lerp_inside([0.5, 0.8]);
            const SIZE: Vec2 = vec2(32.0, 8.0);
            let indicator_rect = Rect::from_center_size(indicator_center, SIZE);
            p.rect_filled(indicator_rect, 0, indicator_color);

            if indicator_color != Color32::BLACK {
                for i in 1..5 {
                    p.rect_filled(
                        indicator_rect.expand(0.5 * i as f32),
                        i,
                        indicator_color.gamma_multiply(0.5 / i as f32),
                    );
                }
            }
        }

        resp
    }
}
