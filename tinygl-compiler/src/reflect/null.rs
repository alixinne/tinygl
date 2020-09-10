use super::*;

#[derive(Default)]
pub struct NullBackend {}

impl NullBackend {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GlslReflectBackend for NullBackend {
    fn reflect<'s>(&self, _input: &str) -> crate::Result<Vec<FoundUniform>> {
        Ok(vec![])
    }
}

#[cfg(feature = "spirv")]
impl SpirVReflectBackend for NullBackend {
    fn reflect<'s>(&self, _input: &rspirv::dr::Module) -> crate::Result<Vec<FoundUniform>> {
        Ok(vec![])
    }
}
