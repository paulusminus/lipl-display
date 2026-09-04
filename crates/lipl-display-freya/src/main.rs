#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod font_size;
mod part;
mod status;
mod theme;

#[cfg(feature = "fake")]
use constant::INPUT;
#[cfg(not(feature = "fake"))]
use constant::PATH;
use constant::{
    APPLICATION_HEIGHT, APPLICATION_TITLE, APPLICATION_WIDTH, FONT_SIZE_INCREMENT,
    MINIMUM_FONT_SIZE, WAIT_MESSAGE,
};
use font_size::FontSize;
use freya::prelude::*;
use futures_util::{FutureExt, StreamExt, TryStreamExt};
use lipl_display_common::{Command, HandleMessage, LiplScreen, Message};
use lipl_gatt_bluer::listen_stream;
use part::Part;
use status::Status;
use std::time::Duration;
use theme::Theme;
use tokio::{runtime::Runtime, time::sleep};

mod constant;
// mod file_input;

async fn background_task() -> Result<(), std::io::Error> {
    #[cfg(feature = "fake")]
    let f = INPUT;
    #[cfg(not(feature = "fake"))]
    let f = json_lines::file_reader(PATH).await?;
    json_lines::lines(f)
        .try_for_each(move |message| {
            match message {
                Message::Part(p) => {
                    consume_context::<Part>().set_text(p.into());
                }
                Message::Status(s) => consume_context::<Status>().set_text(s.into()),
                Message::Command(c) => match c {
                    Command::Dark => {
                        consume_context::<Theme>().set(Theme::dark());
                    }
                    Command::Light => {
                        consume_context::<Theme>().set(Theme::light());
                    }
                    Command::Increase => {
                        let mut font_size = consume_context::<FontSize>();
                        let f = font_size.value();
                        font_size.set(f + FONT_SIZE_INCREMENT);
                    }
                    Command::Decrease => {
                        let font_size = consume_context::<FontSize>().value();
                        if font_size > MINIMUM_FONT_SIZE {
                            consume_context::<FontSize>().set(font_size - FONT_SIZE_INCREMENT);
                        }
                    }
                    Command::Wait => {
                        consume_context::<Part>().set_text("".to_owned().into());
                        consume_context::<Status>().set_text(WAIT_MESSAGE.to_owned().into());
                    }
                    Command::Exit => {
                        // use_platform().close_window();
                    }
                    Command::Poweroff => {
                        // use_platform().close_window();
                    }
                },
            }
            sleep(Duration::from_secs(1)).map(|_| Ok(()))
        })
        .await
}

#[derive(PartialEq)]
struct Root;

impl Component for Root {
    fn render(&self) -> impl IntoElement {
        let theme = use_state(|| Theme::Dark);
        let font_size = use_state(|| FontSize::from(22));
        let status = use_state(|| Status::default());
        let part = use_state(|| Part::default());

        use_future(background_task);

        rect().children([
            rect()
                .width(Size::percent(100.0))
                .height(Size::percent(90.0))
                .background(Fill::Color(theme.read().bg_color()))
                .color(Fill::Color(theme.read().fg_color()))
                .font_size(freya::prelude::FontSize::from(font_size.read().value()))
                .padding(Gaps::new_all(20.0))
                .children([label().text(part.read().to_string()).into_element()])
                .into_element(),
            rect()
                .width(Size::percent(100.0))
                .height(Size::percent(10.0))
                .background(Fill::Color(theme.read().fg_color()))
                .color(Fill::Color(theme.read().bg_color()))
                .padding(Gaps::new_all(20.0))
                .children([label().text(status.read().to_string()).into_element()])
                .into_element(),
        ])
    }
}

#[derive(Default)]
struct DisplayApp;

impl App for DisplayApp {
    fn render(&self) -> impl IntoElement {
        Root {}
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();

    let window_config = WindowConfig::new_app(DisplayApp::default())
        .with_title(APPLICATION_TITLE)
        .with_size(APPLICATION_WIDTH, APPLICATION_HEIGHT);
    let launch_config = LaunchConfig::default()
        .with_window(window_config)
        .with_future(|proxy| async {
            let s = listen_stream()
                .await
                .unwrap()
                .scan(LiplScreen::default(), |last, message| {
                    last.handle_message(message);
                    async move { Some(last.clone()) }
                });
        });
    launch(launch_config);
}
