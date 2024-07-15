use egui::*;

fn color_from_cat(c: catppuccin::Color) -> eframe::egui::Color32 {
    eframe::egui::Color32::from_rgb(c.rgb.r, c.rgb.g, c.rgb.b)
}

/// Code modified from https://github.com/catppuccin/egui
/// Currently the library is broken because of an egui update.
fn make_widget_visual(
    cat: catppuccin::Flavor,
    old: style::WidgetVisuals,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: color_from_cat(cat.colors.overlay1),
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: color_from_cat(cat.colors.text),
            ..old.fg_stroke
        },
        ..old
    }
}

/// Code modified from https://github.com/catppuccin/egui
/// Currently the library is broken because of an egui update.
pub fn visuals(cat: catppuccin::Flavor, old: egui::Visuals) -> egui::Visuals {
    egui::Visuals {
        override_text_color: Some(color_from_cat(cat.colors.text)),
        hyperlink_color: color_from_cat(cat.colors.rosewater),
        faint_bg_color: color_from_cat(cat.colors.surface0),
        extreme_bg_color: color_from_cat(cat.colors.crust),
        code_bg_color: color_from_cat(cat.colors.mantle),
        warn_fg_color: color_from_cat(cat.colors.peach),
        error_fg_color: color_from_cat(cat.colors.maroon),
        window_fill: color_from_cat(cat.colors.base),
        panel_fill: color_from_cat(cat.colors.base),
        window_stroke: egui::Stroke {
            color: color_from_cat(cat.colors.overlay1),
            ..old.window_stroke
        },
        widgets: style::Widgets {
            noninteractive: make_widget_visual(
                cat,
                old.widgets.noninteractive,
                color_from_cat(cat.colors.base),
            ),
            inactive: make_widget_visual(
                cat,
                old.widgets.inactive,
                color_from_cat(cat.colors.surface0),
            ),
            hovered: make_widget_visual(
                cat,
                old.widgets.hovered,
                color_from_cat(cat.colors.surface2),
            ),
            active: make_widget_visual(
                cat,
                old.widgets.active,
                color_from_cat(cat.colors.surface1),
            ),
            open: make_widget_visual(cat, old.widgets.open, color_from_cat(cat.colors.surface0)),
        },
        selection: style::Selection {
            bg_fill: color_from_cat(cat.colors.blue).linear_multiply(if cat.dark {
                0.2
            } else {
                0.4
            }),
            stroke: egui::Stroke {
                color: color_from_cat(cat.colors.overlay1),
                ..old.selection.stroke
            },
        },
        window_shadow: epaint::Shadow {
            color: color_from_cat(cat.colors.base),
            ..old.window_shadow
        },
        popup_shadow: epaint::Shadow {
            color: color_from_cat(cat.colors.base),
            ..old.popup_shadow
        },

        dark_mode: cat.dark,
        ..old
    }
}

pub fn get_color_palette(cat: catppuccin::Flavor) -> Vec<eframe::egui::Color32> {
    vec![
        color_from_cat(cat.colors.red),
        color_from_cat(cat.colors.peach),
        color_from_cat(cat.colors.yellow),
        color_from_cat(cat.colors.green),
        color_from_cat(cat.colors.teal),
        color_from_cat(cat.colors.blue),
        color_from_cat(cat.colors.mauve),
        color_from_cat(cat.colors.pink),
        color_from_cat(cat.colors.text),
        color_from_cat(cat.colors.base),
    ]
}
