use crate::gl;

// TODO: We're using an unwrapped program name because otherwise we would need to borrow the
// program object to keep the location around. Or, this is leads to borrowck problems. The runtime
// program/uniform API being unsafe, this isn't too bad but it's still worth looking into later.

#[derive(Clone)]
pub struct UniformLocation {
    program: crate::gl::Program,
    location: Option<crate::gl::UniformLocation>,
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
    pub fn new(program: crate::gl::Program, location: Option<crate::gl::UniformLocation>) -> Self {
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
    pub fn set_f32(&self, gl: &crate::Context, v: f32) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform1f(self.program, location, v) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_vec2(&self, gl: &crate::Context, v: impl AsRef<[f32; 2]>) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform2fv(self.program, location, 1, v.as_ref().as_ptr()) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_vec3(&self, gl: &crate::Context, v: impl AsRef<[f32; 3]>) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform3fv(self.program, location, 1, v.as_ref().as_ptr()) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_vec4(&self, gl: &crate::Context, v: impl AsRef<[f32; 4]>) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform4fv(self.program, location, 1, v.as_ref().as_ptr()) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_i32(&self, gl: &crate::Context, v: i32) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform1i(self.program, location, v) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_ivec2(&self, gl: &crate::Context, v: impl AsRef<[i32; 2]>) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform2iv(self.program, location, 1, v.as_ref().as_ptr()) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_ivec3(&self, gl: &crate::Context, v: impl AsRef<[i32; 3]>) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform3iv(self.program, location, 1, v.as_ref().as_ptr()) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_ivec4(&self, gl: &crate::Context, v: impl AsRef<[i32; 4]>) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform4iv(self.program, location, 1, v.as_ref().as_ptr()) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_u32(&self, gl: &crate::Context, v: u32) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform1ui(self.program, location, v) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_uvec2(&self, gl: &crate::Context, v: impl AsRef<[u32; 2]>) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform2uiv(self.program, location, 1, v.as_ref().as_ptr()) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_uvec3(&self, gl: &crate::Context, v: impl AsRef<[u32; 3]>) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform3uiv(self.program, location, 1, v.as_ref().as_ptr()) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `v`: new uniform value
    pub fn set_uvec4(&self, gl: &crate::Context, v: impl AsRef<[u32; 4]>) {
        if let Some(location) = self.location {
            unsafe { gl.program_uniform4uiv(self.program, location, 1, v.as_ref().as_ptr()) }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `transpose`: transpose the matrix before uploading
    /// * `v`: new uniform value
    pub fn set_mat2(&self, gl: &crate::Context, transpose: bool, v: impl AsRef<[[f32; 2]; 2]>) {
        if let Some(location) = self.location {
            unsafe {
                gl.program_uniform_matrix2fv(
                    self.program,
                    location,
                    1,
                    if transpose { gl::TRUE } else { gl::FALSE },
                    v.as_ref().as_ptr() as *const _,
                )
            }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `transpose`: transpose the matrix before uploading
    /// * `v`: new uniform value
    pub fn set_mat3(&self, gl: &crate::Context, transpose: bool, v: impl AsRef<[[f32; 3]; 3]>) {
        if let Some(location) = self.location {
            unsafe {
                gl.program_uniform_matrix3fv(
                    self.program,
                    location,
                    1,
                    if transpose { gl::TRUE } else { gl::FALSE },
                    v.as_ref().as_ptr() as *const _,
                )
            }
        }
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `transpose`: transpose the matrix before uploading
    /// * `v`: new uniform value
    pub fn set_mat4(&self, gl: &crate::Context, transpose: bool, v: impl AsRef<[[f32; 4]; 4]>) {
        if let Some(location) = self.location {
            unsafe {
                gl.program_uniform_matrix4fv(
                    self.program,
                    location,
                    1,
                    if transpose { gl::TRUE } else { gl::FALSE },
                    v.as_ref().as_ptr() as *const _,
                )
            }
        }
    }
}
