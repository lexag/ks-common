use egui::{
    Color32, Context, CornerRadius, CursorIcon, FontData, FontDefinitions, FontId, Margin, Shadow,
    Spacing, Stroke, Style, Visuals, hex_color,
    style::{
        Interaction, ScrollAnimation, ScrollStyle, Selection, TextCursorStyle, WidgetVisuals,
        Widgets, default_text_styles,
    },
};

const INTERACTIVE_STROKE_WIDTH: f32 = 2.0;

const GRAY_1: Color32 = hex_color!("#000000");
const GRAY_2: Color32 = hex_color!("#303030");

pub const ACCENT_COLOR: Color32 = hex_color!("#29acaf");

pub const WARNING_COLOR: Color32 = hex_color!("#f0ad4e");
pub const ERROR_COLOR: Color32 = hex_color!("#b41c2b");
pub const ACTIVE_COLOR: Color32 = hex_color!("#009f42");
pub const CUED_COLOR: Color32 = hex_color!("#132916");

pub fn load_fonts(egui_ctx: &mut Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "LTC".to_owned(),
        std::sync::Arc::new(
            // .ttf and .otf supported
            FontData::from_static(include_bytes!("../font/VT323-Regular.ttf")),
        ),
    );

    let ltc_family = egui::FontFamily::Name("LTC".into());

    fonts.families.insert(ltc_family, vec!["LTC".to_owned()]);

    egui_ctx.set_fonts(fonts);
}

pub fn style() -> Style {
    Style {
        spacing: spacing(),
        interaction: interaction(),
        visuals: visuals(),
        animation_time: 0.7,
        explanation_tooltips: true,
        url_in_tooltip: true,
        always_scroll_the_only_direction: true,
        scroll_animation: ScrollAnimation {
            points_per_second: 400.0,
            duration: (0.0..=4.0).into(),
        },
        compact_menu_style: true,
        ..Default::default()
    }
}

fn interaction() -> Interaction {
    Interaction {
        selectable_labels: false,
        multi_widget_text_select: false,
        ..Default::default()
    }
}

fn spacing() -> Spacing {
    Spacing {
        item_spacing: [8.0, 2.0].into(),
        window_margin: Margin::same(4),
        button_padding: [4.0, 2.0].into(),
        menu_margin: Margin::same(4),
        indent: 16.0,
        icon_width: 16.0,
        icon_width_inner: 10.0,
        icon_spacing: 4.0,
        indent_ends_with_horizontal_line: true,
        scroll: ScrollStyle::solid(),
        ..Default::default()
    }
}

fn visuals() -> Visuals {
    Visuals {
        selection: selection(),
        warn_fg_color: WARNING_COLOR,
        error_fg_color: ERROR_COLOR,
        window_shadow: shadow(),
        striped: true,
        ..Default::default()
    }
}

//fn visuals() -> Visuals {
//    Visuals {
//        dark_mode: true,
//        override_text_color: Some(GRAY_5),
//        weak_text_color: Some(GRAY_3),
//        widgets: widgets(),
//        selection: selection(),
//        hyperlink_color: PRIMARY_MAIN,
//        faint_bg_color: GRAY_2_FAINT,
//        extreme_bg_color: GRAY_1,
//        code_bg_color: GRAY_1,
//        window_corner_radius: CORNER_RADIUS,
//        window_fill: GRAY_2,
//        window_stroke: Stroke::new(NONINTERACTIVE_STROKE_WIDTH, GRAY_3),
//        window_highlight_topmost: false,
//        menu_corner_radius: CORNER_RADIUS,
//        panel_fill: GRAY_2,
//        popup_shadow: shadow(),
//        resize_corner_size: 0.0,
//        text_cursor: TextCursorStyle {
//            stroke: Stroke::new(INTERACTIVE_STROKE_WIDTH, GRAY_6),
//            preview: false,
//            blink: false,
//            on_duration: 0.0,
//            off_duration: 0.0,
//        },
//        clip_rect_margin: 2.0,
//        button_frame: true,
//        collapsing_header_frame: false,
//        indent_has_left_vline: false,
//        striped: true,
//        slider_trailing_fill: true,
//        handle_shape: egui::style::HandleShape::Rect { aspect_ratio: 0.5 },
//        interact_cursor: Some(CursorIcon::PointingHand),
//        image_loading_spinners: true,
//        numeric_color_space: egui::style::NumericColorSpace::GammaByte,
//        disabled_alpha: 0.5,
//        ..Default::default()
//    }
//}

fn shadow() -> Shadow {
    Shadow {
        offset: [2, 2],
        blur: 0,
        spread: 3,
        color: GRAY_1,
    }
}

fn selection() -> Selection {
    Selection {
        bg_fill: ACCENT_COLOR,
        stroke: Stroke::new(INTERACTIVE_STROKE_WIDTH, GRAY_2),
    }
}
