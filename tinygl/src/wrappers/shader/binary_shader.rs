use crate::context::{Context, HasContext};

use super::{make_shader, ShaderCommon};

pub fn build_bin_shader(
    gl: &Context,
    binary: &[u8],
    kind: u32,
) -> Result<<glow::Context as HasContext>::Shader, String> {
    unsafe {
        make_shader(gl, kind, |shader_name| {
            use crate::gl;

            // Load the binary
            gl.shader_binary(&[shader_name], gl::SHADER_BINARY_FORMAT_SPIR_V, binary);

            // Specialize the binary
            gl.specialize_shader(shader_name, "main", &[], &[]);
        })
    }
}

/// SPIR-V shader wrapper
pub trait BinaryShader<'a>: ShaderCommon {
    fn get_binary() -> &'a [u8];

    fn build(gl: &Context, kind: u32) -> Result<<glow::Context as HasContext>::Shader, String> {
        build_bin_shader(gl, Self::get_binary(), kind)
    }
}
