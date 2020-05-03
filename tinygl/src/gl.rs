//! Exposed OpenGL bindings

#[cfg(not(target_arch = "wasm32"))]
pub use crate::glowx::gl::SHADER_BINARY_FORMAT_SPIR_V;
pub use ::glow::*;

pub trait CheckGlErrorExt {
    /// Check the state of the last OpenGL operation
    ///
    /// # Safety
    ///
    /// This call returns the value of glGetLastError(), it is up to the library user to call this
    /// at the right location.
    unsafe fn check_last_error(&self) -> crate::Result<()>;
}

impl<T: glow::HasContext> CheckGlErrorExt for T {
    unsafe fn check_last_error(&self) -> crate::Result<()> {
        match self.get_error() {
            glow::NO_ERROR => Ok(()),
            other => Err(crate::Error::OpenGlError(crate::OpenGlErrorCode(other))),
        }
    }
}
