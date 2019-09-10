use std::sync::{Arc, Mutex};

use cgmath::prelude::*;
use glow::*;
use lazy_static::lazy_static;
use wasm_bindgen::{prelude::*, JsCast};

use crate::demo::{self, *};

struct State {
    context: demo::Context,
    demo: demo::Demo,
}

impl State {
    pub fn load_demo(&mut self, demo: Demo) {
        self.demo = demo;
    }

    pub fn compile_demo(&mut self) {
        self.demo
            .compile(&self.context)
            .expect("failed to compile demo");
        self.demo
            .prepare_render(&self.context)
            .expect("failed to prepare rendering");
    }
}

// ?
unsafe impl Send for State {}

lazy_static! {
    static ref STATE: Arc<Mutex<Option<State>>> = Arc::new(Mutex::new(None));
}

#[wasm_bindgen]
pub fn load_demo(val: &JsValue) {
    let demo: Demo = val.into_serde().expect("invalid demo data");

    let mut state_lock = STATE.lock().unwrap();
    let state = state_lock.as_mut().unwrap();
    state.load_demo(demo);
    state.compile_demo();
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn wasm_main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    {
        // This provides better error messages in debug mode.
        console_error_panic_hook::set_once();

        // Set logger
        console_log::init_with_level(log::Level::Debug).unwrap()
    }

    unsafe {
        // Create a context from a WebGL2 context on wasm32 targets
        let (gl, render_loop, shader_version, render_size) = {
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()?;
            let webgl2_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();

            let render_size = cgmath::vec2(canvas.width().into(), canvas.height().into());

            (
                glow::Context::from_webgl2_context(webgl2_context),
                glow::RenderLoop::from_request_animation_frame(),
                "#version 300 es",
                render_size,
            )
        };

        let mut state_mut = STATE.lock().unwrap();

        // Create state with GL context and in-view demo object
        *state_mut = Some(State {
            context: demo::Context::new(gl, render_size, shader_version),
            demo: demo::Demo::default(),
        });

        let state = &mut state_mut.as_mut().unwrap();

        let demo = &mut state.demo;

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

        // Compile demo
        state.compile_demo();

        // Bind VAO so draw_arrays can be called
        state.context.bind_vao();

        // Set default clear color
        state.context.gl.clear_color(0.1, 0.2, 0.3, 1.0);

        // Web render loop
        render_loop.run(|running: &mut bool| {
            let mut state_mut = STATE.lock().unwrap();
            let state = &mut state_mut.as_mut().unwrap();
            let context = &mut state.context;
            let demo = &mut state.demo;

            context.render(&demo);

            if !*running {
                // Delete resources
                demo.drop(&context.gl);
            }
        });
    }

    Ok(())
}
