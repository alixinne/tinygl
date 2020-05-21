use crate::context::HasContext;

pub struct VertexArray {
    name: <glow::Context as HasContext>::VertexArray,
}

impl VertexArray {
    pub fn new(gl: &crate::Context) -> crate::Result<Self> {
        Ok(Self {
            name: unsafe {
                gl.create_vertex_array()
                    .map_err(|msg| crate::Error::VertexArrayCreationFailed(msg))
            }?,
        })
    }

    pub fn name(&self) -> <glow::Context as HasContext>::VertexArray {
        self.name
    }

    pub fn bind(&self, gl: &crate::Context) {
        unsafe { gl.bind_vertex_array(Some(self.name)) };
    }
}

impl super::GlDrop for VertexArray {
    unsafe fn drop(&mut self, gl: &crate::Context) {
        gl.delete_vertex_array(self.name);
    }
}
