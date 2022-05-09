use eframe::egui::{Visuals, style::{Widgets, WidgetVisuals}, Color32, Stroke, Context};

trait ColorExt {
    fn foreground(&self) -> Color32;
    fn background(&self) -> Color32;
}

impl ColorExt for bool {
    fn foreground(&self) -> Color32 {
        if *self {
            Color32::WHITE
        }  
        else {
            Color32::BLACK
        }
    }
    fn background(&self) -> Color32 {
        if *self {
            Color32::BLACK
         }
         else {
            Color32::WHITE
         }
    }
}

fn widget_visuals(dark: bool) -> WidgetVisuals {
    WidgetVisuals {
        bg_fill: dark.background(),
        bg_stroke: Stroke { width: 0., color: dark.background() },
        fg_stroke: Stroke { width: 0., color: dark.foreground() },
        expansion: 1.0,
        rounding: Default::default(),
    }
}

fn widgets(dark: bool) -> Widgets {
    Widgets {
        noninteractive: widget_visuals(dark),
        inactive: widget_visuals(dark),
        hovered: widget_visuals(dark),
        active: widget_visuals(dark),
        open: widget_visuals(dark),
    }
}

fn visuals(dark: bool) -> Visuals {
    Visuals {
        dark_mode: dark,
        widgets: widgets(dark),
        ..Default::default()
    }
}

pub fn set_dark_mode(ctx: &Context, dark: bool) {
    ctx.set_visuals(visuals(dark));
}