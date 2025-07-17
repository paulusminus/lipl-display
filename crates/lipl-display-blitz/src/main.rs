use std::time::Duration;

use chrono::Local;
use dioxus::prelude::*;
use dioxus_native::launch;
use tokio::time::interval;

const STYLESHEET: &str = include_str!("styles.css");

fn main() {
    tracing_subscriber::fmt::init();
    launch(app);
}

async fn background_task(
    mut part: Signal<String>,
    mut status: Signal<String>,
    _dark: Signal<bool>,
) {
    let mut count: usize = 0;
    let mut interval = interval(Duration::from_millis(1000));
    loop {
        interval.tick().await;
        count += 1;
        let time = Local::now();
        let fmt = "%H:%M:%S";
        part.set(time.format(fmt).to_string());
        status.set(format!("Teller = {count}"));
    }
}

fn app() -> Element {
    let part = use_signal(|| "".to_owned());
    let status = use_signal(|| "Even geduld a.u.b. ...".to_owned());
    let dark = use_signal(|| false);

    use_future(move || background_task(part, status, dark));

    rsx!(
        head {
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            title { "Dioxus App" }
            style { r#type: "text/css", {STYLESHEET} }
        }
        body {
            class: if dark() { "dark" } else { "light" },
        div {
            p {
                class: "part",
                {part} }
            p {
                class: "status",
                {status} }
        }
    })
}
