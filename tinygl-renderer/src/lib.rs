pub mod demo;

#[macro_use]
extern crate log;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() -> Result<(), JsValue> {
    use glow::*;
    use demo::Compilable;

    #[cfg(debug_assertions)]
    {
        // This provides better error messages in debug mode.
        console_error_panic_hook::set_once();

        // Set logger
        console_log::init_with_level(log::Level::Debug).unwrap();
    }

    unsafe {
        // Create a context from a WebGL2 context on wasm32 targets
        let (_window, gl, _events_loop, render_loop, shader_version) = {
            use wasm_bindgen::JsCast;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            let webgl2_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            (
                (),
                glow::Context::from_webgl2_context(webgl2_context),
                (),
                glow::RenderLoop::from_request_animation_frame(),
                "#version 300 es",
            )
        };

        // Create a context from a glutin window on non-wasm32 targets
        let context = demo::Context::new(shader_version, gl);

        let mut demo = demo::Demo::default();
        demo.compile(&context).expect("failed to compile demo");

        // Bind VAO so draw_arrays can be called
        context.bind_vao();

        // Set default clear color
        context.gl.clear_color(0.1, 0.2, 0.3, 1.0);

        // Web render loop
        render_loop.run(move |_running: &mut bool| {
            context.render(&demo);
        });
    }

    Ok(())
}
