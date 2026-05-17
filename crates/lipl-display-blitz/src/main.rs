use dioxus_native_blitz::launch;

mod app;

fn main() {
    tracing_subscriber::fmt::init();
    launch(app::app);
}
