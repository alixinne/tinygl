//! Definition of the Compilable trait

use failure::Fail;

use super::Context;

#[derive(Fail, Debug)]
pub enum CompileError {
    #[fail(display = "compilation error: {}", log)]
    CompilationError { log: String },
    #[fail(display = "link error: {}", log)]
    LinkError { log: String },
    #[fail(display = "parse error: {}", error)]
    ParseError { error: String },
}

pub trait Compilable {
    fn compile(&mut self, context: &Context) -> Result<(), CompileError>;
}

impl From<glsl::parser::ParseError> for CompileError {
    fn from(error: glsl::parser::ParseError) -> Self {
        CompileError::ParseError { error: error.info }
    }
}
