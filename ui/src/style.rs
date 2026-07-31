use egui::{
    Color32, Context, CornerRadius, CursorIcon, FontData, FontDefinitions, FontFamily, FontId,
    Margin, Response, Shadow, Spacing, Stroke, Style, Visuals, hex_color,
    style::{
        Interaction, ScrollAnimation, ScrollStyle, Selection, TextCursorStyle, WidgetVisuals,
        Widgets,
    },
    vec2,
};

pub const ACCENT_COLOR: Color32 = hex_color!("#29acaf");

pub const WARNING_COLOR: Color32 = hex_color!("#f0ad4e");
pub const ERROR_COLOR: Color32 = hex_color!("#b41c2b");
pub const ACTIVE_COLOR: Color32 = hex_color!("#009f42");
pub const CUED_COLOR: Color32 = hex_color!("#132916");

pub const CORNER_RADIUS: CornerRadius = CornerRadius::same(4);

pub fn load_fonts(egui_ctx: &mut Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "LTC".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(path!(
            "font/VT323-Regular.ttf"
        )))),
    );
    fonts.font_data.insert(
        "IBMPlexSans".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(path!(
            "font/IBMPlexSans-Regular.ttf"
        )))),
    );
    fonts.font_data.insert(
        "IBMPlexMono".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(path!(
            "font/IBMPlexMono-Regular.ttf"
        )))),
    );
    fonts.font_data.insert(
        "IBMPlexMonoBold".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(path!(
            "font/IBMPlexMono-Bold.ttf"
        )))),
    );
    fonts.font_data.insert(
        "MaterialIcons".to_owned(),
        std::sync::Arc::new(FontData::from_static(material_icons::FONT)),
    );

    fonts.families.insert(
        FontFamily::Name("PlexMono".into()),
        vec!["IBMPlexMono".to_owned(), "MaterialIcons".to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name("PlexMonoBold".into()),
        vec!["IBMPlexMonoBold".to_owned(), "MaterialIcons".to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name("Plex".into()),
        vec!["IBMPlexSans".to_owned(), "MaterialIcons".to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name("LTC".into()),
        vec!["LTC".to_owned(), "MaterialIcons".to_owned()],
    );

    egui_ctx.set_fonts(fonts);
}

pub fn font_icon() -> FontId {
    FontId::new(32.0, FontFamily::Name("PlexMono".into()))
}
pub fn font_menu() -> FontId {
    FontId::new(13.0, FontFamily::Name("Plex".into()))
}
pub fn font_label() -> FontId {
    FontId::new(14.0, FontFamily::Name("Plex".into()))
}
pub fn font_button() -> FontId {
    FontId::new(16.0, FontFamily::Name("PlexMono".into()))
}
pub fn font_numeral_thin() -> FontId {
    FontId::new(20.0, FontFamily::Name("PlexMono".into()))
}
pub fn font_numeral() -> FontId {
    FontId::new(20.0, FontFamily::Name("PlexMonoBold".into()))
}
pub fn font_critical() -> FontId {
    FontId::new(28.0, FontFamily::Name("Plex".into()))
}

pub fn style() -> Style {
    Style {
        spacing: spacing(),
        interaction: interaction(),
        visuals: visuals(),
        animation_time: 0.12,
        explanation_tooltips: true,
        url_in_tooltip: true,
        always_scroll_the_only_direction: true,
        scroll_animation: ScrollAnimation {
            points_per_second: 400.0,
            duration: (0.0..=0.8).into(),
        },
        compact_menu_style: true,
        ..Default::default()
    }
}

pub fn interaction() -> Interaction {
    Interaction {
        selectable_labels: false,
        multi_widget_text_select: false,
        ..Default::default()
    }
}

pub(crate) fn spacing() -> Spacing {
    Spacing {
        item_spacing: vec2(8.0, 6.0),

        window_margin: egui::Margin {
            left: 8,
            right: 8,
            top: 6,
            bottom: 6,
        },

        menu_margin: egui::Margin {
            left: 8,
            right: 8,
            top: 6,
            bottom: 6,
        },

        button_padding: vec2(10.0, 6.0),

        menu_spacing: 4.0,

        indent: 18.0,

        interact_size: vec2(40.0, 28.0),

        slider_width: 140.0,

        combo_width: 140.0,

        text_edit_width: 220.0,

        icon_width: 18.0,

        icon_spacing: 6.0,

        tooltip_width: 360.0,

        indent_ends_with_horizontal_line: true,

        scroll: ScrollStyle {
            bar_width: 10.0,
            handle_min_length: 32.0,
            bar_inner_margin: 3.0,
            bar_outer_margin: 0.0,
            floating: true,
            floating_allocated_width: 8.0,
            foreground_color: true,
            ..Default::default()
        },

        ..Default::default()
    }
}

pub fn visuals() -> Visuals {
    Visuals {
        window_fill: Color32::from_rgb(28, 29, 32).lerp_to_gamma(Color32::BLACK, 0.5),
        panel_fill: Color32::from_rgb(36, 38, 42).lerp_to_gamma(Color32::BLACK, 0.5),

        extreme_bg_color: Color32::from_rgb(18, 19, 22),
        faint_bg_color: Color32::from_rgb(46, 48, 52),
        code_bg_color: Color32::from_rgb(24, 26, 30),

        window_stroke: Stroke::new(1.0, Color32::from_rgb(82, 86, 92)),

        selection: Selection {
            bg_fill: Color32::from_rgb(0, 150, 216),
            stroke: Stroke::new(2.0, Color32::WHITE),
        },

        window_corner_radius: CornerRadius::same(2),
        menu_corner_radius: CornerRadius::same(2),

        dark_mode: true,

        override_text_color: None,

        widgets: widgets(),

        hyperlink_color: Color32::from_rgb(110, 170, 255),
        warn_fg_color: Color32::from_rgb(255, 190, 70),
        error_fg_color: Color32::from_rgb(235, 85, 85),

        window_shadow: Shadow {
            offset: [0, 8],
            blur: 24,
            spread: 0,
            color: Color32::from_black_alpha(160),
        },

        popup_shadow: Shadow {
            offset: [0, 6],
            blur: 18,
            spread: 0,
            color: Color32::from_black_alpha(140),
        },

        resize_corner_size: 12.0,
        text_cursor: egui::style::TextCursorStyle {
            stroke: Stroke::new(2.0, Color32::from_rgb(220, 223, 226)),
            ..TextCursorStyle::default()
        },
        clip_rect_margin: 3.0,
        button_frame: true,
        collapsing_header_frame: false,
        indent_has_left_vline: true,
        striped: false,
        slider_trailing_fill: true,
        handle_shape: egui::style::HandleShape::Circle,
        interact_cursor: None,
        image_loading_spinners: true,
        numeric_color_space: egui::style::NumericColorSpace::GammaByte,
        ..Visuals::dark()
    }
}

pub fn widgets() -> egui::style::Widgets {
    Widgets {
        noninteractive: egui::style::WidgetVisuals {
            weak_bg_fill: Color32::from_rgb(28, 30, 34),
            bg_fill: Color32::from_rgb(34, 36, 40),
            bg_stroke: Stroke::new(1.0, Color32::from_rgb(58, 61, 66)),
            fg_stroke: Stroke::new(1.0, Color32::from_rgb(170, 175, 181)),
            corner_radius: CornerRadius::same(4),
            expansion: 0.0,
        },

        inactive: egui::style::WidgetVisuals {
            weak_bg_fill: Color32::from_rgb(42, 45, 50),
            bg_fill: Color32::from_rgb(50, 53, 59),
            bg_stroke: Stroke::new(1.0, Color32::from_rgb(82, 86, 92)),
            fg_stroke: Stroke::new(1.2, Color32::from_rgb(220, 223, 226)),
            corner_radius: CornerRadius::same(4),
            expansion: 0.0,
        },

        hovered: egui::style::WidgetVisuals {
            weak_bg_fill: Color32::from_rgb(54, 57, 63),
            bg_fill: Color32::from_rgb(64, 68, 74),
            bg_stroke: Stroke::new(1.5, Color32::from_rgb(135, 140, 148)),
            fg_stroke: Stroke::new(1.5, Color32::from_rgb(245, 245, 245)),
            corner_radius: CornerRadius::same(4),
            expansion: 1.0,
        },

        active: egui::style::WidgetVisuals {
            weak_bg_fill: Color32::from_rgb(72, 75, 82),
            bg_fill: Color32::from_rgb(88, 92, 100),
            bg_stroke: Stroke::new(2.0, Color32::from_rgb(170, 175, 185)),
            fg_stroke: Stroke::new(2.0, Color32::WHITE),
            corner_radius: CornerRadius::same(4),
            expansion: 1.0,
        },

        open: egui::style::WidgetVisuals {
            weak_bg_fill: Color32::from_rgb(48, 51, 56),
            bg_fill: Color32::from_rgb(58, 61, 67),
            bg_stroke: Stroke::new(1.5, Color32::from_rgb(150, 155, 165)),
            fg_stroke: Stroke::new(1.3, Color32::from_rgb(245, 245, 245)),
            corner_radius: CornerRadius::same(4),
            expansion: 0.0,
        },
    }
}

pub fn auto_bg_stroke(resp: &Response) -> Stroke {
    let vis = visuals().widgets;

    if resp.is_pointer_button_down_on() {
        vis.active.bg_stroke
    } else if resp.hovered() {
        vis.hovered.bg_stroke
    } else if resp.enabled() {
        vis.inactive.bg_stroke
    } else {
        vis.noninteractive.bg_stroke
    }
}

pub fn auto_bg_fill(resp: &Response) -> Color32 {
    let vis = visuals().widgets;

    if resp.is_pointer_button_down_on() {
        vis.active.bg_fill
    } else if resp.hovered() {
        vis.hovered.bg_fill
    } else if resp.enabled() {
        vis.inactive.bg_fill
    } else {
        vis.noninteractive.bg_fill
    }
}

pub fn auto_fg_stroke(resp: &Response) -> Stroke {
    let vis = visuals().widgets;

    if resp.is_pointer_button_down_on() {
        vis.active.fg_stroke
    } else if resp.hovered() {
        vis.hovered.fg_stroke
    } else if resp.enabled() {
        vis.inactive.fg_stroke
    } else {
        vis.noninteractive.fg_stroke
    }
}

pub fn auto_fg_fill(resp: &Response) -> Color32 {
    let vis = visuals().widgets;

    if resp.is_pointer_button_down_on() {
        vis.active.weak_bg_fill
    } else if resp.hovered() {
        vis.hovered.weak_bg_fill
    } else if resp.enabled() {
        vis.inactive.weak_bg_fill
    } else {
        vis.noninteractive.weak_bg_fill
    }
}
