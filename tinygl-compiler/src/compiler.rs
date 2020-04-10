use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::{shader_kind::ShaderKindInfo, Error, Result};

mod target_type;
pub use target_type::TargetType;

mod uniform_set;
pub use uniform_set::*;

mod wrapped_item;
pub use wrapped_item::*;

mod wrapped_shader;
pub use wrapped_shader::*;

mod wrapped_program;
pub use wrapped_program::*;

#[derive(Default)]
pub struct CompilerBuilder {
    skip_cargo: bool,
    skip_spirv: bool,
    output_type: TargetType,
}

impl CompilerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn skip_cargo(self, skip_cargo: bool) -> Self {
        Self { skip_cargo, ..self }
    }

    pub fn skip_spirv(self, skip_spirv: bool) -> Self {
        Self { skip_spirv, ..self }
    }

    pub fn output_type(self, output_type: TargetType) -> Self {
        Self {
            output_type,
            ..self
        }
    }

    pub fn build(self) -> Result<Compiler> {
        // Are we building for WASM?
        let is_wasm = std::env::var("TARGET")
            .map(|v| v.starts_with("wasm32"))
            .unwrap_or(false);

        // If building for WASM, force source usage unless a specific version is required
        let output_type = match self.output_type {
            TargetType::Automatic => {
                if is_wasm {
                    TargetType::Glsl(spirv_cross::glsl::Version::V3_00Es)
                } else if self.skip_spirv {
                    TargetType::Glsl(spirv_cross::glsl::Version::V4_60)
                } else {
                    TargetType::SpirV
                }
            }
            TargetType::SpirV => {
                if is_wasm {
                    return Err(Error::InvalidTargetType(self.output_type));
                } else if self.skip_spirv {
                    return Err(Error::InvalidSkipSpirV);
                } else {
                    TargetType::SpirV
                }
            }
            TargetType::Glsl(version) => {
                if is_wasm {
                    match version {
                        spirv_cross::glsl::Version::V3_00Es
                        | spirv_cross::glsl::Version::V1_00Es => TargetType::Glsl(version),
                        _ => {
                            return Err(Error::InvalidTargetType(self.output_type));
                        }
                    }
                } else {
                    TargetType::Glsl(version)
                }
            }
        };

        Ok(Compiler {
            compiler: shaderc::Compiler::new().unwrap(),
            skip_cargo: self.skip_cargo,
            skip_spirv: self.skip_spirv,
            output_type,
        })
    }
}

pub struct Compiler {
    compiler: shaderc::Compiler,
    skip_cargo: bool,
    skip_spirv: bool,
    output_type: TargetType,
}

impl Compiler {
    fn wrap_shader_id(
        &mut self,
        source: &str,
        source_path: impl AsRef<Path>,
        kind: ShaderKindInfo,
    ) -> Result<WrappedShader> {
        // Set callback
        let mut options = shaderc::CompileOptions::new().unwrap();

        // Add definitions
        // TODO: Let use configure options?
        options.add_macro_definition("TINYGL", Some(env!("CARGO_PKG_VERSION_MAJOR")));

        // Default to OpenGL targets
        options.set_target_env(shaderc::TargetEnv::OpenGL, 0);

        // Set include callback
        let skip_cargo = self.skip_cargo;
        options.set_include_callback(move |name, _include_type, source, _depth| {
            // TODO: Circular includes?
            // TODO: Better include resolver?
            match std::fs::canonicalize(Path::new(&source).parent().unwrap().join(name)) {
                Ok(full_path) => {
                    if !skip_cargo {
                        // Notify cargo to rerun if included file changed
                        println!("cargo:rerun-if-changed={}", full_path.display());
                    }

                    match std::fs::read_to_string(&full_path) {
                        Ok(content) => Ok(shaderc::ResolvedInclude {
                            resolved_name: full_path.to_string_lossy().to_string(),
                            content,
                        }),
                        Err(error) => Err(error.to_string()),
                    }
                }
                Err(error) => Err(error.to_string()),
            }
        });

        let compiler_result = if self.skip_spirv {
            // Only assemble source if we're skipping SPIR-V
            self.compiler.preprocess(
                &source,
                &source_path.as_ref().to_string_lossy(),
                "main",
                Some(&options),
            )
        } else {
            // Compile into SPIR-V
            self.compiler.compile_into_spirv(
                &source,
                kind.shaderc_kind,
                &source_path.as_ref().to_string_lossy(),
                "main",
                Some(&options),
            )
        };

        match compiler_result {
            Ok(binary_result) => {
                if binary_result.get_num_warnings() > 0 {
                    for l in binary_result.get_warning_messages().lines() {
                        println!("cargo:warning={}", l);
                    }
                }

                // Base name to identify this shader
                let mut wrapped_shader = WrappedShader::new(
                    kind,
                    &source_path.as_ref(),
                    binary_result,
                    self.output_type,
                    self.skip_spirv,
                );

                // Extract uniforms from SPIR-V representation
                if !self.skip_spirv {
                    match wrapped_shader.reflect_uniforms() {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(crate::Error::WrappingShaderFailed {
                                reason: Box::new(err),
                                shader: wrapped_shader,
                            })
                        }
                    }
                }

                Ok(wrapped_shader)
            }
            Err(shaderc::Error::CompilationError(num_errors, errors)) => {
                if !self.skip_cargo {
                    eprintln!("{}", errors);
                }

                Err(Error::CompilationError(num_errors as usize, errors))
            }
            Err(error) => panic!(error.to_string()),
        }
    }

    pub fn wrap_shader_source(
        &mut self,
        source: &str,
        kind: shaderc::ShaderKind,
    ) -> Result<WrappedShader> {
        use sha2::Digest;

        let kind: ShaderKindInfo = kind.into();

        let mut source_path = base64::encode(sha2::Sha256::digest(source.as_bytes()));
        source_path.push('.');
        source_path.push_str(kind.extension);

        self.wrap_shader_id(source, source_path, kind)
    }

    pub fn wrap_shader(&mut self, source_path: impl AsRef<Path>) -> Result<WrappedShader> {
        // Get full path to shader
        let full_path = std::fs::canonicalize(&source_path)?;

        if !self.skip_cargo {
            // Notify cargo to rerun if the source changes
            println!("cargo:rerun-if-changed={}", full_path.display());
        }

        // Read GLSL source
        let source = std::fs::read_to_string(&full_path).unwrap();

        // Match shader type
        let kind = ShaderKindInfo::from_path(&source_path)
            .expect("no file extension on path, cannot determine shader type");

        self.wrap_shader_id(&source, source_path, kind)
    }

    pub fn wrap_program<'s>(
        &mut self,
        attached_shaders: &[&'s WrappedShader],
        program_name: &str,
    ) -> Result<WrappedProgram<'s>> {
        Ok(WrappedProgram::new(&program_name, attached_shaders))
    }

    pub fn wrap_uniforms<'p, 's>(
        &mut self,
        programs: &[&'p WrappedProgram<'s>],
        set_name: &str,
    ) -> Result<WrappedUniformSet<'p, 's>> {
        Ok(WrappedUniformSet::new(programs, set_name))
    }

    pub fn write_root_include<'a>(
        &self,
        dest: impl AsRef<Path>,
        items: &[&'a dyn WrappedItem],
    ) -> Result<()> {
        // Write master shaders.rs file
        let output_rs = File::create(dest.as_ref().join("shaders.rs"))?;
        let mut wr = BufWriter::new(output_rs);

        for item in items {
            item.write_root_include(&mut wr)?;
        }

        Ok(())
    }
}
