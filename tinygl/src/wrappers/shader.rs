use crate::OpenGlErrorCode;

#[cfg(all(not(target_arch = "wasm32"), feature = "opengl46"))]
mod binary_shader;
#[cfg(all(not(target_arch = "wasm32"), feature = "opengl46"))]
pub use binary_shader::BinaryShader;

mod runtime_shader;
pub use runtime_shader::*;

mod shader_common;
pub use shader_common::*;

mod source_shader;
pub use source_shader::SourceShader;

/// Build a shader name and try to compile it using the given callback
unsafe fn make_shader<F>(
    gl: &crate::Context,
    kind: u32,
    mut compile_cb: F,
) -> crate::Result<crate::gl::Shader>
where
    F: FnMut(crate::gl::ShaderName) -> (),
{
    // Create shader object
    let shader_name = gl
        .create_shader(kind)
        .ok_or_else(|| crate::Error::ShaderCreationFailed(OpenGlErrorCode(gl.get_error())))?;

    let name = make_name!(shader_name);

    compile_cb(name);

    // Check that the compile status is ok
    if !gl.get_shader_compile_status(name) {
        let log = gl.get_shader_info_log(name);
        gl.delete_shader(make_name!(Option => shader_name));
        return Err(crate::Error::ShaderCompilationFailed(
            log.unwrap_or_else(String::new),
        ));
    }

    Ok(shader_name)
}
