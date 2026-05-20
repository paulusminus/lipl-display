use clap::Parser;
use dioxus_native_blitz::WindowAttributes;
#[cfg(feature = "fullscreen")]
use winit::monitor::Fullscreen;

use crate::args::Args;
#[cfg(feature = "fullscreen")]
use crate::constant::APP_TITLE;

mod app;
mod args;
mod constant;
mod multi_line;
mod status;
mod store;

#[cfg(not(feature = "fullscreen"))]
fn default_window_attributes() -> Box<WindowAttributes> {
    use crate::constant::APP_TITLE;

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
            .with_title(APP_TITLE)
            .with_maximized(true),
    )
}

fn main() {
    tracing_subscriber::fmt::init();
    dioxus_native_blitz::launch_cfg(
        app::app,
        vec![Box::new(|| Box::new(Args::parse()))],
        vec![default_window_attributes()],
    );
}
