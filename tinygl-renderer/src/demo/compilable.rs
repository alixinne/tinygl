//! Definition of the Compilable trait

use failure::Fail;

use super::Context;

#[derive(Fail, Debug)]
pub enum CompileError {
    #[fail(display = "compilation error: {}", log)]
    CompilationError { log: String },
    #[fail(display = "link error: {}", log)]
    LinkError { log: String },
}

pub trait Compilable {
    fn compile(&mut self, context: &Context) -> Result<(), CompileError>;
}
