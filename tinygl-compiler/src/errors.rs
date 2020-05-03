use super::TargetType;
use std::error;

use crate::compiler::WrappedShader;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("compilation error: {1}")]
    CompilationError(usize, String),
    #[error("invalid target type for current arch: {0:?}")]
    InvalidTargetType(TargetType),
    #[error("cannot skip SPIR-V generation when the target is explicitely SPIR-V")]
    InvalidSkipSpirV,
    #[error("spirv_cross error: {0:?}")]
    SpirVCrossError(spirv_cross::ErrorCode),
    #[error(
        "shader {0} was not wrapped before building the program, call Compiler::wrap_shader first"
    )]
    UnwrappedShader(String),
    #[error("program {0} was not wrapped before building the uniform set, call Compiler::wrap_program first")]
    UnwrappedProgram(String),
    #[error("failed to wrap shader program: {reason}")]
    WrappingShaderFailed {
        reason: Box<dyn error::Error>,
        shader: WrappedShader,
    },
}

impl From<spirv_cross::ErrorCode> for Error {
    fn from(error: spirv_cross::ErrorCode) -> Self {
        Self::SpirVCrossError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
