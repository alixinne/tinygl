use crate::context::HasContext;
use crate::Context;

pub trait ProgramCommon {
    fn name(&self) -> <glow::Context as HasContext>::Program;

    fn use_program(&self, gl: &Context) {
        unsafe { gl.use_program(Some(self.name())) };
    }
}
