//! Definition of the CompileError type

use failure::Fail;

#[derive(Fail, Debug)]
pub enum CompileError {
    #[fail(display = "compilation error: {}", log)]
    CompilationError { log: String },
    #[fail(display = "link error: {}", log)]
    LinkError { log: String },
    #[fail(display = "parse error: {}", error)]
    ParseError { error: String },
}

impl From<glsl::parser::ParseError> for CompileError {
    fn from(error: glsl::parser::ParseError) -> Self {
        CompileError::ParseError { error: error.info }
    }
}
