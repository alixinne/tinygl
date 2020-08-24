use thiserror::Error;

#[derive(Debug)]
pub struct OpenGlErrorCode(pub u32);

use std::fmt;
impl fmt::Display for OpenGlErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::gl;

        match self.0 {
            gl::NO_ERROR => write!(f, "GL_NO_ERROR"),
            gl::INVALID_ENUM => write!(f, "GL_INVALID_ENUM"),
            gl::INVALID_VALUE => write!(f, "GL_INVALID_VALUE"),
            gl::INVALID_OPERATION => write!(f, "GL_INVALID_OPERATION"),
            gl::INVALID_FRAMEBUFFER_OPERATION => write!(f, "GL_INVALID_FRAMEBUFFER_OPERATION"),
            gl::OUT_OF_MEMORY => write!(f, "GL_OUT_OF_MEMORY"),
            #[cfg(not(target_arch = "wasm32"))]
            gl::STACK_UNDERFLOW => write!(f, "GL_STACK_UNDERFLOW"),
            #[cfg(not(target_arch = "wasm32"))]
            gl::STACK_OVERFLOW => write!(f, "GL_STACK_OVERFLOW"),
            x => write!(f, "GL_ERROR: {:x}", x),
        }
    }
}

impl std::error::Error for OpenGlErrorCode {}

#[derive(Error, Debug)]
pub enum Error {
    #[error("OpenGL error: {0}")]
    OpenGlError(#[from] OpenGlErrorCode),
    #[error("failed to create buffer: {0}")]
    BufferCreationFailed(#[source] OpenGlErrorCode),
    #[error("failed to create framebuffer: {0}")]
    FramebufferCreationFailed(#[source] OpenGlErrorCode),
    #[error("failed to create renderbuffer: {0}")]
    RenderbufferCreationFailed(#[source] OpenGlErrorCode),
    #[error("failed to create shader: {0}")]
    ShaderCreationFailed(#[source] OpenGlErrorCode),
    #[error("failed to compile shader: {0}")]
    ShaderCompilationFailed(String),
    #[error("failed to create texture: {0}")]
    TextureCreationFailed(#[source] OpenGlErrorCode),
    #[error("failed to create program: {0}")]
    ProgramCreationFailed(#[source] OpenGlErrorCode),
    #[error("failed to link program: {0}")]
    ProgramLinkFailed(String),
    #[error("query creation failed: {0}")]
    QueryCreationFailed(#[source] OpenGlErrorCode),
    #[error("vertex array creation failed: {0}")]
    VertexArrayCreationFailed(#[source] OpenGlErrorCode),
}

pub type Result<T> = std::result::Result<T, Error>;
