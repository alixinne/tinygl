use crate::Context;

use super::{make_shader, ShaderCommon};

pub fn build_bin_shader(
    gl: &Context,
    binary: &[u8],
    kind: u32,
) -> crate::Result<crate::gl::Shader> {
    unsafe {
        make_shader(gl, kind, |shader_name| {
            use crate::gl;

            // Load the binary
            gl.shader_binary(
                1,
                &shader_name,
                gl::SHADER_BINARY_FORMAT_SPIR_V,
                binary.as_ptr() as *const std::ffi::c_void,
                binary.len() as i32,
            );

            // Specialize the binary
            gl.specialize_shader(
                shader_name,
                b"main\0".as_ptr() as *const i8,
                0,
                std::ptr::null(),
                std::ptr::null(),
            );
        })
    }
}

/// SPIR-V shader wrapper
pub trait BinaryShader<'a>: ShaderCommon {
    fn get_binary() -> &'a [u8];

    fn build(gl: &Context, kind: u32) -> crate::Result<crate::gl::Shader> {
        build_bin_shader(gl, Self::get_binary(), kind)
    }
}
