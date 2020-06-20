mod error;
pub use error::*;

pub mod gl;
pub use gl::Context;

pub mod wrappers;

pub mod prelude {
    pub use super::wrappers::prelude::*;
}

pub fn opengl_version() -> (u8, u8) {
    if cfg!(target_arch = "wasm32") {
        (2, 0)
    } else if cfg!(feature = "opengl46") {
        (4, 6)
    } else if cfg!(feature = "opengl45") {
        (4, 5)
    } else if cfg!(feature = "opengl44") {
        (4, 4)
    } else {
        unreachable!()
    }
}
