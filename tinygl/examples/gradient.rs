use tinygl::gl;
use tinygl::prelude::*;

use glutin::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::ContextBuilder;

const VERTEX_SHADER: &'static str = r#"
#version 460 core

layout(location = 0) out vec2 fragCoord;

void main() {
    fragCoord = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2);
    gl_Position = vec4(fragCoord * 2. - 1., 0., 1.);
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 460 core

layout(location = 0) in vec2 fragCoord;
layout(location = 0) out vec4 fragColor;

uniform float iTime;

void main() {
    // Normalized pixel coordinates (from 0 to 1)
    vec2 fragCoord = fragCoord;

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+fragCoord.xyx+vec3(0,2,4));

    // Output to screen
    fragColor = vec4(col,1.0);
}
"#;

fn main() -> Result<(), String> {
    let el = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_title("tinygl example - gradient")
        .with_inner_size(glutin::dpi::LogicalSize::new(768.0, 768.0));

    let windowed_context = ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 6)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_gl_debug_flag(true)
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();

    let (gl, windowed_context) = unsafe {
        let current = windowed_context
            .make_current()
            .expect("failed to make window context current");
        (
            tinygl::Context::from_loader_function(|s| current.get_proc_address(s) as *const _),
            current,
        )
    };

    // Build program
    let program = {
        // Build vertex shader
        let vertex_shader = tinygl::wrappers::GlRefHandle::new(
            &gl,
            tinygl::wrappers::RuntimeShader::build_src(&gl, VERTEX_SHADER, gl::VERTEX_SHADER)
                .expect("failed to compile vertex shader"),
        );

        // Build fragment shader
        let fragment_shader = tinygl::wrappers::GlRefHandle::new(
            &gl,
            tinygl::wrappers::RuntimeShader::build_src(&gl, FRAGMENT_SHADER, gl::FRAGMENT_SHADER)
                .expect("failed to build fragment shader"),
        );

        tinygl::wrappers::RuntimeProgramBuilder::new(&gl)
            .shader(&*vertex_shader)
            .shader(&*fragment_shader)
            .build()
            .expect("failed to link program")
    };

    // Use program
    program.use_program(&gl);

    // Build and bind an empty VAO
    let vao = tinygl::wrappers::VertexArray::new(&gl).expect("failed to create vertex array");
    vao.bind(&gl);

    // Monitors
    let fullscreen = Some(Fullscreen::Borderless(
        el.available_monitors()
            .nth(0)
            .expect("no avilable monitors"),
    ));

    el.run(move |event, _target, control_flow| {
        // Default behavior: wait for events
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    input.virtual_keycode.map(|key| {
                        if let ElementState::Pressed = input.state {
                            match key {
                                VirtualKeyCode::Escape => {
                                    *control_flow = ControlFlow::Exit;
                                }
                                VirtualKeyCode::F11 => {
                                    if windowed_context.window().fullscreen().is_some() {
                                        windowed_context.window().set_fullscreen(None);
                                    } else {
                                        windowed_context
                                            .window()
                                            .set_fullscreen(fullscreen.clone());
                                    }
                                }
                                _ => {}
                            }
                        }
                    });
                }
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    unsafe {
                        gl.viewport(
                            0,
                            0,
                            physical_size.width as i32,
                            physical_size.height as i32,
                        );
                    }
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            Event::RedrawRequested(_) => {
                // Render demo
                unsafe {
                    // Clear framebuffer
                    gl.clear_color(1.0, 0.0, 1.0, 1.0);
                    gl.clear(tinygl::gl::COLOR_BUFFER_BIT);

                    // Render
                    gl.draw_arrays(gl::TRIANGLES, 0, 3);
                }

                windowed_context.swap_buffers().unwrap();
            }
            Event::RedrawEventsCleared => {
                // windowed_context.window().request_redraw();
            }
            _ => {}
        }
    });
}
