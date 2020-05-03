use crate::context::{Context, HasContext};

use super::{make_shader, ShaderCommon};

pub fn build_src_shader(
    gl: &Context,
    src: &str,
    kind: u32,
) -> crate::Result<<glow::Context as HasContext>::Shader> {
    unsafe {
        make_shader(gl, kind, |shader_name| {
            // Load the binary
            gl.shader_source(shader_name, src);

            // Specialize the binary
            gl.compile_shader(shader_name);
        })
    }
}

/// GLSL shader wrapper
pub trait SourceShader<'a>: ShaderCommon {
    fn get_source() -> &'a str;

    fn build(gl: &Context, kind: u32) -> crate::Result<<glow::Context as HasContext>::Shader> {
        build_src_shader(gl, Self::get_source(), kind)
    }
}
