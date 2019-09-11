use std::path::Path;

use failure::Fail;
use glow::HasContext;

use tinygl_renderer::demo::*;

pub struct State {
    context: Context,
    demo: Option<Demo>,
}

impl State {
    pub fn new(
        gl: glow::Context,
        render_size: cgmath::Vector2<u32>,
        shader_version: &'static str,
    ) -> Self {
        Self {
            context: Context::new(gl, render_size, shader_version),
            demo: None,
        }
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), DemoError> {
        use std::fs::File;

        let mut new_demo: Demo = match path
            .as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
        {
            Some(ref s) => match &s[..] {
                "yaml" | "yml" => serde_yaml::from_reader(File::open::<P>(path)?)?,
                "json" => serde_json::from_reader(File::open::<P>(path)?)?,
                s => return Err(DemoError::UnsupportedFormat(s.to_owned())),
            },
            _ => return Err(DemoError::UnsupportedFormat("<unknown>".to_owned())),
        };

        // Compile demo
        new_demo.compile(&self.context)?;

        // Prepare rendering resources
        new_demo.prepare_render(&self.context)?;

        // Replace current demo now that everything is okay
        let gl = &self.context.gl;
        self.demo
            .replace(new_demo)
            .map(|mut old_demo| old_demo.drop(gl));

        Ok(())
    }

    pub fn drop_demo(&mut self) {
        self.demo.take().map(|mut demo| demo.drop(&self.context.gl));
    }

    pub fn resize(&mut self, new_size: cgmath::Vector2<u32>) -> Result<(), String> {
        // Check that we actually need to resize
        if self.context.render_size == new_size {
            return Ok(());
        }

        // Update size
        self.context.render_size = new_size;

        // Prepare resources
        let context = &self.context;
        self.demo
            .as_mut()
            .map_or(Ok(()), |demo| demo.prepare_render(context))
    }

    pub fn render(&self) {
        if let Some(demo) = &self.demo {
            self.context.render(demo);
        } else {
            unsafe {
                self.context.gl.clear(glow::COLOR_BUFFER_BIT);
            }
        }
    }

    pub fn bind_vao(&self) {
        self.context.bind_vao();
    }
}

#[derive(Debug, Fail)]
pub enum DemoError {
    #[fail(display = "unsupported file format: {}", 0)]
    UnsupportedFormat(String),
    #[fail(display = "invalid json: {}", 0)]
    JsonError(serde_json::Error),
    #[fail(display = "invalid yaml: {}", 0)]
    YamlError(serde_yaml::Error),
    #[fail(display = "i/o error: {}", 0)]
    IoError(std::io::Error),
    #[fail(display = "compile error: {}", 0)]
    CompileError(CompileError),
    #[fail(display = "prepare render error: {}", 0)]
    PrepareRenderError(String),
}

macro_rules! demo_error_for {
    ($error_type:ty, $target:ident) => {
        impl From<$error_type> for DemoError {
            fn from(error: $error_type) -> Self {
                DemoError::$target(error)
            }
        }
    };
}

demo_error_for!(serde_json::Error, JsonError);
demo_error_for!(serde_yaml::Error, YamlError);
demo_error_for!(std::io::Error, IoError);
demo_error_for!(CompileError, CompileError);
demo_error_for!(String, PrepareRenderError);
