use std::time::Duration;

use dioxus::prelude::*;
use futures_util::TryStreamExt;
use lipl_display_common::{Command, Message};
use tokio::time::sleep;

const DEFAULT_STATUS: &str = "Even geduld a.u.b. ...";
const DEFAULT_DARK: bool = false;
const DEFAULT_FONT_SIZE: u32 = 30;
const STYLESHEET: &str = include_str!("styles.css");

pub fn app() -> Element {
    let part = use_signal(|| Vec::<String>::new());
    let status = use_signal(|| DEFAULT_STATUS.to_owned());
    let dark = use_signal(|| DEFAULT_DARK);
    let font_size = use_signal(|| DEFAULT_FONT_SIZE);

    use_future(move || background_task(part, status, dark, font_size));

    rsx!(
        head {
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            title { "Dioxus App" }
            style { r#type: "text/css", {STYLESHEET} }
        }
        body {
            class: if dark() { "dark" } else { "light" },
            ul {
                class: "part",
                style: format!("font-size: {}px;", font_size()),
                   {part.iter().map(|i| rsx! { li { "{i}" } })}
               }
            p {
                class: "status",
                style: format!("font-size: {}px;", font_size().saturating_sub(2)),
                {status} }
    })
}

async fn background_task(
    mut part_signal: Signal<Vec<String>>,
    mut status_signal: Signal<String>,
    mut dark_signal: Signal<bool>,
    mut font_size_signal: Signal<u32>,
) {
    let r = json_lines::file_reader("/home/paul/Code/dart/lipl_display/lipl-gatt-input.txt")
        .await
        .unwrap();
    let mut s = json_lines::lines::<Message, _>(r);

    while let Some(message) = s.try_next().await.unwrap() {
        match message {
            Message::Part(part) => part_signal.set(
                part.split("\n")
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            ),
            Message::Status(status) => status_signal.set(status),
            Message::Command(Command::Dark) => dark_signal.set(true),
            Message::Command(Command::Light) => dark_signal.set(false),
            Message::Command(Command::Increase) => font_size_signal.set(font_size_signal() + 1),
            Message::Command(Command::Decrease) => {
                font_size_signal.set(font_size_signal().saturating_sub(1))
            }
            Message::Command(Command::Exit) | Message::Command(Command::Poweroff) => {
                break;
            }
            Message::Command(Command::Wait) => {
                status_signal.set("Even geduld a.u.b. ...".to_owned());
                part_signal.set(Vec::<String>::new());
            }
        }
        sleep(Duration::from_secs(2)).await;
    }
}
