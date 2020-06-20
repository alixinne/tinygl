use crate::OpenGlErrorCode;

pub struct Texture {
    name: crate::gl::Texture,
}

impl Texture {
    impl_nnew!(TextureCreationFailed, gen_textures, create_texture);

    impl_name!(pub crate::gl::TextureName);

    pub unsafe fn bind(&self, gl: &crate::Context, target: u32) {
        gl.bind_texture(target, Some(self));
    }
}

impl_ndrop!(Texture, delete_textures, delete_texture);
