use super::*;

use std::borrow::Cow;

use ::glsl::parser::Parse;
use ::glsl::syntax::{
    ExternalDeclaration, Preprocessor, PreprocessorVersionProfile, TranslationUnit,
};

use crate::GlslVersion;

/// A GLSL shader
pub struct GlslModule<'s> {
    /// Raw GLSL source
    source: Cow<'s, str>,
    /// Parsed AST,
    ast: Option<TranslationUnit>,
    /// Parsed (or assumed) version
    version: Option<GlslVersion>,
}

impl<'s> GlslModule<'s> {
    /// Return the source of this module as a string
    pub fn as_str(&self) -> &str {
        self.source.as_ref()
    }

    /// Load a GLSL shader from a file
    ///
    /// # Parameters
    ///
    /// * `p`: path to load the shader from
    pub(crate) fn from_path(p: impl AsRef<Path>) -> Result<Self> {
        let p = p.as_ref();
        let source = std::fs::read_to_string(p)?;

        Self::from_string(source)
    }

    /// Load a GLSL shader from source
    ///
    /// # Parameters
    ///
    /// * `source`: source to load the shader from
    pub(crate) fn from_string(source: String) -> Result<GlslModule<'static>> {
        Ok(GlslModule {
            source: Cow::Owned(source),
            ast: None,
            version: None,
        })
    }

    /// Load a GLSL shader from source
    ///
    /// # Parameters
    ///
    /// * `source`: source to load the shader from
    pub(crate) fn from_str(source: &'s str) -> Result<Self> {
        Ok(Self {
            source: Cow::Borrowed(source),
            ast: None,
            version: None,
        })
    }

    /// Parse the source into an AST
    ///
    /// Note that this step is separate as the GLSL parser isn't feature complete.
    pub fn parse(&mut self) -> Result<()> {
        if self.ast.is_none() {
            let ast = TranslationUnit::parse(&self.source)?;

            for decl in &ast.0 {
                if let ExternalDeclaration::Preprocessor(Preprocessor::Version(version)) = decl {
                    self.version = match version.profile {
                        Some(PreprocessorVersionProfile::ES) => match version.version {
                            100 => Some(GlslVersion::V1_00Es),
                            300 => Some(GlslVersion::V3_00Es),
                            _ => None,
                        },
                        _ => match version.version {
                            110 => Some(GlslVersion::V1_10),
                            120 => Some(GlslVersion::V1_20),
                            130 => Some(GlslVersion::V1_30),
                            140 => Some(GlslVersion::V1_40),
                            150 => Some(GlslVersion::V1_50),
                            330 => Some(GlslVersion::V3_30),
                            400 => Some(GlslVersion::V4_00),
                            410 => Some(GlslVersion::V4_10),
                            420 => Some(GlslVersion::V4_20),
                            430 => Some(GlslVersion::V4_30),
                            440 => Some(GlslVersion::V4_40),
                            450 => Some(GlslVersion::V4_50),
                            460 => Some(GlslVersion::V4_60),
                            _ => None,
                        },
                    };
                }
            }

            self.ast = Some(ast);
        }

        Ok(())
    }

    /// Get the version of this source object
    ///
    /// # Returns
    ///
    /// `None` if the version hasn't been parsed yet or could not be parsed from the source.
    pub fn version(&self) -> Option<GlslVersion> {
        self.version
    }
}

#[cfg(feature = "spirv")]
use super::spirv::GlslWithSpirVModule;

impl<'s> ShaderObject<GlslModule<'s>> {
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
            module: GlslModule::from_path(&p)?,
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
    pub fn from_string(
        source: String,
        kind: ShaderKind,
    ) -> Result<ShaderObject<GlslModule<'static>>> {
        let info = bytes_to_id(source.as_bytes(), kind);

        Ok(ShaderObject {
            module: GlslModule::from_string(source)?,
            info,
        })
    }

    /// Load a GLSL shader from source
    ///
    /// # Parameters
    ///
    /// * `source`: source to load the shader from
    /// * `kind`: shader stage
    pub fn from_str(source: &'s str, kind: ShaderKind) -> Result<Self> {
        let info = bytes_to_id(source.as_bytes(), kind);

        Ok(Self {
            module: GlslModule::from_str(source)?,
            info,
        })
    }

    pub fn preprocess(
        self,
        preprocessor: &mut dyn GlslPreprocessor,
    ) -> Result<ShaderObject<GlslModule<'static>>> {
        let source_module = preprocessor.preprocess_module(
            self.module.as_str(),
            self.info.source_path.to_string().as_str(),
        )?;

        Ok(ShaderObject {
            module: source_module,
            info: self.info,
        })
    }

    #[cfg(feature = "spirv")]
    pub fn compile(
        self,
        compiler: &mut dyn GlslCompiler,
    ) -> Result<ShaderObject<GlslWithSpirVModule<'s, 'static>>> {
        let binary_module = compiler.compile_module(
            self.module.as_str(),
            self.info.kind,
            self.info.source_path.to_string().as_str(),
        )?;

        Ok(ShaderObject {
            module: GlslWithSpirVModule::new(self.module, binary_module),
            info: self.info,
        })
    }

    pub fn reflect(self, reflector: &dyn GlslReflectBackend) -> Result<ReflectedObject<Self>> {
        let uniforms = reflector.reflect(self.module.as_str())?;
        Ok(ReflectedObject::new(self, uniforms))
    }
}

impl AsOutputFormat for GlslModule<'_> {
    fn as_source(&self) -> Option<&GlslModule> {
        Some(self)
    }
}
