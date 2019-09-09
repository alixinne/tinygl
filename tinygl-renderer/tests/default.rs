mod common;
use common::*;

use tinygl_renderer::demo::*;

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn compile_default() {
    with_window(|gl, shader_version| {
        let context = Context::new(shader_version, gl);
        let mut demo = Demo::default();

        demo.compile(&context)
            .expect("failed to compile default demo");

        context.bind_vao();
        demo.render(&context);
    });
}
