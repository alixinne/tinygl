use crate::context::HasContext;
use crate::Context;

use crate::wrappers::UniformLocation;

use super::ProgramCommon;

pub trait ProgramCommonExt {
    unsafe fn use_program(&self, gl: &Context);
    fn get_uniform_location(&self, gl: &Context, name: &str) -> UniformLocation;
}

impl<T: ProgramCommon> ProgramCommonExt for T {
    unsafe fn use_program(&self, gl: &Context) {
        gl.use_program(Some(self.name()));
    }

    fn get_uniform_location(&self, gl: &Context, name: &str) -> UniformLocation {
        UniformLocation::new(self.name(), unsafe {
            gl.get_uniform_location(self.name(), name)
        })
    }
}
