use dioxus_native::launch;

mod app;

fn main() {
    tracing_subscriber::fmt::init();
    launch(app::app);
}
