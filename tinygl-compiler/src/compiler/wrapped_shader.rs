use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use heck::CamelCase;
use heck::SnakeCase;

use rspirv::dr as rr;

use super::{TargetType, WrappedItem};
use crate::shader_kind::ShaderKindInfo;
use crate::types::prelude::*;

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

    pub fn reflect_uniforms(&mut self) -> Result<(), crate::Error> {
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

    fn write_shader(&self, dest: impl AsRef<Path>) -> crate::Result<String> {
        let shader_file_name = format!(
            "{}{}",
            self.shader,
            if let TargetType::SpirV = self.output_type {
                ".spv"
            } else {
                ""
            }
        );

        // Write binary to .spv/.glsl file
        let mut output = File::create(&Path::new(dest.as_ref()).join(&shader_file_name))?;

        match self.output_type {
            TargetType::SpirV => {
                // Just write spv file
                output.write_all(self.binary_result.as_binary_u8())?;
            }
            TargetType::Glsl(version) => {
                if self.skip_spirv {
                    // We skipped SPIR-V generation so just fix invalid stuff for OpenGL ES targets
                    // WebGL is more sensitive to leftovers from includes and stuff
                    // TODO: This is an ugly hack, maybe forbid skip_spirv + ES 3.00?
                    for l in self.binary_result.as_text().lines() {
                        if l.starts_with("#extension GL_GOOGLE_include_directive") {
                            continue;
                        } else if l.starts_with("#line") {
                            writeln!(output, "//{}", l)?;
                        } else {
                            writeln!(output, "{}", l)?;
                        }
                    }
                } else {
                    // Use spirv_cross to write valid code
                    let module =
                        spirv_cross::spirv::Module::from_words(self.binary_result.as_binary());
                    let mut ast =
                        spirv_cross::spirv::Ast::<spirv_cross::glsl::Target>::parse(&module)?;

                    // Target the right GLSL version
                    ast.set_compiler_options(&spirv_cross::glsl::CompilerOptions {
                        version,
                        ..Default::default()
                    })
                    .unwrap();

                    write!(output, "{}", ast.compile()?)?;
                }
            }
            _ => unreachable!(),
        }

        Ok(shader_file_name)
    }

    fn write_rust_wrapper(
        &self,
        dest: impl AsRef<Path>,
        shader_file_name: &str,
    ) -> crate::Result<()> {
        // Write Rust interface code
        let output_rs = File::create(&Path::new(dest.as_ref()).join(&self.rs_file_name)).unwrap();
        let mut wr = BufWriter::new(output_rs);

        // Shader resource structure
        writeln!(wr, "/// {} Rust wrapper", self.shader)?;
        writeln!(wr, "pub struct {} {{", self.shader_struct_name())?;
        writeln!(wr, "    name: ::tinygl::gl::Shader,")?;
        writeln!(wr, "}}")?;

        writeln!(wr, "impl {} {{", self.shader_struct_name())?;
        writeln!(
            wr,
            "    pub fn build(gl: &::tinygl::Context) -> ::tinygl::Result<Self> {{"
        )?;
        writeln!(
            wr,
            "        Ok(Self {{ name: <Self as {st}>::build(gl, ::tinygl::gl::{kind})? }})",
            st = if self.output_type.is_source() {
                "::tinygl::wrappers::SourceShader"
            } else {
                "::tinygl::wrappers::BinaryShader"
            },
            kind = self.kind.constant_name,
        )?;
        writeln!(wr, "    }}")?;
        writeln!(wr, "}}")?;

        // Write struct for holding uniform locations
        writeln!(wr, "#[derive(Default)]")?;
        writeln!(wr, "pub struct {} {{", self.uniform_struct_name())?;

        for uniform in &self.uniforms {
            writeln!(
                wr,
                "    {name}: Option<::tinygl::gl::UniformLocation>,",
                name = uniform.location_name()
            )?;
        }
        writeln!(wr, "}}")?;

        writeln!(wr, "impl {} {{", self.uniform_struct_name())?;
        // Write constructor
        writeln!(
            wr,
            "    pub fn new({prefix}gl: &::tinygl::Context, {prefix}program: ::tinygl::gl::Program) -> Self {{",
            prefix = if self.output_type.is_source() {
                if self.uniforms.is_empty() {
                    "_"
                } else {
                    ""
                }
            } else {
                "_"
            })?;
        writeln!(wr, "        Self {{")?;

        for uniform in &self.uniforms {
            if self.output_type.is_source() {
                // Source shader: find uniform locations from variable names
                writeln!(wr, "            {name}: unsafe {{ gl.get_uniform_location(program, \"{uniform_name}\") }},",
                    name = uniform.location_name(),
                    uniform_name = uniform.name)?;
            } else {
                // Binary shader: assume locations form reflection on SPIR-V
                writeln!(
                    wr,
                    "            {name}: Some({location}),",
                    name = uniform.location_name(),
                    location = uniform.location
                )?;
            }
        }

        writeln!(wr, "        }}")?;
        writeln!(wr, "    }}")?;

        // Write getter/setter methods
        for uniform in &self.uniforms {
            let ty = uniform.ty.unwrap();

            if let Some(binding) = uniform.binding {
                writeln!(
                    wr,
                    "    pub fn get_{uniform_sc_name}_binding(&self) -> {type_name} {{",
                    uniform_sc_name = uniform.name.to_snake_case(),
                    type_name = ty.rust_value_type(),
                )?;
                writeln!(wr, "        {}", binding)?;
                writeln!(wr, "    }}")?;
            }

            writeln!(
                wr,
                "    pub fn set_{uniform_sc_name}(&self, gl: &::tinygl::Context, program: ::tinygl::gl::ProgramName, {extra}value: {type_name}) {{",
                uniform_sc_name = uniform.name.to_snake_case(),
                type_name = ty.rust_value_type(),
                extra = ty.uniform_method_extra_args_with_ty().map_or_else(|| String::new(), |x| format!("{}, ", x)),
            )?;

            writeln!(
                wr,
                "        if let Some(location) = self.{location} {{ unsafe {{ gl.program_uniform{prefix}(program, location, {count}{extra}{what}) }}; }}",
                prefix = ty.uniform_method_name(),
                count = ty.uniform_count_arg(),
                location = uniform.location_name(),
                what = ty.uniform_value("value"),
                extra = ty.uniform_method_extra_args_val().map_or_else(|| String::new(), |x| format!("{}, ", x)),
            )?;

            writeln!(wr, "    }}")?;
        }
        writeln!(wr, "}}")?;

        // A wrapped shader implements ShaderCommon
        writeln!(
            wr,
            "impl ::tinygl::wrappers::ShaderCommon for {} {{",
            self.shader_struct_name()
        )?;
        writeln!(wr, "    fn kind(&self) -> u32 {{")?;
        writeln!(wr, "        ::tinygl::gl::{}", self.kind.constant_name)?;
        writeln!(wr, "    }}")?;
        writeln!(wr, "    fn name(&self) -> ::tinygl::gl::Shader {{")?;
        writeln!(wr, "        self.name")?;
        writeln!(wr, "    }}")?;
        writeln!(wr, "}}")?;

        // Implement GlDrop
        writeln!(
            wr,
            "impl ::tinygl::wrappers::GlDrop for {} {{",
            self.shader_struct_name()
        )?;
        writeln!(
            wr,
            "    unsafe fn drop(&mut self, gl: &::tinygl::Context) {{"
        )?;
        writeln!(wr, "        use ::tinygl::prelude::*;")?;
        writeln!(wr, "        gl.delete_shader(self.name());")?;
        writeln!(wr, "    }}")?;
        writeln!(wr, "}}")?;

        // Implement the right shader trait for the given output type
        if self.output_type.is_source() {
            writeln!(
                wr,
                "impl ::tinygl::wrappers::SourceShader<'static> for {} {{",
                self.shader_struct_name()
            )?;
            writeln!(wr, "    fn get_source() -> &'static str {{")?;
            writeln!(wr, "        include_str!(\"{}\")", shader_file_name)?;
            writeln!(wr, "    }}")?;
            writeln!(wr, "}}")?;
        } else {
            writeln!(
                wr,
                "impl ::tinygl::wrappers::BinaryShader<'static> for {} {{",
                self.shader_struct_name()
            )?;
            writeln!(wr, "    fn get_binary() -> &'static [u8] {{")?;
            writeln!(wr, "        include_bytes!(\"{}\")", shader_file_name)?;
            writeln!(wr, "    }}")?;
            writeln!(wr, "}}")?;
        }

        Ok(())
    }
}

impl WrappedItem for WrappedShader {
    fn write(&self, dest: &Path) -> Result<(), crate::Error> {
        self.write_rust_wrapper(dest, &self.write_shader(dest)?)
    }

    fn write_root_include(&self, wr: &mut dyn Write) -> Result<(), crate::Error> {
        writeln!(wr, "// {} shader", self.source_path.to_string_lossy())?;
        writeln!(wr, "include!(\"{}\");", self.rs_file_name)?;
        Ok(())
    }
}
