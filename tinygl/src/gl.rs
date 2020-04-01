//! Exposed OpenGL bindings

#[cfg(not(target_arch = "wasm32"))]
pub use crate::glowx::gl::SHADER_BINARY_FORMAT_SPIR_V;
pub use ::glow::*;

#[derive(Debug)]
pub struct GlError(u32);

use std::fmt;
impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            NO_ERROR => write!(f, "GL_NO_ERROR"),
            INVALID_ENUM => write!(f, "GL_INVALID_ENUM"),
            INVALID_VALUE => write!(f, "GL_INVALID_VALUE"),
            INVALID_OPERATION => write!(f, "GL_INVALID_OPERATION"),
            INVALID_FRAMEBUFFER_OPERATION => write!(f, "GL_INVALID_FRAMEBUFFER_OPERATION"),
            OUT_OF_MEMORY => write!(f, "GL_OUT_OF_MEMORY"),
            STACK_UNDERFLOW => write!(f, "GL_STACK_UNDERFLOW"),
            STACK_OVERFLOW => write!(f, "GL_STACK_OVERFLOW"),
            x => write!(f, "GL_ERROR: {:x}", x),
        }
    }
}

impl std::error::Error for GlError {
}

pub trait CheckGlErrorExt {
    unsafe fn check_last_error(&self) -> Result<(), GlError>;
}

impl<T: glow::HasContext> CheckGlErrorExt for T {
    unsafe fn check_last_error(&self) -> Result<(), GlError> {
        match self.get_error() {
            NO_ERROR => Ok(()),
            other => Err(GlError(other)),
        }
    }
}
