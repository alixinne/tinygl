#[cfg(target_arch = "wasm32")]
mod web {
    use wasm_bindgen::JsCast;
    pub use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    pub fn with_window<F: FnOnce(glow::Context, &'static str)>(cb: F) {
        let document = web_sys::window()
            .expect("failed to get window")
            .document()
            .expect("failed to get document");

        // Create canvas
        let canvas = document
            .create_element("canvas")
            .expect("failed to create canvas");
        canvas
            .set_attribute("id", "canvas")
            .expect("failed to set canvas id");
        document
            .body()
            .expect("failed to get body")
            .append_child(&canvas)
            .expect("failed to append canvas child");

        let (_window, gl, _events_loop, _render_loop, shader_version) = {
            let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
            let webgl2_context = canvas
                .get_context("webgl2")
                .expect("failed to get_context(webgl2)")
                .expect("no webgl2 context returned")
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

        cb(gl, shader_version);
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod desktop {
    pub fn with_window<F: FnOnce(glow::Context, &'static str)>(cb: F) {
        unsafe {
            let (gl, _event_loop, _headless_context, shader_version) = {
                env_logger::init();

                let el = glutin::event_loop::EventLoop::new();
                let size = glutin::dpi::PhysicalSize::new(1024.0, 768.0);
                let headless_context = glutin::ContextBuilder::new()
                    .build_headless(&el, size)
                    .unwrap();
                let headless_context = headless_context.make_current().unwrap();
                let context = glow::Context::from_loader_function(|s| {
                    headless_context.get_proc_address(s) as *const _
                });
                (context, el, headless_context, "#version 410")
            };

            cb(gl, shader_version);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(not(target_arch = "wasm32"))]
pub use desktop::*;
