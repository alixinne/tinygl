use crate::context::HasContext;

pub struct Renderbuffer {
    name: <glow::Context as HasContext>::Renderbuffer,
}

impl Renderbuffer {
    pub fn new(gl: &crate::Context) -> crate::Result<Self> {
        Ok(Self {
            name: unsafe {
                gl.create_renderbuffer()
                    .map_err(|msg| crate::Error::RenderbufferCreationFailed(msg))
            }?,
        })
    }

    pub fn name(&self) -> <glow::Context as HasContext>::Renderbuffer {
        self.name
    }

    pub fn bind(&self, gl: &crate::Context) {
        unsafe { gl.bind_renderbuffer(crate::gl::RENDERBUFFER, Some(self.name)) };
    }
}

impl super::GlDrop for Renderbuffer {
    unsafe fn drop(&mut self, gl: &crate::Context) {
        gl.delete_renderbuffer(self.name);
    }
}
