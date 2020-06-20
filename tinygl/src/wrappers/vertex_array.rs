use crate::OpenGlErrorCode;

pub struct VertexArray {
    name: crate::gl::VertexArray,
}

impl VertexArray {
    impl_nnew!(
        VertexArrayCreationFailed,
        create_vertex_arrays,
        create_vertex_array
    );

    impl_name!(pub crate::gl::VertexArrayName);

    pub unsafe fn bind(&self, gl: &crate::Context) {
        gl.bind_vertex_array(Some(self));
    }
}

impl_ndrop!(VertexArray, delete_vertex_arrays, delete_vertex_array);
