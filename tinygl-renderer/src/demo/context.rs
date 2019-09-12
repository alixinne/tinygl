//! Context type definition

use glow::HasContext;

use super::{CompileError, StepProgram};

pub struct Context {
    pub gl: glow::Context,
    pub render_size: cgmath::Vector2<u32>,

    quad_vao: <glow::Context as HasContext>::VertexArray,
    vertex_shader: <glow::Context as HasContext>::Shader,
    shader_version: &'static str,
}

impl Context {
    pub fn new(
        gl: glow::Context,
        render_size: cgmath::Vector2<u32>,
        shader_version: &'static str,
    ) -> Self {
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
                &[shader_version, vertex_shader_source].join("\n"),
            );
            gl.compile_shader(vertex_shader);

            info!("created tinygl context!");
            Self {
                gl,
                render_size,
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

    pub fn compile_fragment(&self, fragment: &str) -> Result<StepProgram, CompileError> {
        let gl = &self.gl;

        unsafe {
            let shader = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .expect("failed to create fragment shader");
            let shader_source = format!("{}\n{}", self.shader_version, fragment);
            gl.shader_source(shader, &shader_source);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                let log = gl.get_shader_info_log(shader);
                gl.delete_shader(shader);
                return Err(CompileError::CompilationError { log });
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
                let log = gl.get_program_info_log(program);
                gl.delete_program(program);
                return Err(CompileError::LinkError { log });
            }

            // The StepProgram will parse declarations
            Ok(StepProgram::new(&self.gl, &shader_source, program)?)
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.quad_vao);
            self.gl.delete_shader(self.vertex_shader);
        }
    }
}
