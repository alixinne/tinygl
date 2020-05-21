use crate::context::HasContext;

// TODO: We're using an unwrapped program name because otherwise we would need to borrow the
// program object to keep the location around. Or, this is leads to borrowck problems. The runtime
// program/uniform API being unsafe, this isn't too bad but it's still worth looking into later.

// TODO: The glUniform calls require the program to be in use, we should use ProgramUniform if
// possible.

#[derive(Clone)]
pub struct UniformLocation {
    program: <glow::Context as HasContext>::Program,
    location: Option<<glow::Context as HasContext>::UniformLocation>,
}

impl std::fmt::Debug for UniformLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UniformLocation")
            .field("program", &self.program)
            .field("location", &self.location)
            .finish()
    }
}

impl UniformLocation {
    pub fn new(
        program: <glow::Context as HasContext>::Program,
        location: Option<<glow::Context as HasContext>::UniformLocation>,
    ) -> Self {
        Self { program, location }
    }

    pub fn is_active(&self) -> bool {
        self.location.is_some()
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    ///
    /// # Safety
    ///
    /// Requires the associated program to be in use.
    pub unsafe fn set_f32(&self, gl: &crate::Context, v: f32) {
        gl.uniform_1_f32(self.location.as_ref(), v)
    }
}
