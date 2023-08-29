use gtk::prelude::*;

const LIGHT_THEME: &[u8] = include_bytes!("light.css");
const DARK_THEME : &[u8] = include_bytes!("dark.css");

pub enum Theme {
    Dark,
    Light,
}

pub fn load(theme: Theme) {
    let css = match theme {
        Theme::Dark => DARK_THEME,
        Theme::Light => LIGHT_THEME,
    };
    let css_provider: gtk::CssProvider = gtk::CssProvider::new();
    match css_provider.load_from_data(css) {
        Err(e) => { eprintln!("{}", e); },
        Ok(_) => {
            if let Some(screen) = gtk::gdk::Screen::default() {
                gtk::StyleContext::add_provider_for_screen(
                    &screen,
                    &css_provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            };
        }
    }
}
