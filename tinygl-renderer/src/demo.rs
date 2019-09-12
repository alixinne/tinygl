//! Demo type definition

use serde_derive::{Deserialize, Serialize};

mod compile_error;
pub use compile_error::*;

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
    pub common_code: String,
    pub passes: Vec<Pass>,
}

pub enum RenderMode {
    Full {
        target: Option<<glow::Context as glow::HasContext>::Framebuffer>,
    },
    Blit {
        target: Option<<glow::Context as glow::HasContext>::Framebuffer>,
    },
}

impl Demo {
    pub fn sample() -> Self {
        Self {
            passes: vec![PassBuilder::sample("image").build()],
            ..Default::default()
        }
    }

    pub fn compile(&mut self, context: &Context) -> Result<(), CompileError> {
        #[cfg(not(target_arch = "wasm32"))]
        trace!("compiling demo: {:?}", self);

        let common_code = &self.common_code;
        for pass in &mut self.passes {
            pass.compile(context, common_code)?;
        }

        Ok(())
    }

    pub fn prepare_render(&mut self, context: &Context) -> Result<(), String> {
        for pass in &mut self.passes {
            pass.prepare_render(context)?;
        }

        Ok(())
    }

    fn render_full_to_target(
        &self,
        context: &Context,
        target: Option<Option<<glow::Context as glow::HasContext>::Framebuffer>>,
    ) {
        let len = self.passes.len();

        for (i, pass) in self.passes.iter().enumerate() {
            pass.render(context, self, if i == (len - 1) { target } else { None });
        }
    }

    pub fn render(&self, context: &Context, mode: RenderMode) {
        match mode {
            RenderMode::Full { target } => {
                self.render_full_to_target(context, Some(target));
            }
            RenderMode::Blit { target } => {
                self.passes.last().map(|pass| pass.blit(context, target));
            }
        }
    }

    pub fn render_intermediate(
        &self,
        context: &Context,
        mode: RenderMode,
        predicate: impl Fn((usize, &Pass)) -> bool,
    ) {
        // Full render if requested, but without a target
        if let RenderMode::Full { .. } = mode {
            self.render_full_to_target(context, None);
        }

        // Then blit the requested result
        let target = match mode {
            RenderMode::Full { target } => target,
            RenderMode::Blit { target } => target,
        };

        for (i, pass) in self.passes.iter().enumerate() {
            if predicate((i, pass)) {
                pass.blit(context, target);
                break;
            }
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

impl Default for Demo {
    fn default() -> Self {
        Self {
            passes: vec![],
            common_code: "precision mediump float;\nin vec2 texCoords;\nout vec4 color;".to_owned(),
        }
    }
}

impl GlDroppable for Demo {
    fn drop(&mut self, gl: &glow::Context) {
        for pass in &mut self.passes {
            pass.drop(gl);
        }
    }
}
