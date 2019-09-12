#[macro_use]
extern crate log;

mod state;
use state::State;

use cgmath::vec2;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

use docopt::Docopt;
use hotwatch::Hotwatch;
use serde::Deserialize;

use imgui::{FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_opengl_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use std::sync::{Arc, Mutex};

use tinygl_renderer::demo::RenderMode;

const USAGE: &'static str = concat!(
    env!("CARGO_PKG_NAME"),
    " v",
    env!("CARGO_PKG_VERSION"),
    "

Usage:
  tinygl <demo>
  tinygl (-h | --help)
  tinygl --version

Options:
  -h --help  Show this screen.
  --version  Show version.
"
);

#[derive(Debug, Deserialize)]
struct Args {
    arg_demo: String,
}

enum UserEvent {
    ReloadDemo,
}

fn run_ui(ui: &mut Ui) {
    // TODO: render UI
}

fn main() {
    env_logger::from_env(env_logger::Env::new().filter_or("TINYGL_LOG", "debug"))
        .default_format_timestamp(false)
        .init();

    let args: Args = Docopt::new(USAGE)
        .map(|d| {
            d.version(Some(format!(
                "{} v{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            )))
        })
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let (gl, event_loop, windowed_context, shader_version, render_size) = {
        let render_size = vec2(1024, 768);
        let el = glutin::event_loop::EventLoop::<UserEvent>::with_user_event();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("tinygl")
            .with_inner_size(glutin::dpi::LogicalSize::new(
                render_size.x.into(),
                render_size.y.into(),
            ));
        let windowed_context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        let context = glow::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });
        (context, el, windowed_context, "#version 410", render_size)
    };

    // Create Imgui context
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    // Create Imgui platform
    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        windowed_context.window(),
        HiDpiMode::Rounded,
    );

    // Setup fonts
    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../resources/mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    // Imgui renderer
    let renderer = Renderer::new(&mut imgui, |s| windowed_context.get_proc_address(s) as _);

    // Create default context
    let state = Arc::new(Mutex::new(State::new(gl, render_size, shader_version)));

    let path = std::fs::canonicalize(&args.arg_demo)
        .expect(&format!("failed to canonicalize {}", args.arg_demo));

    // Setup watch
    let mut hotwatch = Hotwatch::new().expect("failed to initialize hotwatch");
    let proxy = event_loop.create_proxy();

    let path_clone = path.clone();
    hotwatch
        .watch(
            path.parent().unwrap(),
            move |event: hotwatch::Event| match event {
                hotwatch::Event::Write(ref p) | hotwatch::Event::Create(ref p)
                    if p == &path_clone =>
                {
                    proxy
                        .send_event(UserEvent::ReloadDemo)
                        .expect("failed to send ReloadDemo event");
                }
                _ => {}
            },
        )
        .expect(&format!("failed to watch {}", path.display()));

    // Initialize state
    {
        let mut state = state.lock().unwrap();

        // Compile demo
        state.load_file(&path).expect("failed to load demo");

        // Bind VAO for screen quad
        state.bind_vao();
    }

    // Draw once when starting
    windowed_context.window().request_redraw();

    let mut last_frame = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Handle event using Imgui
        platform.handle_event(imgui.io_mut(), windowed_context.window(), &event);

        match event {
            Event::LoopDestroyed => {
                info!("Event::LoopDestroyed!");
                state.lock().unwrap().drop_demo();
                return;
            }
            Event::UserEvent(UserEvent::ReloadDemo) => {
                match state.lock().unwrap().load_file(&path) {
                    Ok(()) => {
                        info!("reloaded {}", path.display());
                        windowed_context.window().request_redraw();
                    }
                    Err(error) => info!("cannot reload {}: {}", path.display(), error.to_string()),
                }
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    info!("WindowEvent::Resized: {:?}", logical_size);

                    let dpi_factor = windowed_context.window().hidpi_factor();
                    let size = logical_size.to_physical(dpi_factor);
                    windowed_context.resize(size);

                    // Redraw on size change
                    windowed_context.window().request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    let window = windowed_context.window();
                    let dpi_factor = window.hidpi_factor();
                    let size = window.inner_size().to_physical(dpi_factor);
                    let mut state = state.lock().unwrap();

                    // Check that we are rendering at the right size
                    state
                        .resize(cgmath::vec2(size.width as u32, size.height as u32))
                        .expect("failed to resize resources");

                    let io = imgui.io_mut();
                    platform
                        .prepare_frame(io, window)
                        .expect("failed to start imgui frame");
                    last_frame = io.update_delta_time(last_frame);
                    let mut ui = imgui.frame();
                    run_ui(&mut ui);

                    // Render all passes to the default framebuffer
                    state.render(RenderMode::Full { target: None });

                    // Render Imgui
                    platform.prepare_render(&ui, window);
                    renderer.render(ui);

                    // Swap result
                    windowed_context.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => {
                    info!("WindowEvent::CloseRequested");
                    *control_flow = ControlFlow::Exit
                }
                _ => {
                    // For Imgui
                    windowed_context.window().request_redraw();
                }
            },
            _ => (),
        }
    });
}
