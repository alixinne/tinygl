#[derive(Clone)]
pub struct UniformBlock {
    program: crate::gl::Program,
    index: u32,
    size: u32,
}

#[derive(Debug, Default, Clone)]
pub struct UniformDesc {
    pub index: u32,
    pub ty: u32,
    pub offset: usize,
    pub size: usize,
    pub name: String,
}

impl UniformBlock {
    pub fn new(context: &crate::Context, program: crate::gl::Program, index: u32) -> Option<Self> {
        if index == crate::gl::INVALID_INDEX {
            None
        } else {
            let mut this = Self {
                program,
                index,
                size: 0,
            };
            this.size = this.get_size(context);
            Some(this)
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn size(&self) -> usize {
        self.size as _
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_size(&self, context: &crate::Context) -> u32 {
        unsafe {
            let mut result = 0i32;
            context.get_active_uniform_blockiv(
                self.program,
                self.index,
                crate::gl::UNIFORM_BLOCK_DATA_SIZE,
                &mut result,
            );
            result as _
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn get_size(&self, context: &crate::Context) -> u32 {
        context
            .get_active_uniform_block_parameter(
                &self.program,
                self.index,
                crate::gl::UNIFORM_BLOCK_DATA_SIZE,
            )
            .expect("failed to get UNIFORM_BLOCK_DATA_SIZE")
            .as_f64()
            .unwrap() as _
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn block_binding(&self, context: &crate::Context, binding: u32) {
        unsafe { context.uniform_block_binding(self.program, self.index, binding) }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn block_binding(&self, context: &crate::Context, binding: u32) {
        context.uniform_block_binding(&self.program, self.index, binding)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_uniforms(&self, context: &crate::Context) -> Vec<UniformDesc> {
        unsafe {
            // Get the number of active uniforms
            let mut active_uniforms = 0i32;
            context.get_active_uniform_blockiv(
                self.program,
                self.index,
                crate::gl::UNIFORM_BLOCK_ACTIVE_UNIFORMS,
                &mut active_uniforms,
            );

            // Allocate space for the indices
            let mut uniform_indices = vec![0i32; active_uniforms as usize];

            // Get the indices
            context.get_active_uniform_blockiv(
                self.program,
                self.index,
                crate::gl::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES,
                uniform_indices.as_mut_ptr(),
            );

            // Allocate space for the param values
            let mut uniform_values = vec![0i32; active_uniforms as usize];

            // Create vector for the resulting descs
            let mut result = Vec::with_capacity(active_uniforms as usize);

            // Get uniform types
            context.get_active_uniformsiv(
                self.program,
                active_uniforms,
                uniform_indices.as_ptr() as _,
                crate::gl::UNIFORM_TYPE,
                uniform_values.as_mut_ptr(),
            );

            // For the first round, we need to allocate the desc structs
            for ty in uniform_values.iter() {
                result.push(UniformDesc {
                    ty: *ty as _,
                    ..Default::default()
                });
            }

            // Get uniform offsets
            context.get_active_uniformsiv(
                self.program,
                active_uniforms,
                uniform_indices.as_ptr() as _,
                crate::gl::UNIFORM_OFFSET,
                uniform_values.as_mut_ptr(),
            );

            for (offset, desc) in uniform_values.iter().zip(result.iter_mut()) {
                desc.offset = *offset as _;
            }

            // Get uniform sizes
            context.get_active_uniformsiv(
                self.program,
                active_uniforms,
                uniform_indices.as_ptr() as _,
                crate::gl::UNIFORM_SIZE,
                uniform_values.as_mut_ptr(),
            );

            for (size, desc) in uniform_values.iter().zip(result.iter_mut()) {
                desc.size = *size as _;
            }

            // Fetch name lengths
            context.get_active_uniformsiv(
                self.program,
                active_uniforms,
                uniform_indices.as_ptr() as _,
                crate::gl::UNIFORM_NAME_LENGTH,
                uniform_values.as_mut_ptr(),
            );

            // Maximum size of names
            let mut buf = vec![0u8; uniform_values.iter().copied().max().unwrap_or(0) as usize];
            for (index, desc) in uniform_indices.iter().zip(result.iter_mut()) {
                let mut len = 0;

                desc.index = *index as _;

                context.get_active_uniform_name(
                    self.program,
                    *index as _,
                    buf.len() as _,
                    &mut len,
                    buf.as_mut_ptr() as _,
                );

                desc.name = String::from_utf8_lossy(&buf[..len as usize]).to_string();
            }

            result
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_uniforms(&self, context: &crate::Context) -> Vec<UniformDesc> {
        use wasm_bindgen::JsCast;

        // Get the indices
        let uniform_indices = context
            .get_active_uniform_block_parameter(
                &self.program,
                self.index,
                crate::gl::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES,
            )
            .expect("failed to get uniform indices")
            .unchecked_into::<js_sys::Uint32Array>();

        // Create vector for the resulting descs
        let mut result = Vec::with_capacity(uniform_indices.length() as usize);

        // Get the uniform parameters
        let types = context
            .get_active_uniforms(&self.program, &uniform_indices, crate::gl::UNIFORM_TYPE)
            .unchecked_into::<js_sys::Array>();

        let offsets = context
            .get_active_uniforms(&self.program, &uniform_indices, crate::gl::UNIFORM_OFFSET)
            .unchecked_into::<js_sys::Array>();

        let sizes = context
            .get_active_uniforms(&self.program, &uniform_indices, crate::gl::UNIFORM_SIZE)
            .unchecked_into::<js_sys::Array>();

        for (((index, ty), offset), size) in uniform_indices
            .to_vec()
            .iter()
            .zip(types.values())
            .zip(offsets.values())
            .zip(sizes.values())
        {
            let info = context.get_active_uniform(&self.program, *index);

            result.push(UniformDesc {
                index: *index,
                ty: ty.unwrap().as_f64().unwrap() as u32,
                offset: offset.unwrap().as_f64().unwrap() as _,
                size: size.unwrap().as_f64().unwrap() as _,
                name: info
                    .map(|info| info.name().to_string())
                    .unwrap_or_else(String::new),
            });
        }

        result
    }
}
