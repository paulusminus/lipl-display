use std::error::Error;

use femtovg::{renderer::OpenGl, Canvas, Color, FontId, Paint};
use glutin::surface::GlSurface;
use lipl_display_common::{BackgroundThread, Command, HandleMessage, LiplScreen, Message};
use lipl_gatt_bluer::ListenBluer;
use log::error;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{EventLoop, EventLoopProxy},
    window::Window,
};

const ROBOTO_REGULAR: &[u8] = include_bytes!("../assets/Roboto-Regular.ttf");
const DEFAULT_FONT_SIZE: f32 = 32.0;
const BLACK: femtovg::Color = femtovg::Color::black();
const WHITE: femtovg::Color = femtovg::Color::white();

#[allow(dead_code)]
mod gatt_client;
mod helpers;

mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    pub use Gles2 as Gl;
}

fn get_colors(dark: bool) -> (Color, Color) {
    if dark {
        (WHITE, BLACK)
    } else {
        (BLACK, WHITE)
    }
}

fn create_callback(proxy: EventLoopProxy<Message>) -> impl Fn(Message) {
    move |message| {
        if let Err(error) = proxy.send_event(message) {
            error!("Error sending to main loop: {}", error);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    let event_loop = EventLoop::<Message>::with_user_event().build()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let mut gatt = ListenBluer::new(create_callback(event_loop.create_proxy()));

    event_loop.run_app(&mut Application::default())?;

    gatt.stop();
    Ok(())
}

fn draw_paragraph(
    canvas: &mut Canvas<OpenGl>,
    font_id: FontId,
    part: &LiplScreen,
    window: &Window,
) {
    let dpi_factor = window.scale_factor();
    let size = window.inner_size();
    canvas.set_size(size.width, size.height, dpi_factor as f32);
    let x = 0.05 * canvas.width() as f32;
    let mut y = 0.05 * canvas.height() as f32 + part.font_size;

    let (fg_color, bg_color) = get_colors(part.dark);
    canvas.clear_rect(0, 0, size.width, size.height, bg_color);
    let mut paint = Paint::color(fg_color);
    paint.set_font(&[font_id]);
    paint.set_font_size(part.font_size);

    let font_metrics = canvas.measure_font(&paint).expect("Error measuring font");

    let width = canvas.width();

    let lines = canvas
        .break_text_vec(width as f32, &part.text, &paint)
        .expect("Error while breaking text");

    for line_range in lines {
        if let Ok(_res) = canvas.fill_text(x, y, &part.text[line_range], &paint) {
            y += font_metrics.height();
        }
    }

    y = canvas.height() as f32 - font_metrics.height();
    match canvas.fill_text(x, y, &part.status, &paint) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

struct ApplicationGraphics {
    canvas: Canvas<OpenGl>,
    context: glutin::context::PossiblyCurrentContext,
    font_id: FontId,
    surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    window: winit::window::Window,
}

struct Application {
    screen: LiplScreen,
    graphics: Option<ApplicationGraphics>,
}

impl Default for Application {
    fn default() -> Self {
        let mut screen = LiplScreen::new(true, DEFAULT_FONT_SIZE);
        screen.handle_message(Message::Part("Even geduld a.u.b. ..".to_owned()));
        Self {
            screen,
            graphics: None,
        }
    }
}

impl Application {
    fn initialized(&self) -> bool {
        self.graphics.is_some()
    }
}

impl ApplicationHandler<Message> for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        log::info!("resumed");
        let (mut canvas, window, context, surface) =
            helpers::create_window("Lipl Display", event_loop);
        let font_id = canvas.add_font_mem(ROBOTO_REGULAR).ok().unwrap();

        self.screen = LiplScreen::default();
        self.graphics = Some(ApplicationGraphics {
            canvas,
            context,
            font_id,
            surface,
            window,
        });

        let graphics = self.graphics.as_mut().unwrap();
        draw_paragraph(
            &mut graphics.canvas,
            graphics.font_id,
            &self.screen,
            &graphics.window,
        );
        graphics.surface
            .swap_buffers(&graphics.context)
            .unwrap_or_else(|error| {
                panic!("Cannot swap buffers: {error}");
            });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer: _,
            } => {
                log::info!("window_event scalefactorchanged {}", scale_factor);
            }
            WindowEvent::Resized(physical_size) => {
                log::info!("window_event: resized");
                if self.initialized() {
                    let graphics = self.graphics.as_mut().unwrap();
                    graphics.surface.resize(
                        &graphics.context,
                        physical_size.width.try_into().unwrap(),
                        physical_size.height.try_into().unwrap(),
                    );
                }
            }
            WindowEvent::CloseRequested => {
                log::info!("window_event close");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                log::info!("window_event redraw");
                if self.initialized() {
                    let graphics = self.graphics.as_mut().unwrap();
                    draw_paragraph(
                        &mut graphics.canvas,
                        graphics.font_id,
                        &self.screen,
                        &graphics.window,
                    );
                    graphics.surface
                        .swap_buffers(&graphics.context)
                        .unwrap_or_else(|error| {
                            panic!("Cannot swap buffers: {error}");
                        });
                }
            }
            _ => {
                log::info!("window_event {:#?}", event);
            }
        }
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        log::info!("new_events {:#?}", cause);
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: Message) {
        log::info!("user_event: {}", event);
        if [
            Message::Command(Command::Exit),
            Message::Command(Command::Poweroff),
        ]
        .contains(&event)
        {
            event_loop.exit();
        } else if self.initialized() {
            self.screen.handle_message(event);
            self.graphics.as_ref().unwrap().window.request_redraw();
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        _event: winit::event::DeviceEvent,
    ) {
        log::info!("device_event {:?}", device_id);
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.graphics = None;
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.exit();
    }

    fn memory_warning(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}
}
