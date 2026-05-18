use dioxus_native_blitz::WindowAttributes;
#[cfg(feature = "fullscreen")]
use winit::monitor::Fullscreen;

mod app;
mod store;

const APP_TITLE: &str = "Lipl Display";

// fn main() {
//     tracing_subscriber::fmt::init();
//     launch(app::app);
// }

#[cfg(not(feature = "fullscreen"))]
fn default_window_attributes() -> Box<WindowAttributes> {
    Box::new(
        WindowAttributes::default()
            .with_maximized(true)
            .with_title(APP_TITLE),
    )
}

#[cfg(feature = "fullscreen")]
fn default_window_attributes() -> Box<WindowAttributes> {
    Box::new(
        WindowAttributes::default()
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            .with_maximized(true),
    )
}

fn main() {
    tracing_subscriber::fmt::init();
    dioxus_native_blitz::launch_cfg(app::app, vec![], vec![default_window_attributes()]);
}
