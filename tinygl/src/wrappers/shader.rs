use crate::context::{Context, HasContext};

#[cfg(not(target_arch = "wasm32"))]
mod binary_shader;
#[cfg(not(target_arch = "wasm32"))]
pub use binary_shader::BinaryShader;

mod runtime_shader;
pub use runtime_shader::*;

mod shader_common;
pub use shader_common::*;

mod source_shader;
pub use source_shader::SourceShader;

/// Build a shader name and try to compile it using the given callback
unsafe fn make_shader<F>(
    gl: &Context,
    kind: u32,
    mut compile_cb: F,
) -> crate::Result<<glow::Context as HasContext>::Shader>
where
    F: FnMut(<glow::Context as HasContext>::Shader) -> (),
{
    // Create shader object
    let shader_name = gl
        .create_shader(kind)
        .map_err(|msg| crate::Error::ShaderCreationFailed(msg))?;

    compile_cb(shader_name);

    // Check that the compile status is ok
    if !gl.get_shader_compile_status(shader_name) {
        let log = gl.get_shader_info_log(shader_name);
        gl.delete_shader(shader_name);
        return Err(crate::Error::ShaderCompilationFailed(log));
    }

    Ok(shader_name)
}
