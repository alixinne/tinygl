mod found_uniform;
pub use found_uniform::*;

mod null;
pub use null::*;

#[cfg(feature = "spirv")]
mod spirv;
#[cfg(feature = "spirv")]
pub use spirv::*;

use crate::model::{ObjectInfo, ShaderObject};

pub trait GlslReflectBackend {
    fn reflect<'s>(&self, input: &str) -> crate::Result<Vec<FoundUniform>>;
}

#[cfg(feature = "spirv")]
pub trait SpirVReflectBackend {
    fn reflect<'s>(&self, input: &rspirv::dr::Module) -> crate::Result<Vec<FoundUniform>>;
}

#[derive(Debug)]
pub struct ReflectedObject<T> {
    object: T,
    uniforms: Vec<FoundUniform>,
}

impl<T> ReflectedObject<T> {
    pub fn new(object: T, uniforms: Vec<FoundUniform>) -> Self {
        Self { object, uniforms }
    }

    pub fn object(&self) -> &T {
        &self.object
    }

    pub fn uniforms(&self) -> &[FoundUniform] {
        &self.uniforms
    }
}

impl<T> ReflectedObject<ShaderObject<T>> {
    pub fn info(&self) -> &ObjectInfo {
        &self.object.info()
    }
}

impl<T> std::ops::Deref for ReflectedObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
