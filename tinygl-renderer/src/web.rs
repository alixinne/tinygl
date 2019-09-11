use glow::*;
use wasm_bindgen::{prelude::*, JsCast};

use crate::demo::{self, *};

#[wasm_bindgen]
pub struct State {
    context: demo::Context,
    demo: Option<demo::Demo>,
}

#[wasm_bindgen]
impl State {
    pub fn get_demo(&mut self) -> JsValue {
        JsValue::from_serde(&self.demo).unwrap()
    }

    pub fn load_demo(&mut self, demo: JsValue) -> Result<(), JsValue> {
        let mut new_demo: Demo = demo.into_serde().map_err(|error| error.to_string())?;

        new_demo
            .compile(&self.context)
            .map_err(|error| error.to_string())?;

        new_demo
            .prepare_render(&self.context)
            .map_err(|error| error.to_string())?;

        self.demo.replace(new_demo).map(|mut old_demo| {
            old_demo.drop(&self.context.gl);
        });

        Ok(())
    }

    pub fn render(&self) {
        if let Some(demo) = &self.demo {
            demo.render(&self.context);
        } else {
            unsafe {
                self.context.gl.clear(glow::COLOR_BUFFER_BIT);
            }
        }
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
        let state = State {
            context: demo::Context::new(gl, render_size, shader_version),
            demo: None,
        };

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
