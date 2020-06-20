use crate::OpenGlErrorCode;

pub struct Renderbuffer {
    name: crate::gl::Renderbuffer,
}

impl Renderbuffer {
    impl_nnew!(
        RenderbufferCreationFailed,
        create_renderbuffers,
        create_renderbuffer
    );

    impl_name!(pub crate::gl::RenderbufferName);

    pub unsafe fn bind(&self, gl: &crate::Context) {
        gl.bind_renderbuffer(crate::gl::RENDERBUFFER, Some(self));
    }
}

impl_ndrop!(Renderbuffer, delete_renderbuffers, delete_renderbuffer);
