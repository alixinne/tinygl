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

    pub unsafe fn bind_base(&self, gl: &crate::Context, target: u32, index: u32) {
        gl.bind_buffer_base(target, index, Some(self));
    }

    pub unsafe fn bind_range(
        &self,
        gl: &crate::Context,
        target: u32,
        index: u32,
        offset: isize,
        size: isize,
    ) {
        gl.bind_buffer_range(target, index, Some(self), offset, size);
    }
}

impl_ndrop!(Buffer, delete_buffers, delete_buffer);
