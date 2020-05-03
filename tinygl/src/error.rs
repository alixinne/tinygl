use thiserror::Error;

#[derive(Debug)]
pub struct OpenGlErrorCode(pub u32);

use std::fmt;
impl fmt::Display for OpenGlErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ::glow::*;

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

impl std::error::Error for OpenGlErrorCode {}

#[derive(Error, Debug)]
pub enum Error {
    #[error("OpenGL error: {0}")]
    OpenGlError(#[from] OpenGlErrorCode),
    #[error("failed to create buffer: {0}")]
    BufferCreationFailed(String),
    #[error("failed to create framebuffer: {0}")]
    FramebufferCreationFailed(String),
    #[error("failed to create renderbuffer: {0}")]
    RenderbufferCreationFailed(String),
    #[error("failed to create shader: {0}")]
    ShaderCreationFailed(String),
    #[error("failed to compile shader: {0}")]
    ShaderCompilationFailed(String),
    #[error("failed to create texture: {0}")]
    TextureCreationFailed(String),
    #[error("failed to create program: {0}")]
    ProgramCreationFailed(String),
    #[error("failed to link program: {0}")]
    ProgramLinkFailed(String),
}

pub type Result<T> = std::result::Result<T, Error>;
