const LIGHT_THEME: &str = include_str!("light.css");
const DARK_THEME: &str = include_str!("dark.css");

pub enum Theme {
    Dark,
    Light,
}

pub fn load(theme: Theme) {
    let css = match theme {
        Theme::Dark => DARK_THEME,
        Theme::Light => LIGHT_THEME,
    };
    let provider: gtk4::CssProvider = gtk4::CssProvider::new();
    provider.load_from_data(css);

    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
