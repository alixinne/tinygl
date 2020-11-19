use crate::{model::ShaderObject, reflect::ReflectedObject, Error, Result};

#[cfg(feature = "shaderc")]
mod shaderc_compiler;

mod target_type;
pub use target_type::{GlslVersion, TargetType};

mod uniform_set;
pub use uniform_set::*;

mod wrapped_shader;
pub use wrapped_shader::*;

mod wrapped_program;
pub use wrapped_program::*;

pub type IncludeCallback = Box<dyn FnMut(&std::path::Path) -> ()>;

#[cfg_attr(not(feature = "backend-shaderc"), allow(dead_code))]
pub struct Compiler {
    pub(crate) skip_cargo: bool,
    output_type: TargetType,
    include_callback: Option<std::rc::Rc<std::cell::RefCell<IncludeCallback>>>,
}

impl Compiler {
    pub fn new(skip_cargo: bool, output_type: Option<TargetType>) -> Result<Self> {
        Self::with_include_callback(skip_cargo, output_type, None)
    }

    pub fn with_include_callback(
        skip_cargo: bool,
        output_type: Option<TargetType>,
        include_callback: Option<IncludeCallback>,
    ) -> Result<Self> {
        // Are we building for WASM?
        let is_wasm = std::env::var("TARGET")
            .map(|v| v.starts_with("wasm32"))
            .unwrap_or(false);

        // If building for WASM, force source usage unless a specific version is required
        let output_type = match output_type {
            None => {
                if is_wasm {
                    TargetType::Glsl(GlslVersion::V3_00Es)
                } else {
                    TargetType::SpirV
                }
            }
            Some(specific) => match specific {
                TargetType::SpirV => {
                    if is_wasm {
                        return Err(Error::InvalidTargetType(specific));
                    } else {
                        TargetType::SpirV
                    }
                }
                TargetType::Glsl(version) => {
                    if is_wasm {
                        match version {
                            GlslVersion::V3_00Es | GlslVersion::V1_00Es => {
                                TargetType::Glsl(version)
                            }
                            _ => {
                                return Err(Error::InvalidTargetType(specific));
                            }
                        }
                    } else {
                        TargetType::Glsl(version)
                    }
                }
            },
        };

        Ok(Self {
            skip_cargo,
            output_type,
            include_callback: include_callback
                .map(|cb| std::rc::Rc::new(std::cell::RefCell::new(cb))),
        })
    }

    #[cfg(feature = "shaderc")]
    pub fn with_shaderc(self) -> shaderc_compiler::CompilerWithShaderc {
        shaderc_compiler::CompilerWithShaderc::new(self)
    }

    pub fn output_type(&self) -> TargetType {
        self.output_type
    }

    pub fn wrap_shader<'s, T>(
        &mut self,
        shader: ReflectedObject<ShaderObject<T>>,
        prefer_spirv: bool,
    ) -> Result<WrappedShader<T>> {
        Ok(WrappedShader::new(shader, prefer_spirv))
    }

    pub fn wrap_program<'s>(
        &mut self,
        attached_shaders: &[&'s dyn WrappedShaderDetails],
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
}
