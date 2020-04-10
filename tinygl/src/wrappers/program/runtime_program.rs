use crate::context::HasContext;
use crate::wrappers::ShaderCommon;
use crate::Context;

use super::ProgramCommon;

pub struct RuntimeProgram {
    name: <glow::Context as HasContext>::Program,
}

impl RuntimeProgram {
    pub fn into_inner(self) -> <glow::Context as HasContext>::Program {
        self.name
    }
}

impl ProgramCommon for RuntimeProgram {
    fn name(&self) -> <glow::Context as HasContext>::Program {
        self.name
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

    pub fn build(self) -> Result<RuntimeProgram, String> {
        unsafe {
            let program_name = self.gl.create_program()?;

            // Attach shaders
            for shader in &self.shaders {
                self.gl.attach_shader(program_name, shader.name());
            }

            // Link program
            self.gl.link_program(program_name);

            // Detach shaders
            for shader in &self.shaders {
                self.gl.detach_shader(program_name, shader.name());
            }

            // Check link status
            if !self.gl.get_program_link_status(program_name) {
                let error = self.gl.get_program_info_log(program_name);
                self.gl.delete_program(program_name);
                return Err(error);
            }

            Ok(RuntimeProgram { name: program_name })
        }
    }
}
