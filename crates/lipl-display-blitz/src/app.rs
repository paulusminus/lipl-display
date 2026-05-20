use std::time::Duration;

use crate::{
    args::Args,
    constant::SS_ASSET,
    multi_line::MultiLine,
    status::Status,
    store::{Lipl, LiplStoreExt},
};
use dioxus::prelude::*;
use dioxus_native_blitz::use_window;
use futures_util::TryStreamExt;
use lipl_display_common::{Command, Message};
use tokio::time::sleep;
// #[cfg(feature = "fullscreen")]
// use winit::monitor::Fullscreen;

trait ToLines {
    fn to_lines(&self) -> Vec<String>;
}

impl ToLines for String {
    fn to_lines(&self) -> Vec<String> {
        self.lines().map(|s| s.trim().to_owned()).collect()
    }
}

pub fn app() -> Element {
    let args = use_context::<Args>();
    let store = use_store(|| Lipl::from(args));
    use_future(move || background_task(store));
    use_window().set_cursor_visible(false);
    // #[cfg(feature = "fullscreen")]
    // use_window().set_fullscreen(Some(Fullscreen::Borderless(None)));

    rsx! {
        document::Stylesheet {
            href: SS_ASSET,
        },
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" },
        body {
            class: if store.dark().cloned() { "dark" } else { "light" },
            MultiLine {
                content: store.part().cloned().to_lines(),
                font_size: store.font_size().cloned(),
            }
            Status {
                font_size: store.font_size().cloned(),
                text: store.status().cloned(),
            }
        }
    }
}

async fn background_task(store: Store<Lipl>) {
    let r = json_lines::file_reader("/home/paul/Code/dart/lipl_display/lipl-gatt-input.txt")
        .await
        .unwrap();
    let mut s = json_lines::lines::<Message, _>(r);

    while let Some(message) = s.try_next().await.unwrap() {
        match message {
            Message::Part(part) => store.part().set(part),
            Message::Status(status) => store.status().set(status),
            Message::Command(Command::Dark) => store.dark().set(true),
            Message::Command(Command::Light) => store.dark().set(false),
            Message::Command(Command::Increase) => {
                let font_size = store.font_size().cloned().saturating_add(1);
                store.font_size().set(font_size);
            }
            Message::Command(Command::Decrease) => {
                let font_size = store.font_size().cloned().saturating_sub(1);
                store.font_size().set(font_size);
            }
            Message::Command(Command::Exit) | Message::Command(Command::Poweroff) => {
                break;
            }
            Message::Command(Command::Wait) => {
                let wait_message = store.wait_message().cloned();
                store.status().set(wait_message);
                store.part().set(String::new());
            }
        }

        let timeout = store.timeout().cloned();
        sleep(Duration::from_millis(timeout)).await;
    }
}
