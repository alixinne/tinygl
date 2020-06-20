use crate::OpenGlErrorCode;

pub struct Framebuffer {
    name: crate::gl::Framebuffer,
}

impl Framebuffer {
    impl_nnew!(
        FramebufferCreationFailed,
        create_framebuffers,
        create_framebuffer
    );

    impl_name!(pub crate::gl::FramebufferName);

    pub unsafe fn bind(&self, gl: &crate::Context, target: u32) {
        gl.bind_framebuffer(target, Some(self));
    }
}

impl_ndrop!(Framebuffer, delete_framebuffers, delete_framebuffer);
