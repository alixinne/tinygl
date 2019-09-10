use glow::*;
use wasm_bindgen::{prelude::*, JsCast};

use crate::demo::{self, *};

#[wasm_bindgen]
pub struct State {
    context: demo::Context,
    demo: demo::Demo,
}

#[wasm_bindgen]
impl State {
    pub fn get_demo(&mut self) -> JsValue {
        JsValue::from_serde(&self.demo).unwrap()
    }

    pub fn load_demo(&mut self, demo: JsValue) -> Result<(), JsValue> {
        self.demo = demo.into_serde().map_err(|error| error.to_string())?;

        Ok(())
    }

    pub fn compile_demo(&mut self) {
        self.demo
            .compile(&self.context)
            .expect("failed to compile demo");
        self.demo
            .prepare_render(&self.context)
            .expect("failed to prepare rendering");
    }

    pub fn render(&self) {
        self.demo.render(&self.context);
    }
}

pub fn run(canvas: web_sys::HtmlCanvasElement) -> Result<State, JsValue> {
    unsafe {
        // Create a context from a WebGL2 context on wasm32 targets
        let (gl, shader_version, render_size) = {
            let webgl2_context = canvas
                .get_context("webgl2")?
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()?;

            let render_size = cgmath::vec2(canvas.width().into(), canvas.height().into());

            (
                glow::Context::from_webgl2_context(webgl2_context),
                "#version 300 es",
                render_size,
            )
        };

        // Create state with GL context and in-view demo object
        let mut state = State {
            context: demo::Context::new(gl, render_size, shader_version),
            demo: demo::Demo::default(),
        };

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

        Ok(state)
    }
}

#[wasm_bindgen]
pub fn init(canvas: JsValue) -> Result<State, JsValue> {
    // Cast object into canvas object
    let canvas = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("canvas element is required");

    run(canvas)
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

    Ok(())
}
