use std::mem::transmute;

#[derive(Clone)]
pub struct UniformLocation {
    location: Option<crate::gl::UniformLocation>,
}

impl std::fmt::Debug for UniformLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UniformLocation")
            .field("location", &self.location)
            .finish()
    }
}

impl UniformLocation {
    pub fn new(location: Option<crate::gl::UniformLocation>) -> Self {
        Self { location }
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
        gl.uniform1f(self.location.as_ref(), v)
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
        gl.uniform2fv_with_f32_array(self.location.as_ref(), v.as_ref())
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
        gl.uniform3fv_with_f32_array(self.location.as_ref(), v.as_ref())
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
        gl.uniform4fv_with_f32_array(self.location.as_ref(), v.as_ref())
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
        gl.uniform1i(self.location.as_ref(), v)
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
        gl.uniform2iv_with_i32_array(self.location.as_ref(), v.as_ref())
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
        gl.uniform3iv_with_i32_array(self.location.as_ref(), v.as_ref())
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
        gl.uniform4iv_with_i32_array(self.location.as_ref(), v.as_ref())
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
        gl.uniform1ui(self.location.as_ref(), v)
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
        gl.uniform2uiv_with_u32_array(self.location.as_ref(), v.as_ref())
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
        gl.uniform3uiv_with_u32_array(self.location.as_ref(), v.as_ref())
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
        gl.uniform4uiv_with_u32_array(self.location.as_ref(), v.as_ref())
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `transpose`: transpose the matrix before uploading
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
        gl.uniform_matrix2fv_with_f32_array(
            self.location.as_ref(),
            transpose,
            transmute(&v.as_ref()[..]),
        )
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `transpose`: transpose the matrix before uploading
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
        gl.uniform_matrix3fv_with_f32_array(
            self.location.as_ref(),
            transpose,
            transmute(&v.as_ref()[..]),
        )
    }

    /// Set the uniform location to the given value
    ///
    /// # Parameters
    ///
    /// * `gl`: current OpenGL context
    /// * `transpose`: transpose the matrix before uploading
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
        gl.uniform_matrix4fv_with_f32_array(
            self.location.as_ref(),
            transpose,
            transmute(&v.as_ref()[..]),
        )
    }
}
