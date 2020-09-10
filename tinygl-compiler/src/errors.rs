use super::TargetType;

use crate::GlslVersion;

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
    #[cfg(feature = "rspirv")]
    #[error("error parsing SPIR-V module: {0}")]
    SpirVParseEreror(#[from] rspirv::binary::ParseState),
    #[cfg(feature = "spirv_cross")]
    #[error("spirv_cross error: {0:?}")]
    SpirVCrossError(spirv_cross::ErrorCode),
    #[error(
        "shader {0} was not wrapped before building the program, call Compiler::wrap_shader first"
    )]
    UnwrappedShader(String),
    #[error("program {0} was not wrapped before building the uniform set, call Compiler::wrap_program first")]
    UnwrappedProgram(String),
    #[error("failed to wrap shader program: {0}")]
    WrappingShaderFailed(#[from] Box<Error>),
    #[error("transpiling to {0} is not supported, please enable the transpile feature")]
    TranspilingNotSupported(GlslVersion),
    #[error("error parsing GLSL code: {0}")]
    GlslParseError(#[from] glsl::parser::ParseError),
    #[error("could not determine type of shader object based on extension")]
    CouldNotDetermineType,
    #[error("a SPIR-V object is required for this operation")]
    SpirVObjectRequired,
    #[error("a source object is required for this operation")]
    SourceObjectRequired,
    #[error("SPIR-V is not supported, enable the spirv feature")]
    SpirVDisabled,
}

#[cfg(feature = "spirv_cross")]
impl From<spirv_cross::ErrorCode> for Error {
    fn from(error: spirv_cross::ErrorCode) -> Self {
        Self::SpirVCrossError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
