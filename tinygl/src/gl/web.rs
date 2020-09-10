use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlQuery,
    WebGlRenderbuffer, WebGlSampler, WebGlShader, WebGlSync, WebGlTexture, WebGlTransformFeedback,
    WebGlUniformLocation, WebGlVertexArrayObject,
};

use crate::wrappers;

// Import constants
pub const READ_BUFFER: u32 = WebGl2RenderingContext::READ_BUFFER;
pub const UNPACK_ROW_LENGTH: u32 = WebGl2RenderingContext::UNPACK_ROW_LENGTH;
pub const UNPACK_SKIP_ROWS: u32 = WebGl2RenderingContext::UNPACK_SKIP_ROWS;
pub const UNPACK_SKIP_PIXELS: u32 = WebGl2RenderingContext::UNPACK_SKIP_PIXELS;
pub const PACK_ROW_LENGTH: u32 = WebGl2RenderingContext::PACK_ROW_LENGTH;
pub const PACK_SKIP_ROWS: u32 = WebGl2RenderingContext::PACK_SKIP_ROWS;
pub const PACK_SKIP_PIXELS: u32 = WebGl2RenderingContext::PACK_SKIP_PIXELS;
pub const COLOR: u32 = WebGl2RenderingContext::COLOR;
pub const DEPTH: u32 = WebGl2RenderingContext::DEPTH;
pub const STENCIL: u32 = WebGl2RenderingContext::STENCIL;
pub const RED: u32 = WebGl2RenderingContext::RED;
pub const RGB8: u32 = WebGl2RenderingContext::RGB8;
pub const RGBA8: u32 = WebGl2RenderingContext::RGBA8;
pub const RGB10_A2: u32 = WebGl2RenderingContext::RGB10_A2;
pub const TEXTURE_BINDING_3D: u32 = WebGl2RenderingContext::TEXTURE_BINDING_3D;
pub const UNPACK_SKIP_IMAGES: u32 = WebGl2RenderingContext::UNPACK_SKIP_IMAGES;
pub const UNPACK_IMAGE_HEIGHT: u32 = WebGl2RenderingContext::UNPACK_IMAGE_HEIGHT;
pub const TEXTURE_3D: u32 = WebGl2RenderingContext::TEXTURE_3D;
pub const TEXTURE_WRAP_R: u32 = WebGl2RenderingContext::TEXTURE_WRAP_R;
pub const MAX_3D_TEXTURE_SIZE: u32 = WebGl2RenderingContext::MAX_3D_TEXTURE_SIZE;
pub const UNSIGNED_INT_2_10_10_10_REV: u32 = WebGl2RenderingContext::UNSIGNED_INT_2_10_10_10_REV;
pub const MAX_ELEMENTS_VERTICES: u32 = WebGl2RenderingContext::MAX_ELEMENTS_VERTICES;
pub const MAX_ELEMENTS_INDICES: u32 = WebGl2RenderingContext::MAX_ELEMENTS_INDICES;
pub const TEXTURE_MIN_LOD: u32 = WebGl2RenderingContext::TEXTURE_MIN_LOD;
pub const TEXTURE_MAX_LOD: u32 = WebGl2RenderingContext::TEXTURE_MAX_LOD;
pub const TEXTURE_BASE_LEVEL: u32 = WebGl2RenderingContext::TEXTURE_BASE_LEVEL;
pub const TEXTURE_MAX_LEVEL: u32 = WebGl2RenderingContext::TEXTURE_MAX_LEVEL;
pub const MIN: u32 = WebGl2RenderingContext::MIN;
pub const MAX: u32 = WebGl2RenderingContext::MAX;
pub const DEPTH_COMPONENT24: u32 = WebGl2RenderingContext::DEPTH_COMPONENT24;
pub const MAX_TEXTURE_LOD_BIAS: u32 = WebGl2RenderingContext::MAX_TEXTURE_LOD_BIAS;
pub const TEXTURE_COMPARE_MODE: u32 = WebGl2RenderingContext::TEXTURE_COMPARE_MODE;
pub const TEXTURE_COMPARE_FUNC: u32 = WebGl2RenderingContext::TEXTURE_COMPARE_FUNC;
pub const CURRENT_QUERY: u32 = WebGl2RenderingContext::CURRENT_QUERY;
pub const QUERY_RESULT: u32 = WebGl2RenderingContext::QUERY_RESULT;
pub const QUERY_RESULT_AVAILABLE: u32 = WebGl2RenderingContext::QUERY_RESULT_AVAILABLE;
pub const STREAM_READ: u32 = WebGl2RenderingContext::STREAM_READ;
pub const STREAM_COPY: u32 = WebGl2RenderingContext::STREAM_COPY;
pub const STATIC_READ: u32 = WebGl2RenderingContext::STATIC_READ;
pub const STATIC_COPY: u32 = WebGl2RenderingContext::STATIC_COPY;
pub const DYNAMIC_READ: u32 = WebGl2RenderingContext::DYNAMIC_READ;
pub const DYNAMIC_COPY: u32 = WebGl2RenderingContext::DYNAMIC_COPY;
pub const MAX_DRAW_BUFFERS: u32 = WebGl2RenderingContext::MAX_DRAW_BUFFERS;
pub const DRAW_BUFFER0: u32 = WebGl2RenderingContext::DRAW_BUFFER0;
pub const DRAW_BUFFER1: u32 = WebGl2RenderingContext::DRAW_BUFFER1;
pub const DRAW_BUFFER2: u32 = WebGl2RenderingContext::DRAW_BUFFER2;
pub const DRAW_BUFFER3: u32 = WebGl2RenderingContext::DRAW_BUFFER3;
pub const DRAW_BUFFER4: u32 = WebGl2RenderingContext::DRAW_BUFFER4;
pub const DRAW_BUFFER5: u32 = WebGl2RenderingContext::DRAW_BUFFER5;
pub const DRAW_BUFFER6: u32 = WebGl2RenderingContext::DRAW_BUFFER6;
pub const DRAW_BUFFER7: u32 = WebGl2RenderingContext::DRAW_BUFFER7;
pub const DRAW_BUFFER8: u32 = WebGl2RenderingContext::DRAW_BUFFER8;
pub const DRAW_BUFFER9: u32 = WebGl2RenderingContext::DRAW_BUFFER9;
pub const DRAW_BUFFER10: u32 = WebGl2RenderingContext::DRAW_BUFFER10;
pub const DRAW_BUFFER11: u32 = WebGl2RenderingContext::DRAW_BUFFER11;
pub const DRAW_BUFFER12: u32 = WebGl2RenderingContext::DRAW_BUFFER12;
pub const DRAW_BUFFER13: u32 = WebGl2RenderingContext::DRAW_BUFFER13;
pub const DRAW_BUFFER14: u32 = WebGl2RenderingContext::DRAW_BUFFER14;
pub const DRAW_BUFFER15: u32 = WebGl2RenderingContext::DRAW_BUFFER15;
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: u32 =
    WebGl2RenderingContext::MAX_FRAGMENT_UNIFORM_COMPONENTS;
pub const MAX_VERTEX_UNIFORM_COMPONENTS: u32 =
    WebGl2RenderingContext::MAX_VERTEX_UNIFORM_COMPONENTS;
pub const SAMPLER_3D: u32 = WebGl2RenderingContext::SAMPLER_3D;
pub const SAMPLER_2D_SHADOW: u32 = WebGl2RenderingContext::SAMPLER_2D_SHADOW;
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: u32 =
    WebGl2RenderingContext::FRAGMENT_SHADER_DERIVATIVE_HINT;
pub const PIXEL_PACK_BUFFER: u32 = WebGl2RenderingContext::PIXEL_PACK_BUFFER;
pub const PIXEL_UNPACK_BUFFER: u32 = WebGl2RenderingContext::PIXEL_UNPACK_BUFFER;
pub const PIXEL_PACK_BUFFER_BINDING: u32 = WebGl2RenderingContext::PIXEL_PACK_BUFFER_BINDING;
pub const PIXEL_UNPACK_BUFFER_BINDING: u32 = WebGl2RenderingContext::PIXEL_UNPACK_BUFFER_BINDING;
pub const FLOAT_MAT2X3: u32 = WebGl2RenderingContext::FLOAT_MAT2X3;
pub const FLOAT_MAT2X4: u32 = WebGl2RenderingContext::FLOAT_MAT2X4;
pub const FLOAT_MAT3X2: u32 = WebGl2RenderingContext::FLOAT_MAT3X2;
pub const FLOAT_MAT3X4: u32 = WebGl2RenderingContext::FLOAT_MAT3X4;
pub const FLOAT_MAT4X2: u32 = WebGl2RenderingContext::FLOAT_MAT4X2;
pub const FLOAT_MAT4X3: u32 = WebGl2RenderingContext::FLOAT_MAT4X3;
pub const SRGB: u32 = WebGl2RenderingContext::SRGB;
pub const SRGB8: u32 = WebGl2RenderingContext::SRGB8;
pub const SRGB8_ALPHA8: u32 = WebGl2RenderingContext::SRGB8_ALPHA8;
pub const COMPARE_REF_TO_TEXTURE: u32 = WebGl2RenderingContext::COMPARE_REF_TO_TEXTURE;
pub const RGBA32F: u32 = WebGl2RenderingContext::RGBA32F;
pub const RGB32F: u32 = WebGl2RenderingContext::RGB32F;
pub const RGBA16F: u32 = WebGl2RenderingContext::RGBA16F;
pub const RGB16F: u32 = WebGl2RenderingContext::RGB16F;
pub const VERTEX_ATTRIB_ARRAY_INTEGER: u32 = WebGl2RenderingContext::VERTEX_ATTRIB_ARRAY_INTEGER;
pub const MAX_ARRAY_TEXTURE_LAYERS: u32 = WebGl2RenderingContext::MAX_ARRAY_TEXTURE_LAYERS;
pub const MIN_PROGRAM_TEXEL_OFFSET: u32 = WebGl2RenderingContext::MIN_PROGRAM_TEXEL_OFFSET;
pub const MAX_PROGRAM_TEXEL_OFFSET: u32 = WebGl2RenderingContext::MAX_PROGRAM_TEXEL_OFFSET;
pub const MAX_VARYING_COMPONENTS: u32 = WebGl2RenderingContext::MAX_VARYING_COMPONENTS;
pub const TEXTURE_2D_ARRAY: u32 = WebGl2RenderingContext::TEXTURE_2D_ARRAY;
pub const TEXTURE_BINDING_2D_ARRAY: u32 = WebGl2RenderingContext::TEXTURE_BINDING_2D_ARRAY;
pub const R11F_G11F_B10F: u32 = WebGl2RenderingContext::R11F_G11F_B10F;
pub const UNSIGNED_INT_10F_11F_11F_REV: u32 = WebGl2RenderingContext::UNSIGNED_INT_10F_11F_11F_REV;
pub const RGB9_E5: u32 = WebGl2RenderingContext::RGB9_E5;
pub const UNSIGNED_INT_5_9_9_9_REV: u32 = WebGl2RenderingContext::UNSIGNED_INT_5_9_9_9_REV;
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: u32 =
    WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER_MODE;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 =
    WebGl2RenderingContext::MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS;
pub const TRANSFORM_FEEDBACK_VARYINGS: u32 = WebGl2RenderingContext::TRANSFORM_FEEDBACK_VARYINGS;
pub const TRANSFORM_FEEDBACK_BUFFER_START: u32 =
    WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER_START;
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: u32 =
    WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER_SIZE;
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: u32 =
    WebGl2RenderingContext::TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN;
pub const RASTERIZER_DISCARD: u32 = WebGl2RenderingContext::RASTERIZER_DISCARD;
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: u32 =
    WebGl2RenderingContext::MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: u32 =
    WebGl2RenderingContext::MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS;
pub const INTERLEAVED_ATTRIBS: u32 = WebGl2RenderingContext::INTERLEAVED_ATTRIBS;
pub const SEPARATE_ATTRIBS: u32 = WebGl2RenderingContext::SEPARATE_ATTRIBS;
pub const TRANSFORM_FEEDBACK_BUFFER: u32 = WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER;
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: u32 =
    WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER_BINDING;
pub const RGBA32UI: u32 = WebGl2RenderingContext::RGBA32UI;
pub const RGB32UI: u32 = WebGl2RenderingContext::RGB32UI;
pub const RGBA16UI: u32 = WebGl2RenderingContext::RGBA16UI;
pub const RGB16UI: u32 = WebGl2RenderingContext::RGB16UI;
pub const RGBA8UI: u32 = WebGl2RenderingContext::RGBA8UI;
pub const RGB8UI: u32 = WebGl2RenderingContext::RGB8UI;
pub const RGBA32I: u32 = WebGl2RenderingContext::RGBA32I;
pub const RGB32I: u32 = WebGl2RenderingContext::RGB32I;
pub const RGBA16I: u32 = WebGl2RenderingContext::RGBA16I;
pub const RGB16I: u32 = WebGl2RenderingContext::RGB16I;
pub const RGBA8I: u32 = WebGl2RenderingContext::RGBA8I;
pub const RGB8I: u32 = WebGl2RenderingContext::RGB8I;
pub const RED_INTEGER: u32 = WebGl2RenderingContext::RED_INTEGER;
pub const RGB_INTEGER: u32 = WebGl2RenderingContext::RGB_INTEGER;
pub const RGBA_INTEGER: u32 = WebGl2RenderingContext::RGBA_INTEGER;
pub const SAMPLER_2D_ARRAY: u32 = WebGl2RenderingContext::SAMPLER_2D_ARRAY;
pub const SAMPLER_2D_ARRAY_SHADOW: u32 = WebGl2RenderingContext::SAMPLER_2D_ARRAY_SHADOW;
pub const SAMPLER_CUBE_SHADOW: u32 = WebGl2RenderingContext::SAMPLER_CUBE_SHADOW;
pub const UNSIGNED_INT_VEC2: u32 = WebGl2RenderingContext::UNSIGNED_INT_VEC2;
pub const UNSIGNED_INT_VEC3: u32 = WebGl2RenderingContext::UNSIGNED_INT_VEC3;
pub const UNSIGNED_INT_VEC4: u32 = WebGl2RenderingContext::UNSIGNED_INT_VEC4;
pub const INT_SAMPLER_2D: u32 = WebGl2RenderingContext::INT_SAMPLER_2D;
pub const INT_SAMPLER_3D: u32 = WebGl2RenderingContext::INT_SAMPLER_3D;
pub const INT_SAMPLER_CUBE: u32 = WebGl2RenderingContext::INT_SAMPLER_CUBE;
pub const INT_SAMPLER_2D_ARRAY: u32 = WebGl2RenderingContext::INT_SAMPLER_2D_ARRAY;
pub const UNSIGNED_INT_SAMPLER_2D: u32 = WebGl2RenderingContext::UNSIGNED_INT_SAMPLER_2D;
pub const UNSIGNED_INT_SAMPLER_3D: u32 = WebGl2RenderingContext::UNSIGNED_INT_SAMPLER_3D;
pub const UNSIGNED_INT_SAMPLER_CUBE: u32 = WebGl2RenderingContext::UNSIGNED_INT_SAMPLER_CUBE;
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: u32 =
    WebGl2RenderingContext::UNSIGNED_INT_SAMPLER_2D_ARRAY;
pub const DEPTH_COMPONENT32F: u32 = WebGl2RenderingContext::DEPTH_COMPONENT32F;
pub const DEPTH32F_STENCIL8: u32 = WebGl2RenderingContext::DEPTH32F_STENCIL8;
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: u32 =
    WebGl2RenderingContext::FLOAT_32_UNSIGNED_INT_24_8_REV;
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING;
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE;
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_RED_SIZE;
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_GREEN_SIZE;
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_BLUE_SIZE;
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE;
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE;
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE;
pub const FRAMEBUFFER_DEFAULT: u32 = WebGl2RenderingContext::FRAMEBUFFER_DEFAULT;
pub const UNSIGNED_INT_24_8: u32 = WebGl2RenderingContext::UNSIGNED_INT_24_8;
pub const DEPTH24_STENCIL8: u32 = WebGl2RenderingContext::DEPTH24_STENCIL8;
pub const UNSIGNED_NORMALIZED: u32 = WebGl2RenderingContext::UNSIGNED_NORMALIZED;
pub const DRAW_FRAMEBUFFER_BINDING: u32 = WebGl2RenderingContext::DRAW_FRAMEBUFFER_BINDING;
pub const READ_FRAMEBUFFER: u32 = WebGl2RenderingContext::READ_FRAMEBUFFER;
pub const DRAW_FRAMEBUFFER: u32 = WebGl2RenderingContext::DRAW_FRAMEBUFFER;
pub const READ_FRAMEBUFFER_BINDING: u32 = WebGl2RenderingContext::READ_FRAMEBUFFER_BINDING;
pub const RENDERBUFFER_SAMPLES: u32 = WebGl2RenderingContext::RENDERBUFFER_SAMPLES;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER;
pub const MAX_COLOR_ATTACHMENTS: u32 = WebGl2RenderingContext::MAX_COLOR_ATTACHMENTS;
pub const COLOR_ATTACHMENT1: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT1;
pub const COLOR_ATTACHMENT2: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT2;
pub const COLOR_ATTACHMENT3: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT3;
pub const COLOR_ATTACHMENT4: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT4;
pub const COLOR_ATTACHMENT5: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT5;
pub const COLOR_ATTACHMENT6: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT6;
pub const COLOR_ATTACHMENT7: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT7;
pub const COLOR_ATTACHMENT8: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT8;
pub const COLOR_ATTACHMENT9: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT9;
pub const COLOR_ATTACHMENT10: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT10;
pub const COLOR_ATTACHMENT11: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT11;
pub const COLOR_ATTACHMENT12: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT12;
pub const COLOR_ATTACHMENT13: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT13;
pub const COLOR_ATTACHMENT14: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT14;
pub const COLOR_ATTACHMENT15: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT15;
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE;
pub const MAX_SAMPLES: u32 = WebGl2RenderingContext::MAX_SAMPLES;
pub const HALF_FLOAT: u32 = WebGl2RenderingContext::HALF_FLOAT;
pub const RG: u32 = WebGl2RenderingContext::RG;
pub const RG_INTEGER: u32 = WebGl2RenderingContext::RG_INTEGER;
pub const R8: u32 = WebGl2RenderingContext::R8;
pub const RG8: u32 = WebGl2RenderingContext::RG8;
pub const R16F: u32 = WebGl2RenderingContext::R16F;
pub const R32F: u32 = WebGl2RenderingContext::R32F;
pub const RG16F: u32 = WebGl2RenderingContext::RG16F;
pub const RG32F: u32 = WebGl2RenderingContext::RG32F;
pub const R8I: u32 = WebGl2RenderingContext::R8I;
pub const R8UI: u32 = WebGl2RenderingContext::R8UI;
pub const R16I: u32 = WebGl2RenderingContext::R16I;
pub const R16UI: u32 = WebGl2RenderingContext::R16UI;
pub const R32I: u32 = WebGl2RenderingContext::R32I;
pub const R32UI: u32 = WebGl2RenderingContext::R32UI;
pub const RG8I: u32 = WebGl2RenderingContext::RG8I;
pub const RG8UI: u32 = WebGl2RenderingContext::RG8UI;
pub const RG16I: u32 = WebGl2RenderingContext::RG16I;
pub const RG16UI: u32 = WebGl2RenderingContext::RG16UI;
pub const RG32I: u32 = WebGl2RenderingContext::RG32I;
pub const RG32UI: u32 = WebGl2RenderingContext::RG32UI;
pub const VERTEX_ARRAY_BINDING: u32 = WebGl2RenderingContext::VERTEX_ARRAY_BINDING;
pub const R8_SNORM: u32 = WebGl2RenderingContext::R8_SNORM;
pub const RG8_SNORM: u32 = WebGl2RenderingContext::RG8_SNORM;
pub const RGB8_SNORM: u32 = WebGl2RenderingContext::RGB8_SNORM;
pub const RGBA8_SNORM: u32 = WebGl2RenderingContext::RGBA8_SNORM;
pub const SIGNED_NORMALIZED: u32 = WebGl2RenderingContext::SIGNED_NORMALIZED;
pub const COPY_READ_BUFFER: u32 = WebGl2RenderingContext::COPY_READ_BUFFER;
pub const COPY_WRITE_BUFFER: u32 = WebGl2RenderingContext::COPY_WRITE_BUFFER;
pub const COPY_READ_BUFFER_BINDING: u32 = WebGl2RenderingContext::COPY_READ_BUFFER_BINDING;
pub const COPY_WRITE_BUFFER_BINDING: u32 = WebGl2RenderingContext::COPY_WRITE_BUFFER_BINDING;
pub const UNIFORM_BUFFER: u32 = WebGl2RenderingContext::UNIFORM_BUFFER;
pub const UNIFORM_BUFFER_BINDING: u32 = WebGl2RenderingContext::UNIFORM_BUFFER_BINDING;
pub const UNIFORM_BUFFER_START: u32 = WebGl2RenderingContext::UNIFORM_BUFFER_START;
pub const UNIFORM_BUFFER_SIZE: u32 = WebGl2RenderingContext::UNIFORM_BUFFER_SIZE;
pub const MAX_VERTEX_UNIFORM_BLOCKS: u32 = WebGl2RenderingContext::MAX_VERTEX_UNIFORM_BLOCKS;
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: u32 = WebGl2RenderingContext::MAX_FRAGMENT_UNIFORM_BLOCKS;
pub const MAX_COMBINED_UNIFORM_BLOCKS: u32 = WebGl2RenderingContext::MAX_COMBINED_UNIFORM_BLOCKS;
pub const MAX_UNIFORM_BUFFER_BINDINGS: u32 = WebGl2RenderingContext::MAX_UNIFORM_BUFFER_BINDINGS;
pub const MAX_UNIFORM_BLOCK_SIZE: u32 = WebGl2RenderingContext::MAX_UNIFORM_BLOCK_SIZE;
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: u32 =
    WebGl2RenderingContext::MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS;
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: u32 =
    WebGl2RenderingContext::MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS;
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: u32 =
    WebGl2RenderingContext::UNIFORM_BUFFER_OFFSET_ALIGNMENT;
pub const ACTIVE_UNIFORM_BLOCKS: u32 = WebGl2RenderingContext::ACTIVE_UNIFORM_BLOCKS;
pub const UNIFORM_TYPE: u32 = WebGl2RenderingContext::UNIFORM_TYPE;
pub const UNIFORM_SIZE: u32 = WebGl2RenderingContext::UNIFORM_SIZE;
pub const UNIFORM_BLOCK_INDEX: u32 = WebGl2RenderingContext::UNIFORM_BLOCK_INDEX;
pub const UNIFORM_OFFSET: u32 = WebGl2RenderingContext::UNIFORM_OFFSET;
pub const UNIFORM_ARRAY_STRIDE: u32 = WebGl2RenderingContext::UNIFORM_ARRAY_STRIDE;
pub const UNIFORM_MATRIX_STRIDE: u32 = WebGl2RenderingContext::UNIFORM_MATRIX_STRIDE;
pub const UNIFORM_IS_ROW_MAJOR: u32 = WebGl2RenderingContext::UNIFORM_IS_ROW_MAJOR;
pub const UNIFORM_BLOCK_BINDING: u32 = WebGl2RenderingContext::UNIFORM_BLOCK_BINDING;
pub const UNIFORM_BLOCK_DATA_SIZE: u32 = WebGl2RenderingContext::UNIFORM_BLOCK_DATA_SIZE;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: u32 =
    WebGl2RenderingContext::UNIFORM_BLOCK_ACTIVE_UNIFORMS;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: u32 =
    WebGl2RenderingContext::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES;
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: u32 =
    WebGl2RenderingContext::UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER;
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: u32 =
    WebGl2RenderingContext::UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER;
pub const INVALID_INDEX: u32 = WebGl2RenderingContext::INVALID_INDEX;
pub const MAX_VERTEX_OUTPUT_COMPONENTS: u32 = WebGl2RenderingContext::MAX_VERTEX_OUTPUT_COMPONENTS;
pub const MAX_FRAGMENT_INPUT_COMPONENTS: u32 =
    WebGl2RenderingContext::MAX_FRAGMENT_INPUT_COMPONENTS;
pub const MAX_SERVER_WAIT_TIMEOUT: u32 = WebGl2RenderingContext::MAX_SERVER_WAIT_TIMEOUT;
pub const OBJECT_TYPE: u32 = WebGl2RenderingContext::OBJECT_TYPE;
pub const SYNC_CONDITION: u32 = WebGl2RenderingContext::SYNC_CONDITION;
pub const SYNC_STATUS: u32 = WebGl2RenderingContext::SYNC_STATUS;
pub const SYNC_FLAGS: u32 = WebGl2RenderingContext::SYNC_FLAGS;
pub const SYNC_FENCE: u32 = WebGl2RenderingContext::SYNC_FENCE;
pub const SYNC_GPU_COMMANDS_COMPLETE: u32 = WebGl2RenderingContext::SYNC_GPU_COMMANDS_COMPLETE;
pub const UNSIGNALED: u32 = WebGl2RenderingContext::UNSIGNALED;
pub const SIGNALED: u32 = WebGl2RenderingContext::SIGNALED;
pub const ALREADY_SIGNALED: u32 = WebGl2RenderingContext::ALREADY_SIGNALED;
pub const TIMEOUT_EXPIRED: u32 = WebGl2RenderingContext::TIMEOUT_EXPIRED;
pub const CONDITION_SATISFIED: u32 = WebGl2RenderingContext::CONDITION_SATISFIED;
pub const WAIT_FAILED: u32 = WebGl2RenderingContext::WAIT_FAILED;
pub const SYNC_FLUSH_COMMANDS_BIT: u32 = WebGl2RenderingContext::SYNC_FLUSH_COMMANDS_BIT;
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: u32 = WebGl2RenderingContext::VERTEX_ATTRIB_ARRAY_DIVISOR;
pub const ANY_SAMPLES_PASSED: u32 = WebGl2RenderingContext::ANY_SAMPLES_PASSED;
pub const ANY_SAMPLES_PASSED_CONSERVATIVE: u32 =
    WebGl2RenderingContext::ANY_SAMPLES_PASSED_CONSERVATIVE;
pub const SAMPLER_BINDING: u32 = WebGl2RenderingContext::SAMPLER_BINDING;
pub const RGB10_A2UI: u32 = WebGl2RenderingContext::RGB10_A2UI;
pub const INT_2_10_10_10_REV: u32 = WebGl2RenderingContext::INT_2_10_10_10_REV;
pub const TRANSFORM_FEEDBACK: u32 = WebGl2RenderingContext::TRANSFORM_FEEDBACK;
pub const TRANSFORM_FEEDBACK_PAUSED: u32 = WebGl2RenderingContext::TRANSFORM_FEEDBACK_PAUSED;
pub const TRANSFORM_FEEDBACK_ACTIVE: u32 = WebGl2RenderingContext::TRANSFORM_FEEDBACK_ACTIVE;
pub const TRANSFORM_FEEDBACK_BINDING: u32 = WebGl2RenderingContext::TRANSFORM_FEEDBACK_BINDING;
pub const TEXTURE_IMMUTABLE_FORMAT: u32 = WebGl2RenderingContext::TEXTURE_IMMUTABLE_FORMAT;
pub const MAX_ELEMENT_INDEX: u32 = WebGl2RenderingContext::MAX_ELEMENT_INDEX;
pub const TEXTURE_IMMUTABLE_LEVELS: u32 = WebGl2RenderingContext::TEXTURE_IMMUTABLE_LEVELS;
pub const TIMEOUT_IGNORED: f64 = -1i64 as f64;
pub const MAX_CLIENT_WAIT_TIMEOUT_WEBGL: u32 =
    WebGl2RenderingContext::MAX_CLIENT_WAIT_TIMEOUT_WEBGL;
pub const DEPTH_BUFFER_BIT: u32 = WebGl2RenderingContext::DEPTH_BUFFER_BIT;
pub const STENCIL_BUFFER_BIT: u32 = WebGl2RenderingContext::STENCIL_BUFFER_BIT;
pub const COLOR_BUFFER_BIT: u32 = WebGl2RenderingContext::COLOR_BUFFER_BIT;
pub const POINTS: u32 = WebGl2RenderingContext::POINTS;
pub const LINES: u32 = WebGl2RenderingContext::LINES;
pub const LINE_LOOP: u32 = WebGl2RenderingContext::LINE_LOOP;
pub const LINE_STRIP: u32 = WebGl2RenderingContext::LINE_STRIP;
pub const TRIANGLES: u32 = WebGl2RenderingContext::TRIANGLES;
pub const TRIANGLE_STRIP: u32 = WebGl2RenderingContext::TRIANGLE_STRIP;
pub const TRIANGLE_FAN: u32 = WebGl2RenderingContext::TRIANGLE_FAN;
pub const ZERO: u32 = WebGl2RenderingContext::ZERO;
pub const ONE: u32 = WebGl2RenderingContext::ONE;
pub const SRC_COLOR: u32 = WebGl2RenderingContext::SRC_COLOR;
pub const ONE_MINUS_SRC_COLOR: u32 = WebGl2RenderingContext::ONE_MINUS_SRC_COLOR;
pub const SRC_ALPHA: u32 = WebGl2RenderingContext::SRC_ALPHA;
pub const ONE_MINUS_SRC_ALPHA: u32 = WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA;
pub const DST_ALPHA: u32 = WebGl2RenderingContext::DST_ALPHA;
pub const ONE_MINUS_DST_ALPHA: u32 = WebGl2RenderingContext::ONE_MINUS_DST_ALPHA;
pub const DST_COLOR: u32 = WebGl2RenderingContext::DST_COLOR;
pub const ONE_MINUS_DST_COLOR: u32 = WebGl2RenderingContext::ONE_MINUS_DST_COLOR;
pub const SRC_ALPHA_SATURATE: u32 = WebGl2RenderingContext::SRC_ALPHA_SATURATE;
pub const FUNC_ADD: u32 = WebGl2RenderingContext::FUNC_ADD;
pub const BLEND_EQUATION: u32 = WebGl2RenderingContext::BLEND_EQUATION;
pub const BLEND_EQUATION_RGB: u32 = WebGl2RenderingContext::BLEND_EQUATION_RGB;
pub const BLEND_EQUATION_ALPHA: u32 = WebGl2RenderingContext::BLEND_EQUATION_ALPHA;
pub const FUNC_SUBTRACT: u32 = WebGl2RenderingContext::FUNC_SUBTRACT;
pub const FUNC_REVERSE_SUBTRACT: u32 = WebGl2RenderingContext::FUNC_REVERSE_SUBTRACT;
pub const BLEND_DST_RGB: u32 = WebGl2RenderingContext::BLEND_DST_RGB;
pub const BLEND_SRC_RGB: u32 = WebGl2RenderingContext::BLEND_SRC_RGB;
pub const BLEND_DST_ALPHA: u32 = WebGl2RenderingContext::BLEND_DST_ALPHA;
pub const BLEND_SRC_ALPHA: u32 = WebGl2RenderingContext::BLEND_SRC_ALPHA;
pub const CONSTANT_COLOR: u32 = WebGl2RenderingContext::CONSTANT_COLOR;
pub const ONE_MINUS_CONSTANT_COLOR: u32 = WebGl2RenderingContext::ONE_MINUS_CONSTANT_COLOR;
pub const CONSTANT_ALPHA: u32 = WebGl2RenderingContext::CONSTANT_ALPHA;
pub const ONE_MINUS_CONSTANT_ALPHA: u32 = WebGl2RenderingContext::ONE_MINUS_CONSTANT_ALPHA;
pub const BLEND_COLOR: u32 = WebGl2RenderingContext::BLEND_COLOR;
pub const ARRAY_BUFFER: u32 = WebGl2RenderingContext::ARRAY_BUFFER;
pub const ELEMENT_ARRAY_BUFFER: u32 = WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER;
pub const ARRAY_BUFFER_BINDING: u32 = WebGl2RenderingContext::ARRAY_BUFFER_BINDING;
pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER_BINDING;
pub const STREAM_DRAW: u32 = WebGl2RenderingContext::STREAM_DRAW;
pub const STATIC_DRAW: u32 = WebGl2RenderingContext::STATIC_DRAW;
pub const DYNAMIC_DRAW: u32 = WebGl2RenderingContext::DYNAMIC_DRAW;
pub const BUFFER_SIZE: u32 = WebGl2RenderingContext::BUFFER_SIZE;
pub const BUFFER_USAGE: u32 = WebGl2RenderingContext::BUFFER_USAGE;
pub const CURRENT_VERTEX_ATTRIB: u32 = WebGl2RenderingContext::CURRENT_VERTEX_ATTRIB;
pub const FRONT: u32 = WebGl2RenderingContext::FRONT;
pub const BACK: u32 = WebGl2RenderingContext::BACK;
pub const FRONT_AND_BACK: u32 = WebGl2RenderingContext::FRONT_AND_BACK;
pub const CULL_FACE: u32 = WebGl2RenderingContext::CULL_FACE;
pub const BLEND: u32 = WebGl2RenderingContext::BLEND;
pub const DITHER: u32 = WebGl2RenderingContext::DITHER;
pub const STENCIL_TEST: u32 = WebGl2RenderingContext::STENCIL_TEST;
pub const DEPTH_TEST: u32 = WebGl2RenderingContext::DEPTH_TEST;
pub const SCISSOR_TEST: u32 = WebGl2RenderingContext::SCISSOR_TEST;
pub const POLYGON_OFFSET_FILL: u32 = WebGl2RenderingContext::POLYGON_OFFSET_FILL;
pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = WebGl2RenderingContext::SAMPLE_ALPHA_TO_COVERAGE;
pub const SAMPLE_COVERAGE: u32 = WebGl2RenderingContext::SAMPLE_COVERAGE;
pub const NO_ERROR: u32 = WebGl2RenderingContext::NO_ERROR;
pub const INVALID_ENUM: u32 = WebGl2RenderingContext::INVALID_ENUM;
pub const INVALID_VALUE: u32 = WebGl2RenderingContext::INVALID_VALUE;
pub const INVALID_OPERATION: u32 = WebGl2RenderingContext::INVALID_OPERATION;
pub const OUT_OF_MEMORY: u32 = WebGl2RenderingContext::OUT_OF_MEMORY;
pub const CW: u32 = WebGl2RenderingContext::CW;
pub const CCW: u32 = WebGl2RenderingContext::CCW;
pub const LINE_WIDTH: u32 = WebGl2RenderingContext::LINE_WIDTH;
pub const ALIASED_POINT_SIZE_RANGE: u32 = WebGl2RenderingContext::ALIASED_POINT_SIZE_RANGE;
pub const ALIASED_LINE_WIDTH_RANGE: u32 = WebGl2RenderingContext::ALIASED_LINE_WIDTH_RANGE;
pub const CULL_FACE_MODE: u32 = WebGl2RenderingContext::CULL_FACE_MODE;
pub const FRONT_FACE: u32 = WebGl2RenderingContext::FRONT_FACE;
pub const DEPTH_RANGE: u32 = WebGl2RenderingContext::DEPTH_RANGE;
pub const DEPTH_WRITEMASK: u32 = WebGl2RenderingContext::DEPTH_WRITEMASK;
pub const DEPTH_CLEAR_VALUE: u32 = WebGl2RenderingContext::DEPTH_CLEAR_VALUE;
pub const DEPTH_FUNC: u32 = WebGl2RenderingContext::DEPTH_FUNC;
pub const STENCIL_CLEAR_VALUE: u32 = WebGl2RenderingContext::STENCIL_CLEAR_VALUE;
pub const STENCIL_FUNC: u32 = WebGl2RenderingContext::STENCIL_FUNC;
pub const STENCIL_FAIL: u32 = WebGl2RenderingContext::STENCIL_FAIL;
pub const STENCIL_PASS_DEPTH_FAIL: u32 = WebGl2RenderingContext::STENCIL_PASS_DEPTH_FAIL;
pub const STENCIL_PASS_DEPTH_PASS: u32 = WebGl2RenderingContext::STENCIL_PASS_DEPTH_PASS;
pub const STENCIL_REF: u32 = WebGl2RenderingContext::STENCIL_REF;
pub const STENCIL_VALUE_MASK: u32 = WebGl2RenderingContext::STENCIL_VALUE_MASK;
pub const STENCIL_WRITEMASK: u32 = WebGl2RenderingContext::STENCIL_WRITEMASK;
pub const STENCIL_BACK_FUNC: u32 = WebGl2RenderingContext::STENCIL_BACK_FUNC;
pub const STENCIL_BACK_FAIL: u32 = WebGl2RenderingContext::STENCIL_BACK_FAIL;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = WebGl2RenderingContext::STENCIL_BACK_PASS_DEPTH_FAIL;
pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = WebGl2RenderingContext::STENCIL_BACK_PASS_DEPTH_PASS;
pub const STENCIL_BACK_REF: u32 = WebGl2RenderingContext::STENCIL_BACK_REF;
pub const STENCIL_BACK_VALUE_MASK: u32 = WebGl2RenderingContext::STENCIL_BACK_VALUE_MASK;
pub const STENCIL_BACK_WRITEMASK: u32 = WebGl2RenderingContext::STENCIL_BACK_WRITEMASK;
pub const VIEWPORT: u32 = WebGl2RenderingContext::VIEWPORT;
pub const SCISSOR_BOX: u32 = WebGl2RenderingContext::SCISSOR_BOX;
pub const COLOR_CLEAR_VALUE: u32 = WebGl2RenderingContext::COLOR_CLEAR_VALUE;
pub const COLOR_WRITEMASK: u32 = WebGl2RenderingContext::COLOR_WRITEMASK;
pub const UNPACK_ALIGNMENT: u32 = WebGl2RenderingContext::UNPACK_ALIGNMENT;
pub const PACK_ALIGNMENT: u32 = WebGl2RenderingContext::PACK_ALIGNMENT;
pub const MAX_TEXTURE_SIZE: u32 = WebGl2RenderingContext::MAX_TEXTURE_SIZE;
pub const MAX_VIEWPORT_DIMS: u32 = WebGl2RenderingContext::MAX_VIEWPORT_DIMS;
pub const SUBPIXEL_BITS: u32 = WebGl2RenderingContext::SUBPIXEL_BITS;
pub const RED_BITS: u32 = WebGl2RenderingContext::RED_BITS;
pub const GREEN_BITS: u32 = WebGl2RenderingContext::GREEN_BITS;
pub const BLUE_BITS: u32 = WebGl2RenderingContext::BLUE_BITS;
pub const ALPHA_BITS: u32 = WebGl2RenderingContext::ALPHA_BITS;
pub const DEPTH_BITS: u32 = WebGl2RenderingContext::DEPTH_BITS;
pub const STENCIL_BITS: u32 = WebGl2RenderingContext::STENCIL_BITS;
pub const POLYGON_OFFSET_UNITS: u32 = WebGl2RenderingContext::POLYGON_OFFSET_UNITS;
pub const POLYGON_OFFSET_FACTOR: u32 = WebGl2RenderingContext::POLYGON_OFFSET_FACTOR;
pub const TEXTURE_BINDING_2D: u32 = WebGl2RenderingContext::TEXTURE_BINDING_2D;
pub const SAMPLE_BUFFERS: u32 = WebGl2RenderingContext::SAMPLE_BUFFERS;
pub const SAMPLES: u32 = WebGl2RenderingContext::SAMPLES;
pub const SAMPLE_COVERAGE_VALUE: u32 = WebGl2RenderingContext::SAMPLE_COVERAGE_VALUE;
pub const SAMPLE_COVERAGE_INVERT: u32 = WebGl2RenderingContext::SAMPLE_COVERAGE_INVERT;
pub const COMPRESSED_TEXTURE_FORMATS: u32 = WebGl2RenderingContext::COMPRESSED_TEXTURE_FORMATS;
pub const DONT_CARE: u32 = WebGl2RenderingContext::DONT_CARE;
pub const FASTEST: u32 = WebGl2RenderingContext::FASTEST;
pub const NICEST: u32 = WebGl2RenderingContext::NICEST;
pub const GENERATE_MIPMAP_HINT: u32 = WebGl2RenderingContext::GENERATE_MIPMAP_HINT;
pub const BYTE: u32 = WebGl2RenderingContext::BYTE;
pub const UNSIGNED_BYTE: u32 = WebGl2RenderingContext::UNSIGNED_BYTE;
pub const SHORT: u32 = WebGl2RenderingContext::SHORT;
pub const UNSIGNED_SHORT: u32 = WebGl2RenderingContext::UNSIGNED_SHORT;
pub const INT: u32 = WebGl2RenderingContext::INT;
pub const UNSIGNED_INT: u32 = WebGl2RenderingContext::UNSIGNED_INT;
pub const FLOAT: u32 = WebGl2RenderingContext::FLOAT;
pub const DEPTH_COMPONENT: u32 = WebGl2RenderingContext::DEPTH_COMPONENT;
pub const ALPHA: u32 = WebGl2RenderingContext::ALPHA;
pub const RGB: u32 = WebGl2RenderingContext::RGB;
pub const RGBA: u32 = WebGl2RenderingContext::RGBA;
pub const LUMINANCE: u32 = WebGl2RenderingContext::LUMINANCE;
pub const LUMINANCE_ALPHA: u32 = WebGl2RenderingContext::LUMINANCE_ALPHA;
pub const UNSIGNED_SHORT_4_4_4_4: u32 = WebGl2RenderingContext::UNSIGNED_SHORT_4_4_4_4;
pub const UNSIGNED_SHORT_5_5_5_1: u32 = WebGl2RenderingContext::UNSIGNED_SHORT_5_5_5_1;
pub const UNSIGNED_SHORT_5_6_5: u32 = WebGl2RenderingContext::UNSIGNED_SHORT_5_6_5;
pub const FRAGMENT_SHADER: u32 = WebGl2RenderingContext::FRAGMENT_SHADER;
pub const VERTEX_SHADER: u32 = WebGl2RenderingContext::VERTEX_SHADER;
pub const MAX_VERTEX_ATTRIBS: u32 = WebGl2RenderingContext::MAX_VERTEX_ATTRIBS;
pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = WebGl2RenderingContext::MAX_VERTEX_UNIFORM_VECTORS;
pub const MAX_VARYING_VECTORS: u32 = WebGl2RenderingContext::MAX_VARYING_VECTORS;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 =
    WebGl2RenderingContext::MAX_COMBINED_TEXTURE_IMAGE_UNITS;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 =
    WebGl2RenderingContext::MAX_VERTEX_TEXTURE_IMAGE_UNITS;
pub const MAX_TEXTURE_IMAGE_UNITS: u32 = WebGl2RenderingContext::MAX_TEXTURE_IMAGE_UNITS;
pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = WebGl2RenderingContext::MAX_FRAGMENT_UNIFORM_VECTORS;
pub const SHADER_TYPE: u32 = WebGl2RenderingContext::SHADER_TYPE;
pub const DELETE_STATUS: u32 = WebGl2RenderingContext::DELETE_STATUS;
pub const LINK_STATUS: u32 = WebGl2RenderingContext::LINK_STATUS;
pub const VALIDATE_STATUS: u32 = WebGl2RenderingContext::VALIDATE_STATUS;
pub const ATTACHED_SHADERS: u32 = WebGl2RenderingContext::ATTACHED_SHADERS;
pub const ACTIVE_UNIFORMS: u32 = WebGl2RenderingContext::ACTIVE_UNIFORMS;
pub const ACTIVE_ATTRIBUTES: u32 = WebGl2RenderingContext::ACTIVE_ATTRIBUTES;
pub const SHADING_LANGUAGE_VERSION: u32 = WebGl2RenderingContext::SHADING_LANGUAGE_VERSION;
pub const CURRENT_PROGRAM: u32 = WebGl2RenderingContext::CURRENT_PROGRAM;
pub const NEVER: u32 = WebGl2RenderingContext::NEVER;
pub const LESS: u32 = WebGl2RenderingContext::LESS;
pub const EQUAL: u32 = WebGl2RenderingContext::EQUAL;
pub const LEQUAL: u32 = WebGl2RenderingContext::LEQUAL;
pub const GREATER: u32 = WebGl2RenderingContext::GREATER;
pub const NOTEQUAL: u32 = WebGl2RenderingContext::NOTEQUAL;
pub const GEQUAL: u32 = WebGl2RenderingContext::GEQUAL;
pub const ALWAYS: u32 = WebGl2RenderingContext::ALWAYS;
pub const KEEP: u32 = WebGl2RenderingContext::KEEP;
pub const REPLACE: u32 = WebGl2RenderingContext::REPLACE;
pub const INCR: u32 = WebGl2RenderingContext::INCR;
pub const DECR: u32 = WebGl2RenderingContext::DECR;
pub const INVERT: u32 = WebGl2RenderingContext::INVERT;
pub const INCR_WRAP: u32 = WebGl2RenderingContext::INCR_WRAP;
pub const DECR_WRAP: u32 = WebGl2RenderingContext::DECR_WRAP;
pub const VENDOR: u32 = WebGl2RenderingContext::VENDOR;
pub const RENDERER: u32 = WebGl2RenderingContext::RENDERER;
pub const VERSION: u32 = WebGl2RenderingContext::VERSION;
pub const NEAREST: u32 = WebGl2RenderingContext::NEAREST;
pub const LINEAR: u32 = WebGl2RenderingContext::LINEAR;
pub const NEAREST_MIPMAP_NEAREST: u32 = WebGl2RenderingContext::NEAREST_MIPMAP_NEAREST;
pub const LINEAR_MIPMAP_NEAREST: u32 = WebGl2RenderingContext::LINEAR_MIPMAP_NEAREST;
pub const NEAREST_MIPMAP_LINEAR: u32 = WebGl2RenderingContext::NEAREST_MIPMAP_LINEAR;
pub const LINEAR_MIPMAP_LINEAR: u32 = WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR;
pub const TEXTURE_MAG_FILTER: u32 = WebGl2RenderingContext::TEXTURE_MAG_FILTER;
pub const TEXTURE_MIN_FILTER: u32 = WebGl2RenderingContext::TEXTURE_MIN_FILTER;
pub const TEXTURE_WRAP_S: u32 = WebGl2RenderingContext::TEXTURE_WRAP_S;
pub const TEXTURE_WRAP_T: u32 = WebGl2RenderingContext::TEXTURE_WRAP_T;
pub const TEXTURE_2D: u32 = WebGl2RenderingContext::TEXTURE_2D;
pub const TEXTURE: u32 = WebGl2RenderingContext::TEXTURE;
pub const TEXTURE_CUBE_MAP: u32 = WebGl2RenderingContext::TEXTURE_CUBE_MAP;
pub const TEXTURE_BINDING_CUBE_MAP: u32 = WebGl2RenderingContext::TEXTURE_BINDING_CUBE_MAP;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_X;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_X;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Y;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Y;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Z;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Z;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = WebGl2RenderingContext::MAX_CUBE_MAP_TEXTURE_SIZE;
pub const TEXTURE0: u32 = WebGl2RenderingContext::TEXTURE0;
pub const TEXTURE1: u32 = WebGl2RenderingContext::TEXTURE1;
pub const TEXTURE2: u32 = WebGl2RenderingContext::TEXTURE2;
pub const TEXTURE3: u32 = WebGl2RenderingContext::TEXTURE3;
pub const TEXTURE4: u32 = WebGl2RenderingContext::TEXTURE4;
pub const TEXTURE5: u32 = WebGl2RenderingContext::TEXTURE5;
pub const TEXTURE6: u32 = WebGl2RenderingContext::TEXTURE6;
pub const TEXTURE7: u32 = WebGl2RenderingContext::TEXTURE7;
pub const TEXTURE8: u32 = WebGl2RenderingContext::TEXTURE8;
pub const TEXTURE9: u32 = WebGl2RenderingContext::TEXTURE9;
pub const TEXTURE10: u32 = WebGl2RenderingContext::TEXTURE10;
pub const TEXTURE11: u32 = WebGl2RenderingContext::TEXTURE11;
pub const TEXTURE12: u32 = WebGl2RenderingContext::TEXTURE12;
pub const TEXTURE13: u32 = WebGl2RenderingContext::TEXTURE13;
pub const TEXTURE14: u32 = WebGl2RenderingContext::TEXTURE14;
pub const TEXTURE15: u32 = WebGl2RenderingContext::TEXTURE15;
pub const TEXTURE16: u32 = WebGl2RenderingContext::TEXTURE16;
pub const TEXTURE17: u32 = WebGl2RenderingContext::TEXTURE17;
pub const TEXTURE18: u32 = WebGl2RenderingContext::TEXTURE18;
pub const TEXTURE19: u32 = WebGl2RenderingContext::TEXTURE19;
pub const TEXTURE20: u32 = WebGl2RenderingContext::TEXTURE20;
pub const TEXTURE21: u32 = WebGl2RenderingContext::TEXTURE21;
pub const TEXTURE22: u32 = WebGl2RenderingContext::TEXTURE22;
pub const TEXTURE23: u32 = WebGl2RenderingContext::TEXTURE23;
pub const TEXTURE24: u32 = WebGl2RenderingContext::TEXTURE24;
pub const TEXTURE25: u32 = WebGl2RenderingContext::TEXTURE25;
pub const TEXTURE26: u32 = WebGl2RenderingContext::TEXTURE26;
pub const TEXTURE27: u32 = WebGl2RenderingContext::TEXTURE27;
pub const TEXTURE28: u32 = WebGl2RenderingContext::TEXTURE28;
pub const TEXTURE29: u32 = WebGl2RenderingContext::TEXTURE29;
pub const TEXTURE30: u32 = WebGl2RenderingContext::TEXTURE30;
pub const TEXTURE31: u32 = WebGl2RenderingContext::TEXTURE31;
pub const ACTIVE_TEXTURE: u32 = WebGl2RenderingContext::ACTIVE_TEXTURE;
pub const REPEAT: u32 = WebGl2RenderingContext::REPEAT;
pub const CLAMP_TO_EDGE: u32 = WebGl2RenderingContext::CLAMP_TO_EDGE;
pub const MIRRORED_REPEAT: u32 = WebGl2RenderingContext::MIRRORED_REPEAT;
pub const FLOAT_VEC2: u32 = WebGl2RenderingContext::FLOAT_VEC2;
pub const FLOAT_VEC3: u32 = WebGl2RenderingContext::FLOAT_VEC3;
pub const FLOAT_VEC4: u32 = WebGl2RenderingContext::FLOAT_VEC4;
pub const INT_VEC2: u32 = WebGl2RenderingContext::INT_VEC2;
pub const INT_VEC3: u32 = WebGl2RenderingContext::INT_VEC3;
pub const INT_VEC4: u32 = WebGl2RenderingContext::INT_VEC4;
pub const BOOL: u32 = WebGl2RenderingContext::BOOL;
pub const BOOL_VEC2: u32 = WebGl2RenderingContext::BOOL_VEC2;
pub const BOOL_VEC3: u32 = WebGl2RenderingContext::BOOL_VEC3;
pub const BOOL_VEC4: u32 = WebGl2RenderingContext::BOOL_VEC4;
pub const FLOAT_MAT2: u32 = WebGl2RenderingContext::FLOAT_MAT2;
pub const FLOAT_MAT3: u32 = WebGl2RenderingContext::FLOAT_MAT3;
pub const FLOAT_MAT4: u32 = WebGl2RenderingContext::FLOAT_MAT4;
pub const SAMPLER_2D: u32 = WebGl2RenderingContext::SAMPLER_2D;
pub const SAMPLER_CUBE: u32 = WebGl2RenderingContext::SAMPLER_CUBE;
pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = WebGl2RenderingContext::VERTEX_ATTRIB_ARRAY_ENABLED;
pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = WebGl2RenderingContext::VERTEX_ATTRIB_ARRAY_SIZE;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = WebGl2RenderingContext::VERTEX_ATTRIB_ARRAY_STRIDE;
pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = WebGl2RenderingContext::VERTEX_ATTRIB_ARRAY_TYPE;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 =
    WebGl2RenderingContext::VERTEX_ATTRIB_ARRAY_NORMALIZED;
pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = WebGl2RenderingContext::VERTEX_ATTRIB_ARRAY_POINTER;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 =
    WebGl2RenderingContext::VERTEX_ATTRIB_ARRAY_BUFFER_BINDING;
pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 =
    WebGl2RenderingContext::IMPLEMENTATION_COLOR_READ_TYPE;
pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 =
    WebGl2RenderingContext::IMPLEMENTATION_COLOR_READ_FORMAT;
pub const COMPILE_STATUS: u32 = WebGl2RenderingContext::COMPILE_STATUS;
pub const LOW_FLOAT: u32 = WebGl2RenderingContext::LOW_FLOAT;
pub const MEDIUM_FLOAT: u32 = WebGl2RenderingContext::MEDIUM_FLOAT;
pub const HIGH_FLOAT: u32 = WebGl2RenderingContext::HIGH_FLOAT;
pub const LOW_INT: u32 = WebGl2RenderingContext::LOW_INT;
pub const MEDIUM_INT: u32 = WebGl2RenderingContext::MEDIUM_INT;
pub const HIGH_INT: u32 = WebGl2RenderingContext::HIGH_INT;
pub const FRAMEBUFFER: u32 = WebGl2RenderingContext::FRAMEBUFFER;
pub const RENDERBUFFER: u32 = WebGl2RenderingContext::RENDERBUFFER;
pub const RGBA4: u32 = WebGl2RenderingContext::RGBA4;
pub const RGB5_A1: u32 = WebGl2RenderingContext::RGB5_A1;
pub const RGB565: u32 = WebGl2RenderingContext::RGB565;
pub const DEPTH_COMPONENT16: u32 = WebGl2RenderingContext::DEPTH_COMPONENT16;
pub const STENCIL_INDEX8: u32 = WebGl2RenderingContext::STENCIL_INDEX8;
pub const DEPTH_STENCIL: u32 = WebGl2RenderingContext::DEPTH_STENCIL;
pub const RENDERBUFFER_WIDTH: u32 = WebGl2RenderingContext::RENDERBUFFER_WIDTH;
pub const RENDERBUFFER_HEIGHT: u32 = WebGl2RenderingContext::RENDERBUFFER_HEIGHT;
pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = WebGl2RenderingContext::RENDERBUFFER_INTERNAL_FORMAT;
pub const RENDERBUFFER_RED_SIZE: u32 = WebGl2RenderingContext::RENDERBUFFER_RED_SIZE;
pub const RENDERBUFFER_GREEN_SIZE: u32 = WebGl2RenderingContext::RENDERBUFFER_GREEN_SIZE;
pub const RENDERBUFFER_BLUE_SIZE: u32 = WebGl2RenderingContext::RENDERBUFFER_BLUE_SIZE;
pub const RENDERBUFFER_ALPHA_SIZE: u32 = WebGl2RenderingContext::RENDERBUFFER_ALPHA_SIZE;
pub const RENDERBUFFER_DEPTH_SIZE: u32 = WebGl2RenderingContext::RENDERBUFFER_DEPTH_SIZE;
pub const RENDERBUFFER_STENCIL_SIZE: u32 = WebGl2RenderingContext::RENDERBUFFER_STENCIL_SIZE;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_OBJECT_NAME;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE;
pub const COLOR_ATTACHMENT0: u32 = WebGl2RenderingContext::COLOR_ATTACHMENT0;
pub const DEPTH_ATTACHMENT: u32 = WebGl2RenderingContext::DEPTH_ATTACHMENT;
pub const STENCIL_ATTACHMENT: u32 = WebGl2RenderingContext::STENCIL_ATTACHMENT;
pub const DEPTH_STENCIL_ATTACHMENT: u32 = WebGl2RenderingContext::DEPTH_STENCIL_ATTACHMENT;
pub const NONE: u32 = WebGl2RenderingContext::NONE;
pub const FRAMEBUFFER_COMPLETE: u32 = WebGl2RenderingContext::FRAMEBUFFER_COMPLETE;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_INCOMPLETE_ATTACHMENT;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT;
pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: u32 =
    WebGl2RenderingContext::FRAMEBUFFER_INCOMPLETE_DIMENSIONS;
pub const FRAMEBUFFER_UNSUPPORTED: u32 = WebGl2RenderingContext::FRAMEBUFFER_UNSUPPORTED;
pub const FRAMEBUFFER_BINDING: u32 = WebGl2RenderingContext::FRAMEBUFFER_BINDING;
pub const RENDERBUFFER_BINDING: u32 = WebGl2RenderingContext::RENDERBUFFER_BINDING;
pub const MAX_RENDERBUFFER_SIZE: u32 = WebGl2RenderingContext::MAX_RENDERBUFFER_SIZE;
pub const INVALID_FRAMEBUFFER_OPERATION: u32 =
    WebGl2RenderingContext::INVALID_FRAMEBUFFER_OPERATION;
pub const UNPACK_FLIP_Y_WEBGL: u32 = WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL;
pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 =
    WebGl2RenderingContext::UNPACK_PREMULTIPLY_ALPHA_WEBGL;
pub const CONTEXT_LOST_WEBGL: u32 = WebGl2RenderingContext::CONTEXT_LOST_WEBGL;
pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 =
    WebGl2RenderingContext::UNPACK_COLORSPACE_CONVERSION_WEBGL;
pub const BROWSER_DEFAULT_WEBGL: u32 = WebGl2RenderingContext::BROWSER_DEFAULT_WEBGL;

pub type Buffer = WebGlBuffer;
pub type Fence = WebGlSync;
pub type Framebuffer = WebGlFramebuffer;
pub type Program = WebGlProgram;
pub type Query = WebGlQuery;
pub type Renderbuffer = WebGlRenderbuffer;
pub type Sampler = WebGlSampler;
pub type Shader = WebGlShader;
pub type Texture = WebGlTexture;
pub type TransformFeedback = WebGlTransformFeedback;
pub type UniformLocation = WebGlUniformLocation;
pub type VertexArray = WebGlVertexArrayObject;

pub type BufferName<'a> = &'a WebGlBuffer;
pub type FenceName<'a> = &'a WebGlSync;
pub type FramebufferName<'a> = &'a WebGlFramebuffer;
pub type ProgramName<'a> = &'a WebGlProgram;
pub type QueryName<'a> = &'a WebGlQuery;
pub type RenderbufferName<'a> = &'a WebGlRenderbuffer;
pub type SamplerName<'a> = &'a WebGlSampler;
pub type ShaderName<'a> = &'a WebGlShader;
pub type TextureName<'a> = &'a WebGlTexture;
pub type TransformFeedbackName<'a> = &'a WebGlTransformFeedback;
pub type UniformLocationName<'a> = &'a WebGlUniformLocation;
pub type VertexArrayName<'a> = &'a WebGlVertexArrayObject;

pub struct Context {
    gl: WebGl2RenderingContext,
}

impl Context {
    pub fn from_webgl2_context(gl: WebGl2RenderingContext) -> Self {
        Self { gl }
    }

    pub unsafe fn get_program_link_status(&self, program: ProgramName) -> bool {
        self.gl
            .get_program_parameter(program, LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
    }

    pub unsafe fn get_shader_compile_status(&self, shader: ShaderName) -> bool {
        self.gl
            .get_shader_parameter(shader, COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
    }

    pub unsafe fn bind_buffer(&self, target: u32, buffer: Option<&wrappers::Buffer>) {
        self.gl.bind_buffer(target, buffer.map(|b| b.name()));
    }

    pub unsafe fn bind_framebuffer(
        &self,
        target: u32,
        framebuffer: Option<&wrappers::Framebuffer>,
    ) {
        self.gl
            .bind_framebuffer(target, framebuffer.map(|t| t.name()));
    }

    pub unsafe fn bind_renderbuffer(
        &self,
        target: u32,
        renderbuffer: Option<&wrappers::Renderbuffer>,
    ) {
        self.gl
            .bind_renderbuffer(target, renderbuffer.map(|t| t.name()));
    }

    pub unsafe fn bind_texture(&self, target: u32, texture: Option<&wrappers::Texture>) {
        self.gl.bind_texture(target, texture.map(|t| t.name()));
    }

    pub unsafe fn bind_vertex_array(&self, vertex_array: Option<&wrappers::VertexArray>) {
        self.gl.bind_vertex_array(vertex_array.map(|t| t.name()));
    }
}

impl std::ops::Deref for Context {
    type Target = WebGl2RenderingContext;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl std::ops::DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gl
    }
}
