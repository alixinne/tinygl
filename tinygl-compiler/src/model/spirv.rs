use super::*;

use crate::reflect::SpirVReflectBackend;

use std::borrow::Cow;

/// A SPIR-V object
pub struct SpirVModule<'s> {
    /// Raw SPIR-V data
    binary: Cow<'s, [u32]>,
    /// Parsed SPIR-V representation
    module: rspirv::dr::Module,
}

fn bytes_to_words(binary: &[u8]) -> Vec<u32> {
    unsafe {
        let s = &binary[..];
        std::slice::from_raw_parts(s.as_ptr() as *mut u32, s.len() / std::mem::size_of::<u32>())
    }
    .to_vec()
}

impl<'s> SpirVModule<'s> {
    pub fn as_bytes_u8(&self) -> &[u8] {
        unsafe {
            let s = &self.binary[..];
            std::slice::from_raw_parts(s.as_ptr() as *const _, s.len() * std::mem::size_of::<u32>())
        }
    }

    pub fn as_bytes(&self) -> &[u32] {
        &self.binary
    }

    /// Load a SPIR-V object from a file
    ///
    /// # Parameters
    ///
    /// * `p`: path to load the object from
    pub(crate) fn from_path(p: impl AsRef<Path>) -> Result<Self> {
        let p = p.as_ref();
        Self::from_words(bytes_to_words(&std::fs::read(p)?))
    }

    pub(crate) fn from_slice(binary: &[u8]) -> Result<SpirVModule<'static>> {
        Ok(SpirVModule::from_words(bytes_to_words(binary))?)
    }

    /// Load a SPIR-V from its binary representation
    ///
    /// # Parameters
    ///
    /// * `binary`: bytes representing the object
    pub(crate) fn from_words(binary: Vec<u32>) -> Result<SpirVModule<'static>> {
        let mut loader = rspirv::dr::Loader::new();
        rspirv::binary::parse_words(&binary, &mut loader)?;

        Ok(SpirVModule {
            binary: Cow::Owned(binary),
            module: loader.module(),
        })
    }

    /// Load a SPIR-V from its binary representation
    ///
    /// # Parameters
    ///
    /// * `binary`: bytes representing the object
    #[allow(dead_code)]
    pub(crate) fn from_word_slice(binary: &'s [u32]) -> Result<Self> {
        let mut loader = rspirv::dr::Loader::new();
        rspirv::binary::parse_words(binary, &mut loader)?;

        Ok(Self {
            binary: Cow::Borrowed(binary),
            module: loader.module(),
        })
    }

    /// Get the module parsed using rspirv
    pub fn module(&self) -> &rspirv::dr::Module {
        &self.module
    }
}

impl<'s> ShaderObject<SpirVModule<'s>> {
    /// Load a GLSL shader from a file
    ///
    /// # Parameters
    ///
    /// * `p`: path to load the shader from
    /// * `kind`: shader stage
    pub fn from_path(p: impl AsRef<Path>, kind: Option<ShaderKind>) -> Result<Self> {
        let p = std::fs::canonicalize(p.as_ref())?;

        let kind = match kind {
            Some(kind) => kind.into(),
            None => ShaderKind::from_path(&p).ok_or(crate::Error::CouldNotDetermineType)?,
        };

        Ok(Self {
            module: SpirVModule::from_path(&p)?,
            info: ObjectInfo {
                kind,
                source_path: SourcePath::File(p),
            },
        })
    }

    /// Load a GLSL shader from source
    ///
    /// # Parameters
    ///
    /// * `source`: source to load the shader from
    /// * `kind`: shader stage
    pub fn from_bytes(
        binary: &[u8],
        kind: ShaderKind,
    ) -> Result<ShaderObject<SpirVModule<'static>>> {
        let info = bytes_to_id(binary, kind);

        Ok(ShaderObject {
            module: SpirVModule::from_slice(binary)?,
            info,
        })
    }

    pub fn reflect(self, reflector: &dyn SpirVReflectBackend) -> Result<ReflectedObject<Self>> {
        let uniforms = reflector.reflect(&self.module.module)?;
        Ok(ReflectedObject::new(self, uniforms))
    }

    #[cfg(feature = "transpile")]
    pub fn transpile(
        self,
        version: crate::GlslVersion,
    ) -> crate::Result<GlslWithSpirVModule<'static, 's>> {
        // Use spirv_cross to write valid code
        let module = spirv_cross::spirv::Module::from_words(self.module.as_bytes());
        let mut ast = spirv_cross::spirv::Ast::<spirv_cross::glsl::Target>::parse(&module)?;

        // Target the right GLSL version
        ast.set_compiler_options(&spirv_cross::glsl::CompilerOptions {
            version: version.into(),
            ..Default::default()
        })
        .unwrap();

        Ok(GlslWithSpirVModule {
            glsl: glsl::GlslModule::from_string(ast.compile()?)?,
            spirv: self.module,
        })
    }
}

impl AsOutputFormat for SpirVModule<'_> {
    fn as_spirv(&self) -> Option<&SpirVModule> {
        Some(self)
    }
}

pub struct GlslWithSpirVModule<'s, 't> {
    glsl: glsl::GlslModule<'s>,
    spirv: SpirVModule<'t>,
}

// TODO: Implement traits to delegate functionality
impl<'s, 't> GlslWithSpirVModule<'s, 't> {
    pub fn new(glsl: glsl::GlslModule<'s>, spirv: SpirVModule<'t>) -> Self {
        Self { glsl, spirv }
    }
}

impl<'s, 't> ShaderObject<GlslWithSpirVModule<'s, 't>> {
    pub fn reflect_glsl(self, reflector: &dyn GlslReflectBackend) -> Result<ReflectedObject<Self>> {
        let uniforms = reflector.reflect(&self.glsl.as_str())?;
        Ok(ReflectedObject::new(self, uniforms))
    }

    pub fn reflect_spirv(
        self,
        reflector: &dyn SpirVReflectBackend,
    ) -> Result<ReflectedObject<Self>> {
        let uniforms = reflector.reflect(&self.spirv.module)?;
        Ok(ReflectedObject::new(self, uniforms))
    }
}

impl AsOutputFormat for GlslWithSpirVModule<'_, '_> {
    fn as_source(&self) -> Option<&GlslModule> {
        Some(&self.glsl)
    }
    fn as_spirv(&self) -> Option<&SpirVModule> {
        Some(&self.spirv)
    }
}
