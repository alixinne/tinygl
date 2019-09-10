//! Definition of the GlDroppable trait

pub trait GlDroppable {
    fn drop(&mut self, gl: &glow::Context);
}
