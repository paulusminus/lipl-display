#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(feature = "fake")]
use std::time::Duration;

use constant::{APPLICATION_HEIGHT, APPLICATION_TITLE, APPLICATION_WIDTH};
use freya::{prelude::*, sdk::use_track_watcher};
#[cfg(feature = "fake")]
use futures_util::TryStream;
use futures_util::TryStreamExt;
use lipl_display_common::{HandleMessage, LiplScreen, Message};
use tokio::runtime::Runtime;

mod constant;

struct DisplayApp {
    rx: tokio::sync::watch::Receiver<LiplScreen>,
}

trait LiplScreenExt {
    fn bg_color(&self) -> Color;
    fn fg_color(&self) -> Color;
}

impl LiplScreenExt for LiplScreen {
    fn bg_color(&self) -> Color {
        if self.dark {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }

    fn fg_color(&self) -> Color {
        if self.dark {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}

impl App for DisplayApp {
    fn render(&self) -> impl IntoElement {
        use_track_watcher(&self.rx);

        rect().children([
            rect()
                .width(Size::percent(100.0))
                .height(Size::percent(85.0))
                .background(Fill::Color(self.rx.borrow().bg_color()))
                .color(Fill::Color(self.rx.borrow().fg_color()))
                .font_size(FontSize::from(self.rx.borrow().font_size))
                .padding(Gaps::new_all(20.0))
                .children([label().text(self.rx.borrow().text.clone()).into_element()])
                .into_element(),
            rect()
                .width(Size::percent(100.0))
                .height(Size::percent(15.0))
                .background(Fill::Color(self.rx.borrow().bg_color()))
                .color(Fill::Color(self.rx.borrow().fg_color()))
                .font_size(FontSize::from(self.rx.borrow().font_size))
                .padding(Gaps::new_all(20.0))
                .children([label().text(self.rx.borrow().status.clone()).into_element()])
                .into_element(),
        ])
    }
}

#[cfg(feature = "fake")]
async fn listen() -> impl TryStream<Ok = Message, Error = std::io::Error> {
    let lines = json_lines::file_reader(constant::PATH).await.unwrap();
    json_lines::lines(lines)
}

#[cfg(not(feature = "fake"))]
async fn listen() -> impl TryStream<Ok = Message, Error = std::io::Error> {
    let s = lipl_gatt_bluer::listen_stream().await.unwrap();
    s
}

fn main() {
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();

    let (tx, rx) = tokio::sync::watch::channel(LiplScreen::default());

    let window_config = WindowConfig::new_app(DisplayApp { rx })
        .with_title(APPLICATION_TITLE)
        .with_size(APPLICATION_WIDTH, APPLICATION_HEIGHT)
        .with_window_attributes(|attributes, _| {
            attributes.with_fullscreen(Some(freya::winit::window::Fullscreen::Borderless(None)))
        })
        .with_window_handle(|window| {
            window.set_cursor_visible(false);
        });
    let launch_config = LaunchConfig::default()
        .with_window(window_config)
        .with_future(|_proxy: LaunchProxy| async move {
            let mut s = listen().await;
            let mut screen = LiplScreen::default();
            screen.font_size = constant::FONT_SIZE;
            screen.status = constant::WAIT_MESSAGE.to_string();

            while let Ok(Some(message)) = s.try_next().await {
                if message.is_stop() {
                    let result = _proxy.post_callback(|render_context| {
                        render_context.exit();
                    });
                    result.await.unwrap();
                    break;
                }
                screen.handle_message(message);
                tx.send(screen.clone()).unwrap();
                #[cfg(feature = "fake")]
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        });
    launch(launch_config);
}
