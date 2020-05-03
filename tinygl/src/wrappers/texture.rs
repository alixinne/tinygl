use crate::context::HasContext;

pub struct Texture {
    name: <glow::Context as HasContext>::Texture,
}

impl Texture {
    pub fn new(gl: &crate::Context) -> crate::Result<Self> {
        Ok(Self {
            name: unsafe {
                gl.create_texture()
                    .map_err(|msg| crate::Error::TextureCreationFailed(msg))
            }?,
        })
    }

    pub fn name(&self) -> <glow::Context as HasContext>::Texture {
        self.name
    }

    pub fn bind(&self, gl: &crate::Context, target: u32) {
        unsafe { gl.bind_texture(target, Some(self.name)) };
    }
}

impl super::GlDrop for Texture {
    fn drop(&mut self, gl: &crate::Context) {
        unsafe { gl.delete_texture(self.name) }
    }
}
