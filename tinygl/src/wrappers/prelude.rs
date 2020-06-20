//! Wrappers prelude for extension traits

pub use super::program::ProgramCommonExt;
#[cfg(all(not(target_arch = "wasm32"), feature = "opengl46"))]
pub use super::shader::BinaryShader;
pub use super::shader::ShaderCommon;
pub use super::shader::SourceShader;
