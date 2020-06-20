use crate::OpenGlErrorCode;

pub struct Buffer {
    name: crate::gl::Buffer,
}

impl Buffer {
    impl_nnew!(BufferCreationFailed, create_buffers, create_buffer);

    impl_name!(pub crate::gl::BufferName);

    pub unsafe fn bind(&self, gl: &crate::Context, target: u32) {
        gl.bind_buffer(target, Some(self));
    }
}

impl_ndrop!(Buffer, delete_buffers, delete_buffer);
