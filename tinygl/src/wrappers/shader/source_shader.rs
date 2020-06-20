use crate::Context;

use super::{make_shader, ShaderCommon};

#[cfg(not(target_arch = "wasm32"))]
pub fn build_src_shader(gl: &Context, src: &str, kind: u32) -> crate::Result<crate::gl::Shader> {
    unsafe {
        make_shader(gl, kind, |shader_name| {
            // Load the source
            gl.shader_source(
                shader_name,
                1,
                &(src.as_ptr() as *const i8),
                &(src.len() as i32),
            );

            // Specialize the binary
            gl.compile_shader(shader_name);
        })
    }
}

#[cfg(target_arch = "wasm32")]
pub fn build_src_shader(gl: &Context, src: &str, kind: u32) -> crate::Result<crate::gl::Shader> {
    unsafe {
        make_shader(gl, kind, |shader_name| {
            // Load the source
            gl.shader_source(shader_name, src);

            // Specialize the binary
            gl.compile_shader(shader_name);
        })
    }
}

/// GLSL shader wrapper
pub trait SourceShader<'a>: ShaderCommon {
    fn get_source() -> &'a str;

    fn build(gl: &Context, kind: u32) -> crate::Result<crate::gl::Shader> {
        build_src_shader(gl, Self::get_source(), kind)
    }
}
