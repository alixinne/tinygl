use std::path::{Path, PathBuf};

use crate::{
    reflect::{GlslReflectBackend, ReflectedObject},
    Result, ShaderKind,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourcePath {
    File(PathBuf),
    Generated(String),
}

impl SourcePath {
    pub fn to_path(&self) -> PathBuf {
        match self {
            Self::File(p) => p.clone(),
            Self::Generated(s) => PathBuf::from(s),
        }
    }
}

impl std::fmt::Display for SourcePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(path) => write!(f, "{}", path.display()),
            Self::Generated(s) => write!(f, "{}", s),
        }
    }
}

/// Metadata for a shader object
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectInfo {
    /// Source path for this shader
    pub source_path: SourcePath,
    /// Shader stage
    pub kind: ShaderKind,
}

/// A shader object manipulated by tinygl
#[derive(Debug)]
pub struct ShaderObject<T> {
    /// Shader module in its current representation
    module: T,
    /// Metadata for the module
    info: ObjectInfo,
}

impl<T> ShaderObject<T> {
    pub fn info(&self) -> &ObjectInfo {
        &self.info
    }

    pub fn track_cargo(self) -> Self {
        if let SourcePath::File(p) = &self.info.source_path {
            // Notify cargo to rerun if the source changes
            println!("cargo:rerun-if-changed={}", p.display());
        }

        self
    }
}

#[cfg(feature = "spirv")]
pub trait GlslCompiler {
    fn compile_module(
        &mut self,
        source: &str,
        kind: ShaderKind,
        source_path: &str,
    ) -> Result<SpirVModule<'static>>;
}

pub trait GlslPreprocessor {
    fn preprocess_module(&mut self, source: &str, source_path: &str)
        -> Result<GlslModule<'static>>;
}

fn bytes_to_id(src: &[u8], kind: ShaderKind) -> ObjectInfo {
    use sha2::Digest;

    let mut source_path = base64::encode(sha2::Sha256::digest(src));
    source_path.push('.');
    source_path.push_str(kind.extension());

    ObjectInfo {
        source_path: SourcePath::Generated(source_path.into()),
        kind: kind.into(),
    }
}

impl<T> std::ops::Deref for ShaderObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.module
    }
}

impl<T> std::ops::DerefMut for ShaderObject<T> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        &mut self.module
    }
}

pub trait AsOutputFormat {
    fn as_source(&self) -> Option<&GlslModule> {
        None
    }
    #[cfg(feature = "spirv")]
    fn as_spirv(&self) -> Option<&SpirVModule> {
        None
    }
}

mod glsl;
pub use self::glsl::GlslModule;
pub type GlslObject<'s> = ShaderObject<GlslModule<'s>>;

#[cfg(feature = "spirv")]
mod spirv;
#[cfg(feature = "spirv")]
pub use spirv::{GlslWithSpirVModule, SpirVModule};
#[cfg(feature = "spirv")]
pub type SpirVObject<'s> = ShaderObject<SpirVModule<'s>>;
