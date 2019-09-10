mod common;
use common::*;

use tinygl_renderer::demo::*;

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn compile_default() {
    with_window(|gl, render_size, shader_version| {
        let context = Context::new(gl, render_size, shader_version);
        let mut demo = Demo::default();

        demo.compile(&context)
            .expect("failed to compile default demo");

        demo.prepare_render(&context)
            .expect("failed to prepare render");

        context.bind_vao();
        demo.render(&context);

        demo.drop(&context.gl);
    });
}
