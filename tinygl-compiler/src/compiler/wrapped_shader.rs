use std::path::{Path, PathBuf};

use heck::{CamelCase, SnakeCase};

use rspirv::dr as rr;

use super::TargetType;
use crate::shader_kind::ShaderKindInfo;

pub struct WrappedShader {
    shader: String,
    rs_file_name: String,
    uniforms: Vec<crate::reflect::FoundUniform>,
    kind: ShaderKindInfo,
    source_path: PathBuf,

    shader_struct_name: String,
    shader_variable_name: String,
    uniform_struct_name: String,
    uniform_locations_name: String,

    binary_result: shaderc::CompilationArtifact,
    output_type: TargetType,
    skip_spirv: bool,
}

impl std::fmt::Debug for WrappedShader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WrappedShader")
            .field("shader", &self.shader)
            .field("rs_file_name", &self.rs_file_name)
            .field("uniforms", &self.uniforms)
            .field("kind", &self.kind)
            .field("source_path", &self.source_path)
            .field("output_type", &self.output_type)
            .field("skip_spirv", &self.skip_spirv)
            .finish()
    }
}

impl WrappedShader {
    pub fn new(
        kind: ShaderKindInfo,
        source_path: &Path,
        binary_result: shaderc::CompilationArtifact,
        output_type: TargetType,
        skip_spirv: bool,
    ) -> Self {
        let shader: String = source_path.file_name().unwrap().to_string_lossy().into();

        let base_name = shader.replace(".", "_");
        let shader_struct_name = (base_name.to_owned() + "_shader").to_camel_case();
        let shader_variable_name = shader_struct_name.to_snake_case();

        Self {
            shader,
            rs_file_name: base_name.to_owned() + ".rs",
            uniforms: Vec::new(),
            kind,
            source_path: source_path.to_owned(),
            shader_struct_name,
            shader_variable_name,
            uniform_struct_name: (base_name.to_owned() + "_uniforms").to_camel_case(),
            uniform_locations_name: (base_name + "_locations").to_snake_case(),
            binary_result,
            output_type,
            skip_spirv,
        }
    }
    pub fn reflect_uniforms(&mut self) -> crate::Result<()> {
        // Extract uniform data
        let mut loader = rr::Loader::new();
        rspirv::binary::parse_words(self.binary_result.as_binary(), &mut loader).unwrap_or_else(
            |_| {
                panic!(
                    "failed to parse binary module for {}",
                    self.source_path.to_string_lossy()
                )
            },
        );

        self.uniforms =
            crate::reflect::find_uniforms(&self.source_path.to_string_lossy(), &loader.module())?;

        Ok(())
    }

    pub fn transpile(&self, version: spirv_cross::glsl::Version) -> crate::Result<String> {
        // Use spirv_cross to write valid code
        let module = spirv_cross::spirv::Module::from_words(self.binary_result.as_binary());
        let mut ast = spirv_cross::spirv::Ast::<spirv_cross::glsl::Target>::parse(&module)?;

        // Target the right GLSL version
        ast.set_compiler_options(&spirv_cross::glsl::CompilerOptions {
            version,
            ..Default::default()
        })
        .unwrap();

        Ok(ast.compile()?)
    }

    pub fn kind(&self) -> &ShaderKindInfo {
        &self.kind
    }

    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    pub fn uniforms(&self) -> &[crate::reflect::FoundUniform] {
        &self.uniforms[..]
    }

    pub fn shader_struct_name(&self) -> &str {
        &self.shader_struct_name
    }

    pub fn shader_variable_name(&self) -> &str {
        &self.shader_variable_name
    }

    pub fn uniform_struct_name(&self) -> &str {
        &self.uniform_struct_name
    }

    pub fn uniform_locations_name(&self) -> &str {
        &self.uniform_locations_name
    }

    pub fn output_type(&self) -> TargetType {
        self.output_type
    }

    pub fn skip_spirv(&self) -> bool {
        self.skip_spirv
    }

    pub fn text_result(&self) -> String {
        self.binary_result.as_text()
    }

    pub fn binary_result_u8(&self) -> &[u8] {
        self.binary_result.as_binary_u8()
    }
}
