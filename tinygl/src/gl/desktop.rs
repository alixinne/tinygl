use crate::wrappers;

#[cfg(feature = "opengl44")]
mod bindings44;
#[cfg(feature = "opengl44")]
pub use bindings44::*;

#[cfg(feature = "opengl45")]
mod bindings45;
#[cfg(feature = "opengl45")]
pub use bindings45::*;

#[cfg(feature = "opengl46")]
mod bindings46;
#[cfg(feature = "opengl46")]
pub use bindings46::*;

pub type Buffer = types::GLuint;
pub type Fence = types::GLsync;
pub type Framebuffer = types::GLuint;
pub type Program = types::GLuint;
pub type Query = types::GLuint;
pub type Renderbuffer = types::GLuint;
pub type Sampler = types::GLuint;
pub type Shader = types::GLuint;
pub type Texture = types::GLuint;
pub type TransformFeedback = types::GLuint;
pub type UniformLocation = types::GLint;
pub type VertexArray = types::GLuint;

pub type BufferName = types::GLuint;
pub type FenceName = types::GLsync;
pub type FramebufferName = types::GLuint;
pub type ProgramName = types::GLuint;
pub type QueryName = types::GLuint;
pub type RenderbufferName = types::GLuint;
pub type SamplerName = types::GLuint;
pub type ShaderName = types::GLuint;
pub type TextureName = types::GLuint;
pub type TransformFeedbackName = types::GLuint;
pub type UniformLocationName = types::GLint;
pub type VertexArrayName = types::GLuint;

pub struct Context {
    gl: Gl,
}

extern "system" fn tinygl_debug_message_callback<F>(
    source: u32,
    message_type: u32,
    id: u32,
    severity: u32,
    length: i32,
    message: *const i8,
    user_param: *mut std::ffi::c_void,
) where
    F: FnMut(u32, u32, u32, u32, &std::ffi::CStr),
{
    unsafe {
        let callback_ptr = user_param as *mut F;
        let callback = &mut *callback_ptr;

        let message = &std::ffi::CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(
            message as *const _,
            length as usize,
        ));

        callback(source, message_type, id, severity, message);
    }
}

#[cfg(feature = "log-backtrace")]
fn fmt_backtrace() -> String {
    format!(", stack backtrace:\n{:?}", backtrace::Backtrace::new())
}

#[cfg(not(feature = "log-backtrace"))]
fn fmt_backtrace() -> String {
    String::new()
}

impl Context {
    pub unsafe fn from_loader_function<F>(loader_function: F) -> Self
    where
        F: FnMut(&str) -> *const std::os::raw::c_void + Clone,
    {
        let gl = Self {
            gl: Gl::load_with(loader_function),
        };

        // Setup logging on the context
        gl.debug_message_callback(|source, message_type, id, severity, message| {
            let source = match source {
                DEBUG_SOURCE_API => "opengl::api",
                DEBUG_SOURCE_WINDOW_SYSTEM => "opengl::window_system",
                DEBUG_SOURCE_SHADER_COMPILER => "opengl::shader_compiler",
                DEBUG_SOURCE_THIRD_PARTY => "opengl::third_party",
                DEBUG_SOURCE_APPLICATION => "opengl::application",
                DEBUG_SOURCE_OTHER => "opengl::other",
                _ => "opengl::unknown",
            };

            let level = match severity {
                DEBUG_SEVERITY_HIGH => log::Level::Error,
                DEBUG_SEVERITY_MEDIUM => log::Level::Warn,
                DEBUG_SEVERITY_LOW => log::Level::Info,
                DEBUG_SEVERITY_NOTIFICATION => log::Level::Debug,
                _ => log::Level::Trace,
            };

            let message_type = match message_type {
                DEBUG_TYPE_ERROR => "error",
                DEBUG_TYPE_DEPRECATED_BEHAVIOR => "deprecated behavior",
                DEBUG_TYPE_UNDEFINED_BEHAVIOR => "undefined behavior",
                DEBUG_TYPE_PORTABILITY => "portability",
                DEBUG_TYPE_PERFORMANCE => "performance",
                DEBUG_TYPE_MARKER => "marker",
                DEBUG_TYPE_PUSH_GROUP => "push group",
                DEBUG_TYPE_POP_GROUP => "pop group",
                DEBUG_TYPE_OTHER => "other",
                _ => "unknown",
            };

            // Create record manually so we can override the module path
            log::logger().log(
                &log::Record::builder()
                    .args(format_args!(
                        "{} ({}): {}{}",
                        message_type,
                        id,
                        message.to_string_lossy(),
                        if level == log::Level::Warn || level == log::Level::Error {
                            fmt_backtrace()
                        } else {
                            "".to_owned()
                        }
                    ))
                    .level(level)
                    .target("opengl")
                    .module_path_static(Some(source))
                    .build(),
            );
        });

        gl
    }

    /// Set up a callback for debug messages from the OpenGL driver
    pub unsafe fn debug_message_callback<F>(&self, callback: F)
    where
        F: FnMut(u32, u32, u32, u32, &std::ffi::CStr) + 'static,
    {
        self.gl.debug_message_callback(
            Some(tinygl_debug_message_callback::<F>),
            Box::into_raw(Box::new(callback)) as *const std::ffi::c_void,
        );
    }

    pub unsafe fn check_last_error(&self) -> crate::Result<()> {
        match self.get_error() {
            NO_ERROR => Ok(()),
            other => Err(crate::Error::OpenGlError(crate::OpenGlErrorCode(other))),
        }
    }

    pub unsafe fn get_shader_compile_status(&self, shader: Shader) -> bool {
        let mut status = 0;
        self.gl.get_shaderiv(shader, COMPILE_STATUS, &mut status);
        status == TRUE as i32
    }

    pub unsafe fn get_shader_info_log(&self, shader: Shader) -> Option<String> {
        // Get log length
        let mut length = 0;
        self.gl.get_shaderiv(shader, INFO_LOG_LENGTH, &mut length);

        if length == 0 {
            return None;
        }

        // Allocate buffer
        let mut info_log = vec![0u8; length as usize];

        // Fetch string
        self.gl.get_shader_info_log(
            shader,
            info_log.len() as i32,
            &mut length,
            info_log.as_mut_ptr() as *mut i8,
        );

        // Return string
        Some(String::from_utf8_lossy(&info_log[..length as usize]).to_string())
    }

    pub unsafe fn get_program_link_status(&self, program: Program) -> bool {
        let mut status = 0;
        self.gl.get_programiv(program, LINK_STATUS, &mut status);
        status == TRUE as i32
    }

    pub unsafe fn get_program_info_log(&self, program: Program) -> Option<String> {
        // Get log length
        let mut length = 0;
        self.gl.get_programiv(program, INFO_LOG_LENGTH, &mut length);

        if length == 0 {
            return None;
        }

        // Allocate buffer
        let mut info_log = vec![0u8; length as usize];

        // Fetch string
        self.gl.get_program_info_log(
            program,
            info_log.len() as i32,
            &mut length,
            info_log.as_mut_ptr() as *mut i8,
        );

        // Return string
        Some(String::from_utf8_lossy(&info_log[..length as usize]).to_string())
    }

    pub unsafe fn bind_buffer(&self, target: u32, buffer: Option<&wrappers::Buffer>) {
        self.gl
            .bind_buffer(target, buffer.map(|t| t.name()).unwrap_or(0));
    }

    pub unsafe fn bind_framebuffer(
        &self,
        target: u32,
        framebuffer: Option<&wrappers::Framebuffer>,
    ) {
        self.gl
            .bind_framebuffer(target, framebuffer.map(|t| t.name()).unwrap_or(0));
    }

    pub unsafe fn bind_renderbuffer(
        &self,
        target: u32,
        renderbuffer: Option<&wrappers::Renderbuffer>,
    ) {
        self.gl
            .bind_renderbuffer(target, renderbuffer.map(|t| t.name()).unwrap_or(0));
    }

    pub unsafe fn bind_texture(&self, target: u32, texture: Option<&wrappers::Texture>) {
        self.gl
            .bind_texture(target, texture.map(|t| t.name()).unwrap_or(0));
    }

    pub unsafe fn bind_vertex_array(&self, vertex_array: Option<&wrappers::VertexArray>) {
        self.gl
            .bind_vertex_array(vertex_array.map(|t| t.name()).unwrap_or(0));
    }

    pub unsafe fn create_program(&self) -> Option<Program> {
        let name = self.gl.create_program();
        if name == 0 {
            None
        } else {
            Some(name)
        }
    }

    pub unsafe fn create_shader(&self, kind: u32) -> Option<Shader> {
        let name = self.gl.create_shader(kind);
        if name == 0 {
            None
        } else {
            Some(name)
        }
    }

    pub unsafe fn use_program(&self, program: Option<ProgramName>) {
        self.gl.use_program(program.unwrap_or(0));
    }

    pub unsafe fn tex_image_1d(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLint,
        width: types::GLsizei,
        border: types::GLint,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: Option<&[u8]>,
    ) {
        self.gl.tex_image_1d(
            target,
            level,
            internalformat,
            width,
            border,
            format,
            type_,
            pixels
                .map(|p| p.as_ptr())
                .unwrap_or_else(|| std::ptr::null()) as *const _,
        );
    }

    pub unsafe fn tex_image_2d(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
        border: types::GLint,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: Option<&[u8]>,
    ) {
        self.gl.tex_image_2d(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            type_,
            pixels
                .map(|p| p.as_ptr())
                .unwrap_or_else(|| std::ptr::null()) as *const _,
        );
    }

    pub unsafe fn tex_image_3d(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
        depth: types::GLsizei,
        border: types::GLint,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: Option<&[u8]>,
    ) {
        self.gl.tex_image_3d(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            type_,
            pixels
                .map(|p| p.as_ptr())
                .unwrap_or_else(|| std::ptr::null()) as *const _,
        );
    }

    pub unsafe fn framebuffer_texture(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        texture: Option<&wrappers::Texture>,
        level: types::GLint,
    ) {
        self.gl.framebuffer_texture(
            target,
            attachment,
            texture.map(wrappers::Texture::name).unwrap_or(0),
            level,
        )
    }

    pub unsafe fn framebuffer_renderbuffer(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        renderbuffertarget: types::GLenum,
        renderbuffer: Option<&wrappers::Renderbuffer>,
    ) {
        self.gl.framebuffer_renderbuffer(
            target,
            attachment,
            renderbuffertarget,
            renderbuffer.map(wrappers::Renderbuffer::name).unwrap_or(0),
        )
    }

    pub unsafe fn framebuffer_texture_1d(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        textarget: types::GLenum,
        texture: Option<&wrappers::Texture>,
        level: types::GLint,
    ) {
        self.gl.framebuffer_texture_1d(
            target,
            attachment,
            textarget,
            texture.map(wrappers::Texture::name).unwrap_or(0),
            level,
        )
    }

    pub unsafe fn framebuffer_texture_2d(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        textarget: types::GLenum,
        texture: Option<&wrappers::Texture>,
        level: types::GLint,
    ) {
        self.gl.framebuffer_texture_2d(
            target,
            attachment,
            textarget,
            texture.map(wrappers::Texture::name).unwrap_or(0),
            level,
        );
    }

    pub unsafe fn framebuffer_texture_3d(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        textarget: types::GLenum,
        texture: Option<&wrappers::Texture>,
        level: types::GLint,
        zoffset: types::GLint,
    ) {
        self.gl.framebuffer_texture_3d(
            target,
            attachment,
            textarget,
            texture.map(wrappers::Texture::name).unwrap_or(0),
            level,
            zoffset,
        );
    }

    pub unsafe fn bind_image_texture(
        &self,
        unit: types::GLuint,
        texture: Option<&wrappers::Texture>,
        level: types::GLint,
        layered: bool,
        layer: types::GLint,
        access: types::GLenum,
        format: types::GLenum,
    ) {
        self.gl.bind_image_texture(
            unit,
            texture.map(wrappers::Texture::name).unwrap_or(0),
            level,
            layered as u8,
            layer,
            access,
            format,
        );
    }

    pub unsafe fn get_buffer_sub_data(&self, target: u32, offset: i32, dst_data: &mut [u8]) {
        self.gl.get_buffer_sub_data(
            target,
            offset as isize,
            dst_data.len() as isize,
            dst_data.as_mut_ptr() as *mut std::ffi::c_void,
        );
    }

    pub unsafe fn buffer_data_u8_slice(&self, target: u32, data: &[u8], usage: u32) {
        self.gl.buffer_data(
            target,
            data.len() as isize,
            data.as_ptr() as *const std::ffi::c_void,
            usage,
        );
    }

    pub unsafe fn buffer_data_size(&self, target: u32, size: i32, usage: u32) {
        self.gl
            .buffer_data(target, size as isize, std::ptr::null(), usage);
    }

    pub unsafe fn get_tex_image_u8_slice(
        &self,
        target: u32,
        level: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        self.gl.get_tex_image(
            target,
            level,
            format,
            ty,
            pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *mut std::ffi::c_void,
        );
    }

    pub unsafe fn draw_buffers(&self, buffers: &[u32]) {
        self.gl.draw_buffers(buffers.len() as i32, buffers.as_ptr());
    }

    pub unsafe fn depth_mask(&self, flag: bool) {
        self.gl.depth_mask(flag as u8);
    }

    pub unsafe fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        self.gl
            .color_mask(red as u8, green as u8, blue as u8, alpha as u8);
    }

    pub unsafe fn draw_elements(
        &self,
        mode: types::GLenum,
        count: types::GLsizei,
        type_: types::GLenum,
        indices: usize,
    ) {
        self.gl
            .draw_elements(mode, count, type_, indices as *const _);
    }

    pub unsafe fn buffer_storage(
        &self,
        target: types::GLenum,
        size: types::GLsizeiptr,
        data: Option<&[u8]>,
        flags: types::GLbitfield,
    ) {
        self.gl.buffer_storage(
            target,
            size,
            data.map(|d| d.as_ptr()).unwrap_or_else(|| std::ptr::null()) as *const _,
            flags,
        );
    }

    pub unsafe fn vertex_attrib_pointer_f32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        self.gl.vertex_attrib_pointer(
            index,
            size,
            data_type,
            normalized as u8,
            stride,
            offset as *const std::ffi::c_void,
        );
    }

    pub unsafe fn vertex_attrib_pointer_i32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        self.gl.vertex_attrib_i_pointer(
            index,
            size,
            data_type,
            stride,
            offset as *const std::ffi::c_void,
        );
    }

    pub unsafe fn vertex_attrib_pointer_f64(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        self.gl.vertex_attrib_l_pointer(
            index,
            size,
            data_type,
            stride,
            offset as *const std::ffi::c_void,
        );
    }
}

impl std::ops::Deref for Context {
    type Target = Gl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl std::ops::DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gl
    }
}
