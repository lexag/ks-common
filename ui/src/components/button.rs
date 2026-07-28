use crate::style::{self, CORNER_RADIUS, auto_bg_stroke, auto_fg_fill};
use egui::{Align2, Color32, Key, Rect, Sense, Vec2, Widget, vec2};
use material_icons::Icon;

pub struct Button {
    text: String,
    icon: Option<char>,
    override_fill: Option<Color32>,
    indicator: Option<Color32>,
    width: f32,
}

impl Button {
    pub(crate) const SQUARE_BUTTON_WIDTH: f32 = 64.0;

    const SIZE: Vec2 = Vec2::splat(Self::SQUARE_BUTTON_WIDTH);

    pub fn new(text: impl ToString) -> Self {
        let s = text.to_string();
        let icon = if s.chars().count() == 1 {
            s.chars().next()
        } else {
            None
        };
        Self {
            text: s,
            icon,
            override_fill: None,
            indicator: None,
            width: Self::SQUARE_BUTTON_WIDTH,
        }
    }

    pub fn keyboard_key(key: Key) -> Self {
        Self::new(match key {
            Key::Backspace => Icon::Backspace.to_string(),
            Key::Enter => Icon::CheckCircle.to_string(),
            Key::Escape => Icon::Cancel.to_string(),
            Key::ArrowUp => Icon::ArrowUpward.to_string(),
            Key::ArrowDown => Icon::ArrowDownward.to_string(),
            Key::ArrowLeft => Icon::ArrowLeft.to_string(),
            Key::ArrowRight => Icon::ArrowRight.to_string(),
            k => k.symbol_or_name().to_string(),
        })
    }

    pub fn wide(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
            icon: None,
            override_fill: None,
            indicator: None,
            width: 3.0 * Self::SQUARE_BUTTON_WIDTH + 2.0 * style::spacing().item_spacing.x,
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

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (resp, p) =
            ui.allocate_painter(vec2(self.width, Self::SQUARE_BUTTON_WIDTH), Sense::click());
        let body_rect = resp.rect;
        let indicator_center = body_rect.lerp_inside([0.5, 0.8]);

        // Main button body
        p.rect(
            body_rect,
            CORNER_RADIUS,
            self.override_fill.unwrap_or_else(|| auto_fg_fill(&resp)),
            auto_bg_stroke(&resp),
            egui::StrokeKind::Inside,
        );

        let mut text_rect = body_rect;
        if self.indicator.is_some() {
            if self.icon.is_some() {
                *text_rect.bottom_mut() = indicator_center.y;
            } else {
                *text_rect.bottom_mut() -= 4.0;
            }
        }

        let text = self
            .icon
            .map_or(self.text.to_string(), |icon| icon.to_string());

        p.text(
            text_rect.center(),
            Align2::CENTER_CENTER,
            text,
            if self.icon.is_some() {
                style::font_icon()
            } else {
                style::font_button()
            },
            ui.visuals().widgets.inactive.fg_stroke.color,
        );

        // colored indicator
        if let Some(mut indicator_color) = self.indicator {
            if !resp.enabled() {
                indicator_color = indicator_color.gamma_multiply(ui.visuals().disabled_alpha);
            }

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

pub struct ToggleButton<'a> {
    val: &'a mut bool,
    button: Button,
}

impl<'a> ToggleButton<'a> {
    pub fn new(val: &'a mut bool, text: impl ToString, indicator_color: Color32) -> Self {
        Self {
            button: Button::new(text).indicator(val.then_some(indicator_color)),
            val,
        }
    }
}

impl<'a> Widget for ToggleButton<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let resp = self.button.ui(ui);
        if resp.clicked() {
            *self.val = !*self.val;
        }
        resp
    }
}
