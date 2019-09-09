//! Context type definition

use glow::HasContext;

use super::{Demo, CompileError};

pub struct Context {
    pub gl: glow::Context,
    quad_vao: <glow::Context as HasContext>::VertexArray,
    vertex_shader: <glow::Context as HasContext>::Shader,
    shader_version: &'static str,
}

impl Context {
    pub fn new(shader_version: &'static str, gl: glow::Context) -> Self {
        unsafe {
            // Create VAO object
            let quad_vao = gl
                .create_vertex_array()
                .expect("cannot create quad vertex array");

            // Create quad vertex buffer
            let vertex_shader_source = r#"
                out vec2 texCoords;
                void main() {
                    texCoords = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2);
                    gl_Position = vec4(texCoords * 2. - 1., 0., 1.);
                }
            "#;

            let vertex_shader = gl
                .create_shader(glow::VERTEX_SHADER)
                .expect("cannot create quad vertex shader");
            gl.shader_source(
                vertex_shader,
                &format!("{}\n{}", shader_version, vertex_shader_source),
            );
            gl.compile_shader(vertex_shader);

            info!("created tinygl context!");
            Self {
                gl,
                quad_vao,
                vertex_shader,
                shader_version,
            }
        }
    }

    pub fn bind_vao(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.quad_vao));
        }
    }

    pub fn render(&self, demo: &Demo) {
        unsafe {
            self.gl.clear(glow::COLOR_BUFFER_BIT);
            demo.render(self);
        }
    }

    pub fn compile_fragment(
        &self,
        fragment: &str,
    ) -> Result<<glow::Context as HasContext>::Program, CompileError> {
        let gl = &self.gl;

        unsafe {
            let shader = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .expect("failed to create fragment shader");
            gl.shader_source(shader, &format!("{}\n{}", self.shader_version, fragment));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                gl.delete_shader(shader);
                return Err(CompileError::CompilationError { log: gl.get_shader_info_log(shader) });
            }

            // Create program
            let program = gl.create_program().expect("failed to create program");

            // Attach shaders
            gl.attach_shader(program, self.vertex_shader);
            gl.attach_shader(program, shader);

            // Link program
            gl.link_program(program);

            // Cleanup shader resources
            gl.detach_shader(program, self.vertex_shader);
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);

            if !gl.get_program_link_status(program) {
                gl.delete_program(program);
                return Err(CompileError::LinkError { log: gl.get_program_info_log(program) });
            }

            Ok(program)
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.quad_vao);
        }
    }
}
