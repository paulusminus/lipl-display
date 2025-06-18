use futures_util::TryStreamExt;
use lipl_display_common::{Command, HandleMessage, LiplScreen, Message};
use masonry::widgets::GridParams;
use masonry_winit::widgets::{CrossAxisAlignment, MainAxisAlignment};
use std::time::Duration;
use winit::error::EventLoopError;
use winit::window::{Fullscreen, Window};
use xilem::core::{MessageProxy, fork};
use xilem::view::{Axis, GridExt, flex, grid, label, sized_box, task};
use xilem::{Color, EventLoop, WidgetView, Xilem, tokio};

const APP_TITLE: &str = "Elm";

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

fn display(screen: &mut LiplScreen) -> impl WidgetView<LiplScreen> + use<> {
    sized_box(grid(
        (
            flex(
                label(screen.text.clone())
                    .text_size(screen.font_size)
                    .font("Roboto")
                    .brush(screen.fg_color()),
            )
            .direction(Axis::Horizontal)
            .main_axis_alignment(MainAxisAlignment::Center)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .grid_item(GridParams::new(0, 0, 1, 11)),
            flex(
                label(screen.status.clone())
                    .text_size(screen.font_size)
                    .brush(screen.fg_color()),
            )
            .direction(Axis::Horizontal)
            .main_axis_alignment(MainAxisAlignment::Center)
            .grid_pos(0, 11),
        ),
        1,
        12,
    ))
    .background(screen.bg_color())
}

async fn background_task(proxy: MessageProxy<Message>) {
    let result = async {
        tracing::info!("Starting task");
        let r = json_lines::file_reader("/home/paul/Code/dart/lipl_display/lipl-gatt-input.txt")
            .await?;
        tracing::info!("input from file ok");
        let mut s = json_lines::lines::<Message>(r);
        while let Some(message) = s.try_next().await? {
            tracing::info!("Received message: {}", message);
            proxy.message(message)?;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    }
    .await;
    match result {
        Ok(()) => {
            tracing::info!("Task completed successfully");
        }
        Err(error) => {
            tracing::error!("Task failed: {}", error);
        }
    }
}

fn on_message_received(screen: &mut LiplScreen, message: Message) {
    let handled = {
        if let Message::Command(command) = &message {
            if *command == Command::Wait {
                screen.text = "".into();
                screen.status = "Even geduld a.u.b. ...".into();
                true
            } else {
                false
            }
        } else {
            false
        }
    };
    if !handled {
        screen.handle_message(message);
    }
}

fn app_logic(screen: &mut LiplScreen) -> impl WidgetView<LiplScreen> + use<> {
    fork(display(screen), task(background_task, on_message_received))
}

fn main() -> Result<(), EventLoopError> {
    tracing_subscriber::fmt::init();
    let app = Xilem::new(
        LiplScreen {
            text: "".into(),
            status: "Even geduld a.u.b. ...".into(),
            font_size: 22.0,
            dark: false,
        },
        app_logic,
    )
    .with_font(include_bytes!("/usr/share/fonts/google-roboto/Roboto-Regular.ttf").to_vec());
    let attributes = Window::default_attributes()
        .with_title(APP_TITLE)
        .with_fullscreen(Some(Fullscreen::Borderless(None)));
    app.run_windowed_in(EventLoop::with_user_event(), attributes)?;
    Ok(())
}
