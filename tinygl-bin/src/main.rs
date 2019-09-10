#[macro_use]
extern crate log;

use cgmath::vec2;

use tinygl_renderer::demo::{self, *};

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

fn main() {
    let (gl, event_loop, windowed_context, shader_version, render_size) = {
        env_logger::init();

        let render_size = vec2(1024, 768);
        let el = glutin::event_loop::EventLoop::new();
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

    // Create default context
    let mut context = demo::Context::new(gl, render_size, shader_version);

    // Compile demo
    let mut demo = demo::Demo::default();

    // First pass, renders a gradient
    demo.passes.push(PassBuilder::sample("gradient").build());

    // Second pass, take gradient, change it and return it
    demo.passes.push(
        PassBuilder::new("image")
            .with_fragment(
                r#"precision mediump float;
    in vec2 texCoords;
    out vec4 color;

    uniform sampler2D inPassGradient;
    void main() {
        color = texture(inPassGradient, texCoords) * vec4(1.0, 1.0, 0.0, 1.0);
    }"#,
            )
            .build(),
    );

    demo.compile(&context).expect("failed to compile demo");
    demo.prepare_render(&context)
        .expect("failed to prepare rendering");

    // Bind VAO for screen quad
    context.bind_vao();

    // Draw once when starting
    windowed_context.window().request_redraw();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {
                info!("Event::LoopDestroyed!");
                demo.drop(&context.gl);
                return;
            }
            Event::EventsCleared => {
                // TODO: check behavior
                windowed_context.window().request_redraw();
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    info!("WindowEvent::Resized: {:?}", logical_size);
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    let size = logical_size.to_physical(dpi_factor);
                    context.render_size = cgmath::vec2(size.width as u32, size.height as u32);
                    demo.prepare_render(&context)
                        .expect("failed to prepare rendering");
                    windowed_context.resize(size);
                }
                WindowEvent::RedrawRequested => {
                    //info!("WindowEvent::RedrawRequested");
                    context.render(&demo);
                    windowed_context.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => {
                    info!("WindowEvent::CloseRequested");
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            _ => (),
        }
    });
}
