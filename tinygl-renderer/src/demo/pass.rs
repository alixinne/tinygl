//! Pass type definition

use serde_derive::{Deserialize, Serialize};

use glow::HasContext;

use super::{CompileError, Context, Demo, GlDroppable, StepProgram};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Pass {
    pub name: String,
    fragment: String,
    #[serde(skip_deserializing)]
    program: Option<StepProgram>,
    #[serde(skip)]
    render_size: Option<cgmath::Vector2<u32>>,
    #[serde(skip)]
    render_texture: Option<<glow::Context as glow::HasContext>::Texture>,
    #[serde(skip)]
    render_texture_size: Option<cgmath::Vector2<u32>>,
    #[serde(skip)]
    render_framebuffer: Option<<glow::Context as glow::HasContext>::Framebuffer>,
}

pub struct PassBuilder {
    name: String,
    fragment: String,
}

impl PassBuilder {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            fragment: Pass::default().fragment,
        }
    }

    pub fn with_fragment<S: Into<String>>(self, fragment: S) -> Self {
        Self {
            fragment: fragment.into(),
            ..self
        }
    }

    pub fn sample<S: Into<String>>(name: S) -> PassBuilder {
        Self {
            name: name.into(),
            fragment: r#"void main() {
                color = vec4(texCoords.xy, 0.5, 1.0);
            }"#
            .to_owned(),
        }
    }

    pub fn build(self) -> Pass {
        Pass {
            name: self.name,
            fragment: self.fragment,
            ..Default::default()
        }
    }
}

impl Pass {
    pub fn compile(&mut self, context: &Context, common_code: &str) -> Result<(), CompileError> {
        self.program = Some(context.compile_fragment(&format!("{}\n{}", common_code, self.fragment))?);

        Ok(())
    }

    fn compute_render_size(&self, context: &Context) -> cgmath::Vector2<u32> {
        // TODO: custom render sizes
        context.render_size
    }

    pub fn prepare_render(&mut self, context: &Context) -> Result<(), String> {
        self.render_size = Some(self.compute_render_size(context));
        let render_size = self.render_size.unwrap();
        let mut created_render_objects = false;

        unsafe {
            // Create texture if needed
            if self.render_texture.is_none() {
                self.render_texture = Some(context.gl.create_texture()?);

                // Note that we created a texture this call
                created_render_objects = true;
            }

            // Make sure texture is the right size
            if self.render_texture_size.is_none()
                || self.render_texture_size.unwrap() != render_size
            {
                // Bind texture
                context
                    .gl
                    .bind_texture(glow::TEXTURE_2D, self.render_texture);

                // Set default filter to nearest
                // TODO: support mipmaps and other filters
                context.gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MIN_FILTER,
                    glow::NEAREST as i32,
                );
                context.gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MAG_FILTER,
                    glow::NEAREST as i32,
                );

                // TODO: support other texture formats
                context.gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGBA as i32,
                    render_size.x as i32,
                    render_size.y as i32,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    None,
                );

                let error = context.gl.get_error();
                if error != glow::NO_ERROR {
                    // Unbind texture
                    context.gl.bind_texture(glow::TEXTURE_2D, None);
                    // Throw the texture away, we'll try with a new one
                    context
                        .gl
                        .delete_texture(self.render_texture.take().unwrap());

                    return Err(format!("failed to create texture: {}", error));
                }

                self.render_texture_size = Some(render_size);

                // Unbind texture
                context.gl.bind_texture(glow::TEXTURE_2D, None);
            }

            // Create framebuffer if needed
            if self.render_framebuffer.is_none() {
                self.render_framebuffer = Some(context.gl.create_framebuffer()?);

                // We created the framebuffer, so it needs the textures to be attached
                created_render_objects = true;
            }

            // Bind framebuffer
            context
                .gl
                .bind_framebuffer(glow::FRAMEBUFFER, self.render_framebuffer);

            // Bind the texture to the framebuffer
            if created_render_objects {
                context.gl.framebuffer_texture_2d(
                    glow::FRAMEBUFFER,
                    glow::COLOR_ATTACHMENT0,
                    glow::TEXTURE_2D,
                    self.render_texture,
                    0,
                );

                let error = context.gl.get_error();
                if error != glow::NO_ERROR {
                    // Unbind framebuffer
                    context.gl.bind_framebuffer(glow::FRAMEBUFFER, None);

                    return Err(format!("failed to attach texture: {}", error));
                }
            }

            // Check framebuffer status
            let status = context.gl.check_framebuffer_status(glow::FRAMEBUFFER);

            // Unbind framebuffer
            context.gl.bind_framebuffer(glow::FRAMEBUFFER, None);

            if status != glow::FRAMEBUFFER_COMPLETE {
                return Err("framebuffer is incomplete".to_owned());
            }
        }

        Ok(())
    }

    pub fn render(
        &self,
        context: &Context,
        demo: &Demo,
        target: Option<Option<<glow::Context as HasContext>::Framebuffer>>,
    ) {
        unsafe {
            let rs = self.render_size.expect("prepare_render was not called!");
            let program = self.program.as_ref().expect("compile was not called");

            // Bind framebuffer
            context
                .gl
                .bind_framebuffer(glow::FRAMEBUFFER, target.unwrap_or(self.render_framebuffer));
            // Set viewport for this draw call
            context.gl.viewport(0, 0, rs.x as i32, rs.y as i32);
            // Set program
            context.gl.use_program(Some(program.program));
            // Bind uniform samplers
            // TODO: Cache computations in this
            let mut active_unit = 0u32;
            for sampler in program.uniform_samplers.iter() {
                if let Some(location) = sampler.location {
                    context.gl.active_texture(glow::TEXTURE0 + active_unit);
                    context
                        .gl
                        .bind_texture(glow::TEXTURE_2D, demo.get_texture(&sampler.source));
                    context.gl.uniform_1_i32(Some(location), active_unit as i32);
                    active_unit += 1;
                }
            }

            // Render
            context.gl.draw_arrays(glow::TRIANGLES, 0, 3);

            // Unbind units
            for i in 0..active_unit {
                context.gl.active_texture(glow::TEXTURE0 + i);
                context.gl.bind_texture(glow::TEXTURE_2D, None);
            }
        }
    }

    pub fn blit(
        &self,
        context: &Context,
        target: Option<<glow::Context as HasContext>::Framebuffer>,
    ) {
        unsafe {
            context
                .gl
                .bind_framebuffer(glow::READ_FRAMEBUFFER, self.render_framebuffer);
            context.gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, target);

            let size = self.render_size.unwrap();
            context.gl.blit_framebuffer(
                0,
                0,
                size.x as i32,
                size.y as i32,
                0,
                0,
                size.x as i32,
                size.y as i32,
                glow::COLOR_BUFFER_BIT,
                glow::NEAREST,
            );
        }
    }

    pub fn get_render_texture(&self) -> Option<<glow::Context as glow::HasContext>::Texture> {
        self.render_texture
    }

}

impl Default for Pass {
    fn default() -> Self {
        Self {
            name: "image".to_owned(),
            fragment: r#"void main() {
                color = vec4(0.0, 0.0, 0.0, 1.0);
            }"#
            .to_owned(),
            program: None,
            render_size: None,
            render_texture: None,
            render_texture_size: None,
            render_framebuffer: None,
        }
    }
}

impl GlDroppable for Pass {
    fn drop(&mut self, gl: &glow::Context) {
        self.program.take().map(|mut program| program.drop(gl));
        unsafe {
            self.render_texture
                .take()
                .map(|texture| gl.delete_texture(texture));
            self.render_framebuffer
                .take()
                .map(|framebuffer| gl.delete_framebuffer(framebuffer));
        }
    }
}
