use crate::wrappers::GlDrop;
use crate::Context;

use super::ShaderCommon;

pub struct RuntimeShader {
    kind: u32,
    name: crate::gl::Shader,
}

impl RuntimeShader {
    #[cfg(all(not(target_arch = "wasm32"), feature = "opengl46"))]
    pub fn build_bin(gl: &Context, binary: &[u8], kind: u32) -> crate::Result<Self> {
        Ok(Self {
            kind,
            name: super::binary_shader::build_bin_shader(gl, binary, kind)?,
        })
    }

    pub fn build_src(gl: &Context, src: &str, kind: u32) -> crate::Result<Self> {
        Ok(Self {
            kind,
            name: super::source_shader::build_src_shader(gl, src, kind)?,
        })
    }
}

impl ShaderCommon for RuntimeShader {
    fn kind(&self) -> u32 {
        self.kind
    }

    impl_name!(crate::gl::ShaderName);
}

impl GlDrop for RuntimeShader {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn drop(&mut self, gl: &Context) {
        gl.delete_shader(self.name);
    }

    #[cfg(target_arch = "wasm32")]
    unsafe fn drop(&mut self, gl: &Context) {
        gl.delete_shader(Some(&self.name));
    }
}
