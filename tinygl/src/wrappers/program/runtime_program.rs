use crate::wrappers::{GlDrop, ShaderCommon};
use crate::{Context, OpenGlErrorCode};

use super::ProgramCommon;

pub struct RuntimeProgram {
    name: crate::gl::Program,
}

impl RuntimeProgram {
    pub fn into_inner(self) -> crate::gl::Program {
        self.name
    }
}

impl ProgramCommon for RuntimeProgram {
    #[cfg(not(target_arch = "wasm32"))]
    fn name(&self) -> crate::gl::ProgramName {
        self.name
    }

    #[cfg(target_arch = "wasm32")]
    fn name(&self) -> crate::gl::ProgramName {
        &self.name
    }
}

pub struct RuntimeProgramBuilder<'a> {
    gl: &'a Context,
    shaders: Vec<&'a dyn ShaderCommon>,
}

impl<'a> RuntimeProgramBuilder<'a> {
    pub fn new(gl: &'a Context) -> Self {
        Self {
            gl,
            shaders: Vec::with_capacity(2),
        }
    }

    pub fn shader(mut self, shader: &'a dyn ShaderCommon) -> Self {
        self.shaders.push(shader);
        self
    }

    pub fn build(self) -> crate::Result<RuntimeProgram> {
        unsafe {
            let mut program = RuntimeProgram {
                name: self.gl.create_program().ok_or_else(|| {
                    crate::Error::ProgramCreationFailed(OpenGlErrorCode(self.gl.get_error()))
                })?,
            };

            // Attach shaders
            for shader in &self.shaders {
                self.gl.attach_shader(program.name(), shader.name());
            }

            // Link program
            self.gl.link_program(program.name());

            // Detach shaders
            for shader in &self.shaders {
                self.gl.detach_shader(program.name(), shader.name());
            }

            // Check link status
            if !self.gl.get_program_link_status(program.name()) {
                let error = self.gl.get_program_info_log(program.name());
                program.drop(self.gl);
                return Err(crate::Error::ProgramLinkFailed(
                    error.unwrap_or_else(String::new),
                ));
            }

            Ok(program)
        }
    }
}

impl GlDrop for RuntimeProgram {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn drop(&mut self, gl: &Context) {
        gl.delete_program(self.name);
    }

    #[cfg(target_arch = "wasm32")]
    unsafe fn drop(&mut self, gl: &Context) {
        gl.delete_program(Some(&self.name));
    }
}
