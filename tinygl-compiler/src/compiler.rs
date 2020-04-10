use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use crate::{shader_kind::ShaderKindInfo, Error, Result};

mod target_type;
pub use target_type::TargetType;

mod uniform_set;
pub use uniform_set::*;

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
                } else {
                    if self.skip_spirv {
                        TargetType::Glsl(spirv_cross::glsl::Version::V4_60)
                    } else {
                        TargetType::SpirV
                    }
                }
            }
            TargetType::SpirV => {
                if is_wasm {
                    return Err(Error::InvalidTargetType(self.output_type));
                } else {
                    if self.skip_spirv {
                        return Err(Error::InvalidSkipSpirV);
                    } else {
                        TargetType::SpirV
                    }
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
            wrapped_shaders: HashMap::new(),
            wrapped_programs: HashMap::new(),
            wrapped_uniform_sets: HashMap::new(),
            skip_spirv: self.skip_spirv,
            output_type,
        })
    }
}

pub struct Compiler {
    compiler: shaderc::Compiler,
    wrapped_shaders: HashMap<PathBuf, WrappedShader>,
    wrapped_programs: HashMap<String, WrappedProgram>,
    wrapped_uniform_sets: HashMap<String, WrappedUniformSet>,
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
    ) -> Result<WrappedShaderRef<'_>> {
        let wrapped_shader_entry = {
            // Set callback
            let mut options = shaderc::CompileOptions::new().unwrap();

            // Shader name
            let shader = source_path.as_ref().to_string_lossy();

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
                self.compiler
                    .preprocess(&source, &shader, "main", Some(&options))
            } else {
                // Compile into SPIR-V
                self.compiler.compile_into_spirv(
                    &source,
                    kind.shaderc_kind,
                    &shader,
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
                        &shader,
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
        };

        match wrapped_shader_entry {
            Ok(wrapped_shader) => {
                // Add to list of files to include
                self.wrapped_shaders
                    .insert(source_path.as_ref().to_owned(), wrapped_shader);
                Ok(WrappedShaderRef::new(
                    self.wrapped_shaders.get(source_path.as_ref()).unwrap(),
                ))
            }
            Err(err) => Err(err),
        }
    }

    pub fn wrap_shader_source(
        &mut self,
        source: &str,
        kind: shaderc::ShaderKind,
    ) -> Result<WrappedShaderRef<'_>> {
        use sha2::Digest;

        let kind: ShaderKindInfo = kind.into();

        let mut source_path = base64::encode(sha2::Sha256::digest(source.as_bytes()));
        source_path.push('.');
        source_path.push_str(kind.extension);

        self.wrap_shader_id(source, source_path, kind)
    }

    pub fn wrap_shader(&mut self, source_path: impl AsRef<Path>) -> Result<WrappedShaderRef<'_>> {
        // Get full path to shader
        let source_path = std::fs::canonicalize(source_path)?;

        if !self.skip_cargo {
            // Notify cargo to rerun if the source changes
            println!("cargo:rerun-if-changed={}", source_path.display());
        }

        // Read GLSL source
        let source = std::fs::read_to_string(&source_path).unwrap();

        // Match shader type
        let kind = ShaderKindInfo::from_path(&source_path)
            .expect("no file extension on path, cannot determine shader type");

        self.wrap_shader_id(&source, source_path, kind)
    }

    pub fn wrap_program(
        &mut self,
        attached_shaders: &[&dyn WrappedShaderId],
        program_name: &str,
    ) -> Result<WrappedProgramRef<'_>> {
        let wrapped_program = WrappedProgram::new(&program_name, attached_shaders);

        // Resolve uniforms
        let uniform_data = wrapped_program.resolve_shaders(&self.wrapped_shaders)?;

        // Add to list of wrapped programs
        let id = wrapped_program.id().to_owned();
        self.wrapped_programs.insert(id.clone(), wrapped_program);

        Ok(WrappedProgramRef::new(
            self.wrapped_programs.get(&id).unwrap(),
            uniform_data,
        ))
    }

    pub fn wrap_uniforms(
        &mut self,
        programs: &[&dyn WrappedProgramId],
        set_name: &str,
    ) -> Result<WrappedUniformSetRef<'_>> {
        let uniform_set = WrappedUniformSet::new(&set_name);

        // Resolve programs
        let uniform_data = uniform_set.resolve_programs(
            programs,
            &self.wrapped_programs,
            &self.wrapped_shaders,
        )?;

        // Add to list of wrapped sets
        let id = uniform_set.id().to_owned();
        self.wrapped_uniform_sets.insert(id.clone(), uniform_set);

        Ok(WrappedUniformSetRef::new(
            self.wrapped_uniform_sets.get(&id).unwrap(),
            uniform_data,
        ))
    }

    pub fn write_root_include(&self, dest: impl AsRef<Path>) -> Result<()> {
        // Write master shaders.rs file
        let output_rs = File::create(dest.as_ref().join("shaders.rs"))?;
        let mut wr = BufWriter::new(output_rs);

        // Include shaders
        for (_source_path, shader) in self.wrapped_shaders.iter() {
            shader.write_root_include(&mut wr)?;
        }

        // Include programs
        for (_program_name, program) in self.wrapped_programs.iter() {
            program.write_root_include(&mut wr)?;
        }

        // Write program wrappers
        for (_uniform_set_name, uniform_set) in self.wrapped_uniform_sets.iter() {
            uniform_set.write_root_include(&mut wr)?;
        }

        Ok(())
    }
}
