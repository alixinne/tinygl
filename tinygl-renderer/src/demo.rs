//! Demo type definition

use serde_derive::{Deserialize, Serialize};

mod compilable;
pub use compilable::*;

mod context;
pub use context::*;

mod gl_droppable;
pub use gl_droppable::*;

mod pass;
pub use pass::*;

mod step_program;
pub use step_program::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Demo {
    pub passes: Vec<Pass>,
}

impl Demo {
    pub fn sample() -> Self {
        Self {
            passes: vec![PassBuilder::sample("image").build()],
        }
    }

    pub fn prepare_render(&mut self, context: &Context) -> Result<(), String> {
        let len = self.passes.len();

        for (i, pass) in self.passes.iter_mut().enumerate() {
            pass.prepare_render(context, i == len - 1)?;
        }

        Ok(())
    }

    pub fn render(&self, context: &Context) {
        for pass in &self.passes {
            pass.render(context, self);
        }
    }

    pub fn get_texture(
        &self,
        source: &SamplerSource,
    ) -> Option<<glow::Context as glow::HasContext>::Texture> {
        match source {
            SamplerSource::BufferId(id) if *id < self.passes.len() => {
                self.passes[*id].get_render_texture()
            }
            SamplerSource::BufferName(name) => {
                let target = name.to_lowercase();
                self.passes
                    .iter()
                    .find(|pass| pass.name.to_lowercase() == target)
                    .and_then(|pass| pass.get_render_texture())
            }
            _ => None,
        }
    }
}

impl Compilable for Demo {
    fn compile(&mut self, context: &Context) -> Result<(), CompileError> {
        #[cfg(not(target_arch = "wasm32"))]
        trace!("compiling demo: {:?}", self);

        for pass in &mut self.passes {
            pass.compile(context)?;
        }

        Ok(())
    }
}

impl Default for Demo {
    fn default() -> Self {
        Self { passes: vec![] }
    }
}

impl GlDroppable for Demo {
    fn drop(&mut self, gl: &glow::Context) {
        for pass in &mut self.passes {
            pass.drop(gl);
        }
    }
}
