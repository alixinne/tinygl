//! OpenGL object wrappers

use std::rc::Rc;

mod buffer;
pub use buffer::*;

mod framebuffer;
pub use framebuffer::*;

mod renderbuffer;
pub use renderbuffer::*;

mod shader;
pub use shader::*;

mod program;
pub use program::*;

mod texture;
pub use texture::*;

/// Trait for GL objects that can be dropped
pub trait GlDrop {
    fn drop(&mut self, gl: &crate::Context);
}

/// Handle to a GL object that will be cleaned up when this handle is dropped
///
/// This keeps a RC reference to the context, so it is best used as a long-lived handle.
pub struct GlHandle<T: GlDrop> {
    gl: Rc<crate::Context>,
    res: Option<T>,
}

impl<T: GlDrop> GlHandle<T> {
    pub fn new(gl: &Rc<crate::Context>, res: T) -> Self {
        Self {
            gl: gl.clone(),
            res: Some(res),
        }
    }

    pub fn into_inner(mut self) -> T {
        self.res.take().unwrap()
    }
}

impl<T: GlDrop> Drop for GlHandle<T> {
    fn drop(&mut self) {
        self.res.take().map(|mut res| res.drop(self.gl.as_ref()));
    }
}

impl<T: GlDrop> std::ops::Deref for GlHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.res.as_ref().unwrap()
    }
}

impl<T: GlDrop> std::ops::DerefMut for GlHandle<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.res.as_mut().unwrap()
    }
}

impl<T: GlDrop> std::convert::AsRef<T> for GlHandle<T> {
    fn as_ref(&self) -> &T {
        self.res.as_ref().unwrap()
    }
}

impl<T: GlDrop> std::convert::AsMut<T> for GlHandle<T> {
    fn as_mut(&mut self) -> &mut T {
        self.res.as_mut().unwrap()
    }
}

/// Handle to a GL object that will be cleaned up when this handle is dropped
///
/// This keeps a reference to the context, so it is best used as a temporary handle.
pub struct GlRefHandle<'gl, T: GlDrop> {
    gl: &'gl crate::Context,
    res: Option<T>,
}

impl<'gl, T: GlDrop> GlRefHandle<'gl, T> {
    pub fn new(gl: &'gl crate::Context, res: T) -> Self {
        Self { gl, res: Some(res) }
    }

    pub fn into_inner(mut self) -> T {
        self.res.take().unwrap()
    }
}

impl<'gl, T: GlDrop> Drop for GlRefHandle<'gl, T> {
    fn drop(&mut self) {
        self.res.take().map(|mut res| res.drop(self.gl));
    }
}

impl<'gl, T: GlDrop> std::convert::AsRef<T> for GlRefHandle<'gl, T> {
    fn as_ref(&self) -> &T {
        self.res.as_ref().unwrap()
    }
}

impl<'gl, T: GlDrop> std::convert::AsMut<T> for GlRefHandle<'gl, T> {
    fn as_mut(&mut self) -> &mut T {
        self.res.as_mut().unwrap()
    }
}

impl<'gl, T: GlDrop> std::ops::Deref for GlRefHandle<'gl, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.res.as_ref().unwrap()
    }
}

impl<'gl, T: GlDrop> std::ops::DerefMut for GlRefHandle<'gl, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.res.as_mut().unwrap()
    }
}
