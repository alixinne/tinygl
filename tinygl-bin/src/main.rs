#[macro_use] extern crate log;

use tinygl_renderer::demo::{self, Compilable};

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

fn main() {
    let (gl, event_loop, windowed_context, shader_version) = {
        env_logger::init();

        let el = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("tinygl")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let windowed_context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        let context = glow::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });
        (context, el, windowed_context, "#version 410")
    };

    // Create default context
    let context = demo::Context::new(shader_version, gl);

    // Compile demo
    let mut demo = demo::Demo::default();
    demo.compile(&context).expect("failed to compile demo");

    // Bind VAO for screen quad
    context.bind_vao();

    // Draw once when starting
    windowed_context.window().request_redraw();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {
                info!("Event::LoopDestroyed!");
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
                    windowed_context.resize(logical_size.to_physical(dpi_factor));
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
