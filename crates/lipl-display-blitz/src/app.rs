use chrono::Local;
use dioxus::prelude::*;
use std::time::Duration;
use tokio::time::interval;

const DEFAULT_PART: &str = "";
const DEFAULT_STATUS: &str = "Even geduld a.u.b. ...";
const DEFAULT_DARK: bool = false;
const STYLESHEET: &str = include_str!("styles.css");

pub fn app() -> Element {
    let part = use_signal(|| DEFAULT_PART.to_owned());
    let status = use_signal(|| DEFAULT_STATUS.to_owned());
    let dark = use_signal(|| DEFAULT_DARK);

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
