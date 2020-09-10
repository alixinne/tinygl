use heck::{CamelCase, SnakeCase};

use crate::model::ShaderObject;
use crate::reflect::ReflectedObject;

#[derive(Debug)]
pub struct WrappedShader<T> {
    shader: String,
    rs_file_name: String,

    shader_struct_name: String,
    shader_variable_name: String,
    uniform_struct_name: String,
    uniform_locations_name: String,

    result: ReflectedObject<ShaderObject<T>>,
    prefer_spirv: bool,
}

impl<T> WrappedShader<T> {
    pub fn new(result: ReflectedObject<ShaderObject<T>>, prefer_spirv: bool) -> Self {
        let shader: String = result
            .info()
            .source_path
            .to_path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into();

        let base_name = shader.replace(".", "_");
        let shader_struct_name = (base_name.to_owned() + "_shader").to_camel_case();
        let shader_variable_name = shader_struct_name.to_snake_case();

        Self {
            shader,
            rs_file_name: base_name.to_owned() + ".rs",
            shader_struct_name,
            shader_variable_name,
            uniform_struct_name: (base_name.to_owned() + "_uniforms").to_camel_case(),
            uniform_locations_name: (base_name + "_locations").to_snake_case(),
            result,
            prefer_spirv,
        }
    }

    pub fn prefer_spirv(&self) -> bool {
        self.prefer_spirv
    }

    pub fn result(&self) -> &ReflectedObject<ShaderObject<T>> {
        &self.result
    }
}

pub trait WrappedShaderDetails {
    fn uniforms(&self) -> &[crate::reflect::FoundUniform];
    fn shader_variable_name(&self) -> &str;
    fn shader_struct_name(&self) -> &str;
    fn uniform_struct_name(&self) -> &str;
    fn uniform_locations_name(&self) -> &str;
}

impl<T> WrappedShaderDetails for WrappedShader<T> {
    fn uniforms(&self) -> &[crate::reflect::FoundUniform] {
        self.result().uniforms()
    }

    fn shader_variable_name(&self) -> &str {
        &self.shader_variable_name
    }

    fn shader_struct_name(&self) -> &str {
        &self.shader_struct_name
    }

    fn uniform_struct_name(&self) -> &str {
        &self.uniform_struct_name
    }

    fn uniform_locations_name(&self) -> &str {
        &self.uniform_locations_name
    }
}
