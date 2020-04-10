use crate::context::{Context, HasContext};

use super::ShaderCommon;

pub struct RuntimeShader {
    kind: u32,
    name: <glow::Context as HasContext>::Shader,
}

impl RuntimeShader {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build_bin(gl: &Context, binary: &[u8], kind: u32) -> Result<Self, String> {
        Ok(Self {
            kind,
            name: super::binary_shader::build_bin_shader(gl, binary, kind)?,
        })
    }

    pub fn build_src(gl: &Context, src: &str, kind: u32) -> Result<Self, String> {
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
    fn name(&self) -> <glow::Context as HasContext>::Shader {
        self.name
    }
}
