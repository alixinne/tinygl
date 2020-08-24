use crate::OpenGlErrorCode;

pub struct Query {
    name: crate::gl::Query,
}

impl Query {
    impl_nnew!(QueryCreationFailed, gen_queries, gen_query);

    impl_name!(pub crate::gl::QueryName);

    pub unsafe fn begin(&self, gl: &crate::Context, target: u32) {
        gl.begin_query(target, self.name);
    }

    pub unsafe fn result_u64(&self, gl: &crate::Context) -> u64 {
        let mut res = 0u64;
        gl.get_query_objectui64v(self.name, crate::gl::QUERY_RESULT, &mut res);
        res
    }
}

impl_ndrop!(Query, delete_queries, delete_query);
