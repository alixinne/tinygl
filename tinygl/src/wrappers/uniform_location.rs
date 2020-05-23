use std::mem::transmute;

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
    pub unsafe fn set_vec2(&self, gl: &crate::Context, v: impl AsRef<[f32; 2]>) {
        gl.uniform_2_f32_slice(self.location.as_ref(), v.as_ref())
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
    pub unsafe fn set_vec3(&self, gl: &crate::Context, v: impl AsRef<[f32; 3]>) {
        gl.uniform_3_f32_slice(self.location.as_ref(), v.as_ref())
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
    pub unsafe fn set_vec4(&self, gl: &crate::Context, v: impl AsRef<[f32; 4]>) {
        gl.uniform_4_f32_slice(self.location.as_ref(), v.as_ref())
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
    pub unsafe fn set_i32(&self, gl: &crate::Context, v: i32) {
        gl.uniform_1_i32(self.location.as_ref(), v)
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
    pub unsafe fn set_ivec2(&self, gl: &crate::Context, v: impl AsRef<[i32; 2]>) {
        gl.uniform_2_i32_slice(self.location.as_ref(), v.as_ref())
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
    pub unsafe fn set_ivec3(&self, gl: &crate::Context, v: impl AsRef<[i32; 3]>) {
        gl.uniform_3_i32_slice(self.location.as_ref(), v.as_ref())
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
    pub unsafe fn set_ivec4(&self, gl: &crate::Context, v: impl AsRef<[i32; 4]>) {
        gl.uniform_4_i32_slice(self.location.as_ref(), v.as_ref())
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
    pub unsafe fn set_u32(&self, gl: &crate::Context, v: u32) {
        gl.uniform_1_u32(self.location.as_ref(), v)
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
    pub unsafe fn set_uvec2(&self, gl: &crate::Context, v: impl AsRef<[u32; 2]>) {
        gl.uniform_2_u32_slice(self.location.as_ref(), v.as_ref())
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
    pub unsafe fn set_uvec3(&self, gl: &crate::Context, v: impl AsRef<[u32; 3]>) {
        gl.uniform_3_u32_slice(self.location.as_ref(), v.as_ref())
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
    pub unsafe fn set_uvec4(&self, gl: &crate::Context, v: impl AsRef<[u32; 4]>) {
        gl.uniform_4_u32_slice(self.location.as_ref(), v.as_ref())
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
    pub unsafe fn set_mat2(
        &self,
        gl: &crate::Context,
        transpose: bool,
        v: impl AsRef<[[f32; 2]; 2]>,
    ) {
        gl.uniform_matrix_2_f32_slice(
            self.location.as_ref(),
            transpose,
            transmute::<_, &[_; 4]>(v.as_ref()),
        )
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
    pub unsafe fn set_mat3(
        &self,
        gl: &crate::Context,
        transpose: bool,
        v: impl AsRef<[[f32; 3]; 3]>,
    ) {
        gl.uniform_matrix_3_f32_slice(
            self.location.as_ref(),
            transpose,
            transmute::<_, &[_; 9]>(v.as_ref()),
        )
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
    pub unsafe fn set_mat4(
        &self,
        gl: &crate::Context,
        transpose: bool,
        v: impl AsRef<[[f32; 4]; 4]>,
    ) {
        gl.uniform_matrix_4_f32_slice(
            self.location.as_ref(),
            transpose,
            transmute::<_, &[_; 16]>(v.as_ref()),
        )
    }
}
