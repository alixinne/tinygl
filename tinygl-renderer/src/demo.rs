//! Demo type definition

use serde_derive::{Serialize, Deserialize};

mod compilable;
pub use compilable::*;

mod pass;
pub use pass::*;

mod context;
pub use context::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Demo {
    passes: Vec<Pass>
}

impl Demo {
    pub fn render(&self, context: &Context) {
        for pass in &self.passes {
            pass.render(context);
        }
    }
}

impl Compilable for Demo {
    fn compile(&mut self, context: &Context) -> Result<(), CompileError> {
        trace!("compiling demo");

        for pass in &mut self.passes {
            pass.compile(context)?;
        }

        Ok(())
    }
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            passes: vec![Pass::default()],
        }
    }
}
