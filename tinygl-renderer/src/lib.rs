pub mod demo;

#[macro_use]
extern crate log;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() -> Result<(), JsValue> {
    use cgmath::prelude::*;
    use demo::*;
    use glow::*;

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

        // Create a context from a glutin window on non-wasm32 targets
        let context = demo::Context::new(gl, render_size, shader_version);

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

        // Bind VAO so draw_arrays can be called
        context.bind_vao();

        // Set default clear color
        context.gl.clear_color(0.1, 0.2, 0.3, 1.0);

        // Web render loop
        render_loop.run(move |running: &mut bool| {
            context.render(&demo);

            if !*running {
                // Delete resources
                demo.drop(&context.gl);
            }
        });
    }

    Ok(())
}
