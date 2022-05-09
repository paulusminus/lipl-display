use eframe::egui::{Context, TextStyle};

pub const FONT_SMALL_FACTOR: f32 = 0.7;

pub fn set_font_size(ctx: &Context, font_size: f32) {
    let mut style = (*ctx.style()).clone(); 
    style.text_styles.insert(TextStyle::Body, eframe::epaint::FontId { size: font_size, family: Default::default() });
    style.text_styles.insert(TextStyle::Small, eframe::epaint::FontId { size: font_size * FONT_SMALL_FACTOR, family: Default::default() });
    ctx.set_style(style);
}