use std::num::NonZeroU32;

use femtovg::{renderer::OpenGl, Canvas};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{Surface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub fn create_window<T: 'static>(
    title: &'static str,
    event_loop: &EventLoop<T>,
) -> (
    Canvas<OpenGl>,
    Window,
    PossiblyCurrentContext,
    Surface<WindowSurface>,
) {
    let window_builder = WindowBuilder::new()
        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
        .with_title(title);

    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(event_loop, template, |configs| {
            // Find the config with the maximum number of samples, so our triangle will
            // be smooth.
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() < accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    let window = window.unwrap();
    window.set_cursor_visible(false);

    let raw_window_handle = window.raw_window_handle();
    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(raw_window_handle));
    let mut not_current_gl_context = Some(unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(&gl_config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
    });

    let (width, height): (u32, u32) = window.inner_size().into();
    let raw_window_handle = window.raw_window_handle();
    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
    );

    let surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    let gl_context = not_current_gl_context
        .take()
        .unwrap()
        .make_current(&surface)
        .unwrap();

    let renderer =
        unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
            .expect("Cannot create renderer");

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    canvas.set_size(width, height, window.scale_factor() as f32);

    (canvas, window, gl_context, surface)
}
