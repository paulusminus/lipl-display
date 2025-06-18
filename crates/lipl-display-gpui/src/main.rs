use constant::{APP_ID, FONT, WINDOW_HEIGHT, WINDOW_WIDTH};
use gpui::{
    App, Application, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div, px, size,
};

use lipl_display_common::Message;
use lipl_screen::LiplScreen;
use ui::{IntoElement, ParentElement, Styled};

mod constant;
mod lipl_screen;
mod listen_bluer;

impl Render for LiplScreen {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(window.bounds().bottom())
            .w(window.bounds().right())
            .bg(self.background_color())
            .text_color(self.foreground_color())
            .text_size(self.font_size())
            .font_family(FONT)
            .cursor_none(gpui::CursorStyle::None)
            .children([
                div()
                    .h(0.9 * window.bounds().bottom())
                    .child(self.text())
                    .p(self.font_size()),
                div()
                    .h(0.1 * window.bounds().bottom())
                    .child(self.status())
                    .p(self.font_size())
                    .text_size(self.font_size_status()),
            ])
    }
}

fn window_bounds(cx: &mut App) -> Option<WindowBounds> {
    Some(WindowBounds::Fullscreen(Bounds::centered(
        None,
        size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)),
        cx,
    )))
}

fn main() {
    env_logger::init();
    Application::new().run(|cx: &mut App| {
        gpui_tokio::init(cx);
        let (sender, receiver) = async_channel::unbounded::<Message>();
        listen_bluer::init(cx, sender);
        let window_bounds = window_bounds(cx);
        cx.open_window(
            WindowOptions {
                window_bounds,
                app_id: Some(APP_ID.into()),
                ..Default::default()
            },
            |_, cx| lipl_screen::init(cx, receiver),
        )
        .inspect_err(|e| log::error!("Error: {e}"))
        .unwrap();

        cx.activate(true);
    });
}
