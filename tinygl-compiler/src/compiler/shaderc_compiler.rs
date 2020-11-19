use std::path::Path;

use crate::model::{GlslCompiler, GlslModule, GlslPreprocessor, SpirVModule};
use crate::{Compiler, Error, ShaderKind};

pub struct CompilerWithShaderc {
    compiler: Compiler,
    shaderc: shaderc::Compiler,
}

impl CompilerWithShaderc {
    pub fn new(compiler: Compiler) -> Self {
        Self {
            compiler,
            shaderc: shaderc::Compiler::new().unwrap(),
        }
    }

    fn get_options(&self) -> shaderc::CompileOptions<'static> {
        let skip_cargo = self.compiler.skip_cargo;
        let cb = self.compiler.include_callback.as_ref().map(|cb| cb.clone());

        // Set callback
        let mut options = shaderc::CompileOptions::new().unwrap();

        // Add definitions
        // TODO: Let use configure options?
        options.add_macro_definition("TINYGL", Some(env!("CARGO_PKG_VERSION_MAJOR")));

        // Default to OpenGL targets
        options.set_target_env(shaderc::TargetEnv::OpenGL, 0);

        // Set include callback
        options.set_include_callback(move |name, _include_type, source, _depth| {
            // TODO: Circular includes?
            // TODO: Better include resolver?
            match std::fs::canonicalize(Path::new(&source).parent().unwrap().join(name)) {
                Ok(full_path) => {
                    if !skip_cargo {
                        // Notify cargo to rerun if included file changed
                        println!("cargo:rerun-if-changed={}", full_path.display());
                    }

                    if let Some(cb) = &cb {
                        cb.borrow_mut()(&full_path);
                    }

                    match std::fs::read_to_string(&full_path) {
                        Ok(content) => Ok(shaderc::ResolvedInclude {
                            resolved_name: full_path.to_string_lossy().to_string(),
                            content,
                        }),
                        Err(error) => Err(error.to_string()),
                    }
                }
                Err(error) => Err(error.to_string()),
            }
        });

        options
    }
}

impl GlslCompiler for CompilerWithShaderc {
    fn compile_module(
        &mut self,
        source: &str,
        kind: ShaderKind,
        source_path: &str,
    ) -> Result<SpirVModule<'static>, crate::Error> {
        let options = self.get_options();

        // Compile the requested targets
        match self.shaderc.compile_into_spirv(
            source,
            kind.into(),
            source_path,
            "main",
            Some(&options),
        ) {
            Ok(result) => {
                if !self.compiler.skip_cargo {
                    // Print warnings
                    // TODO: Store warnings in result?
                    if result.get_num_warnings() > 0 {
                        for l in result.get_warning_messages().lines() {
                            println!("cargo:warning={}", l);
                        }
                    }
                }

                Ok(SpirVModule::from_words(result.as_binary().to_vec())?)
            }
            Err(shaderc::Error::CompilationError(num_errors, errors)) => {
                if !self.compiler.skip_cargo {
                    eprintln!("{}", errors);
                }

                return Err(Error::CompilationError(num_errors as usize, errors.clone()));
            }
            Err(error) => panic!(error.to_string()),
        }
    }
}

impl GlslPreprocessor for CompilerWithShaderc {
    fn preprocess_module(
        &mut self,
        source: &str,
        source_path: &str,
    ) -> Result<GlslModule<'static>, crate::Error> {
        let options = self.get_options();

        // Compile the requested targets
        match self
            .shaderc
            .preprocess(source, source_path, "main", Some(&options))
        {
            Ok(result) => {
                if !self.compiler.skip_cargo {
                    // Print warnings
                    // TODO: Store warnings in result?
                    if result.get_num_warnings() > 0 {
                        for l in result.get_warning_messages().lines() {
                            println!("cargo:warning={}", l);
                        }
                    }
                }

                Ok(GlslModule::from_string(result.as_text())?)
            }
            Err(shaderc::Error::CompilationError(num_errors, errors)) => {
                if !self.compiler.skip_cargo {
                    eprintln!("{}", errors);
                }

                return Err(Error::CompilationError(num_errors as usize, errors.clone()));
            }
            Err(error) => panic!(error.to_string()),
        }
    }
}

impl std::ops::Deref for CompilerWithShaderc {
    type Target = Compiler;

    fn deref(&self) -> &Self::Target {
        &self.compiler
    }
}

impl std::ops::DerefMut for CompilerWithShaderc {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        &mut self.compiler
    }
}
