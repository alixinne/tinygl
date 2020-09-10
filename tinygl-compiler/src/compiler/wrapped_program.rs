use heck::CamelCase;

use super::wrapped_shader::*;

pub struct WrappedProgram<'s> {
    struct_name: String,
    attached_shaders: Vec<&'s dyn WrappedShaderDetails>,
}

impl<'s> WrappedProgram<'s> {
    pub fn new(program_name: &str, attached_shaders: &[&'s dyn WrappedShaderDetails]) -> Self {
        let struct_name = program_name.to_camel_case() + "Program";

        Self {
            struct_name,
            attached_shaders: attached_shaders.to_vec(),
        }
    }

    pub fn struct_name(&self) -> &str {
        &self.struct_name
    }

    pub fn shaders(&self) -> impl Iterator<Item = &&'s dyn WrappedShaderDetails> {
        self.attached_shaders.iter()
    }

    pub fn shaders_with_uniforms(&self) -> impl Iterator<Item = &&'s dyn WrappedShaderDetails> {
        self.attached_shaders
            .iter()
            .filter(|s| !s.uniforms().is_empty())
    }
}
