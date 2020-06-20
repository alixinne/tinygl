use crate::wrappers::UniformLocation;
use crate::Context;

use super::ProgramCommon;

pub trait ProgramCommonExt {
    unsafe fn use_program(&self, gl: &Context);
    fn get_uniform_location(&self, gl: &Context, name: &str) -> UniformLocation;
}

impl<T: ProgramCommon> ProgramCommonExt for T {
    unsafe fn use_program(&self, gl: &Context) {
        gl.use_program(Some(self.name()));
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_uniform_location(&self, gl: &Context, name: &str) -> UniformLocation {
        use std::ffi::CString;

        UniformLocation::new(self.name(), unsafe {
            let loc = gl.get_uniform_location(
                self.name(),
                CString::new(name)
                    .expect("invalid location identifier")
                    .as_ptr(),
            );

            if loc < 0 {
                None
            } else {
                Some(loc)
            }
        })
    }

    #[cfg(target_arch = "wasm32")]
    fn get_uniform_location(&self, gl: &Context, name: &str) -> UniformLocation {
        UniformLocation::new(gl.get_uniform_location(self.name(), name))
    }
}
