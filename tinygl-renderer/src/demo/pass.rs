//! Pass type definition

use serde_derive::{Deserialize, Serialize};

use glow::HasContext;

use super::{Compilable, CompileError, Context};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Pass {
    fragment: String,
    #[serde(skip)]
    program: Option<<glow::Context as glow::HasContext>::Program>,
}

impl Pass {
    pub fn render(&self, context: &Context) {
        unsafe {
            context.gl.use_program(self.program);
            context.gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}

impl Compilable for Pass {
    fn compile(&mut self, context: &Context) -> Result<(), CompileError> {
        trace!("compiling pass {:?}", self);
        self.program = Some(context.compile_fragment(&self.fragment)?);

        Ok(())
    }
}

impl Default for Pass {
    fn default() -> Self {
        Self {
            fragment: r#"precision mediump float;
            in vec2 texCoords;
            out vec4 color;
            void main() {
                color = vec4(texCoords.xy, 0.5, 1.0);
            }"#
            .to_owned(),
            program: None,
        }
    }
}
