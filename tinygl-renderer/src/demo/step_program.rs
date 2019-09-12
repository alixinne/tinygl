//! Definition of the StepProgram type

use glow::HasContext;
use glsl::{
    parser::Parse,
    visitor::{Host, Visit, Visitor},
};
use serde_derive::{Deserialize, Serialize};

use super::{CompileError, GlDroppable};

#[derive(Debug, Serialize, Deserialize)]
pub enum SamplerSource {
    None,
    BufferId(usize),
    BufferName(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sampler {
    #[cfg_attr(target_arch = "wasm32", serde(skip))] // UniformLocation not serializable on wasm
    pub location: Option<<glow::Context as HasContext>::UniformLocation>,
    pub source: SamplerSource,
    pub name: String,
}

impl Sampler {
    pub fn new(
        name: String,
        location: Option<<glow::Context as HasContext>::UniformLocation>,
    ) -> Self {
        let name_base = "inPass";
        let source = if name.starts_with(name_base) {
            let name = &name[name_base.len()..];
            if let Ok(id) = name.parse::<u32>() {
                SamplerSource::BufferId(id as usize)
            } else {
                SamplerSource::BufferName(name.to_owned())
            }
        } else {
            SamplerSource::None
        };

        Self {
            name,
            source,
            location,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UniformSource {
    None,
    Resolution,
    Time,
    FrameNumber,
}

impl UniformSource {
    pub fn from_name_ty(name: &str, ty: &UniformValue) -> Self {
        match ty {
            UniformValue::Vec3(_) if name == "iResolution" => UniformSource::Resolution,
            UniformValue::Float(_) if name == "iTime" => UniformSource::Time,
            UniformValue::Int(_) if name == "iFrame" => UniformSource::FrameNumber,
            _ => UniformSource::None,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Int(i32),
    IVec2([i32; 2]),
    IVec3([i32; 3]),
    IVec4([i32; 4]),
    Bool(bool),
    BVec2([bool; 2]),
    BVec3([bool; 3]),
    BVec4([bool; 4]),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Uniform {
    #[cfg_attr(target_arch = "wasm32", serde(skip))]
    pub location: Option<<glow::Context as HasContext>::UniformLocation>,
    pub value: UniformValue,
    pub source: UniformSource,
    pub name: String,
}

macro_rules! bint {
    ($x:expr) => {
        if $x {
            1
        } else {
            0
        }
    };
}

impl Uniform {
    pub fn set(&self, gl: &glow::Context) {
        unsafe {
            match self.value {
                UniformValue::Float(x) => gl.uniform_1_f32(self.location, x),
                UniformValue::Vec2([x, y]) => gl.uniform_2_f32(self.location, x, y),
                UniformValue::Vec3([x, y, z]) => gl.uniform_3_f32(self.location, x, y, z),
                UniformValue::Vec4([x, y, z, w]) => gl.uniform_4_f32(self.location, x, y, z, w),
                UniformValue::Int(x) => gl.uniform_1_i32(self.location, x),
                UniformValue::IVec2([x, y]) => gl.uniform_2_i32(self.location, x, y),
                UniformValue::IVec3([x, y, z]) => gl.uniform_3_i32(self.location, x, y, z),
                UniformValue::IVec4([x, y, z, w]) => gl.uniform_4_i32(self.location, x, y, z, w),
                UniformValue::Bool(x) => gl.uniform_1_i32(self.location, bint!(x)),
                UniformValue::BVec2([x, y]) => gl.uniform_2_i32(self.location, bint!(x), bint!(y)),
                UniformValue::BVec3([x, y, z]) => {
                    gl.uniform_3_i32(self.location, bint!(x), bint!(y), bint!(z))
                }
                UniformValue::BVec4([x, y, z, w]) => {
                    gl.uniform_4_i32(self.location, bint!(x), bint!(y), bint!(z), bint!(w))
                }
            }
        }
    }
}

struct UniformBuilder {
    default_value: UniformValue,
}

impl UniformBuilder {
    fn try_new(ty: glsl::syntax::TypeSpecifierNonArray) -> Option<Self> {
        let default_value = match ty {
            glsl::syntax::TypeSpecifierNonArray::Float => Some(UniformValue::Float(0.)),
            glsl::syntax::TypeSpecifierNonArray::Vec2 => Some(UniformValue::Vec2([0., 0.])),
            glsl::syntax::TypeSpecifierNonArray::Vec3 => Some(UniformValue::Vec3([0., 0., 0.])),
            glsl::syntax::TypeSpecifierNonArray::Vec4 => Some(UniformValue::Vec4([0., 0., 0., 0.])),
            glsl::syntax::TypeSpecifierNonArray::Int => Some(UniformValue::Int(0)),
            glsl::syntax::TypeSpecifierNonArray::IVec2 => Some(UniformValue::IVec2([0, 0])),
            glsl::syntax::TypeSpecifierNonArray::IVec3 => Some(UniformValue::IVec3([0, 0, 0])),
            glsl::syntax::TypeSpecifierNonArray::IVec4 => Some(UniformValue::IVec4([0, 0, 0, 0])),
            glsl::syntax::TypeSpecifierNonArray::Bool => Some(UniformValue::Bool(false)),
            glsl::syntax::TypeSpecifierNonArray::BVec2 => Some(UniformValue::BVec2([false, false])),
            glsl::syntax::TypeSpecifierNonArray::BVec3 => {
                Some(UniformValue::BVec3([false, false, false]))
            }
            glsl::syntax::TypeSpecifierNonArray::BVec4 => {
                Some(UniformValue::BVec4([false, false, false, false]))
            }
            _ => None,
        };

        default_value.map(|default_value| Self { default_value })
    }

    fn build(
        &self,
        name: String,
        location: Option<<glow::Context as HasContext>::UniformLocation>,
    ) -> Uniform {
        let source = UniformSource::from_name_ty(&name, &self.default_value);

        Uniform {
            name,
            value: self.default_value,
            source,
            location,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepProgram {
    #[serde(skip)]
    pub program: <glow::Context as HasContext>::Program,
    pub uniform_samplers: Vec<Sampler>,
    pub uniforms: Vec<Uniform>,
}

struct UniformVisitor<'a> {
    uniform_samplers: &'a mut Vec<Sampler>,
    uniforms: &'a mut Vec<Uniform>,
    gl: &'a glow::Context,
    program: <glow::Context as HasContext>::Program,
}

impl<'a> UniformVisitor<'a> {
    fn make_sampler(&self, name: String) -> Sampler {
        let location = unsafe { self.gl.get_uniform_location(self.program, &name) };
        Sampler::new(name, location)
    }

    fn make_value(&self, name: String, builder: &UniformBuilder) -> Uniform {
        let location = unsafe { self.gl.get_uniform_location(self.program, &name) };
        builder.build(name, location)
    }
}

impl<'a> Visitor for UniformVisitor<'a> {
    fn visit_external_declaration(
        &mut self,
        declaration: &mut glsl::syntax::ExternalDeclaration,
    ) -> Visit {
        match declaration {
            glsl::syntax::ExternalDeclaration::Declaration(
                glsl::syntax::Declaration::InitDeclaratorList(list),
            ) => {
                let ty = &list.head.ty.ty.ty;
                if *ty == glsl::syntax::TypeSpecifierNonArray::Sampler2D {
                    self.uniform_samplers
                        .push(self.make_sampler(list.head.name.as_ref().unwrap().as_str().into()));
                    for tail_item in list.tail.iter() {
                        self.uniform_samplers
                            .push(self.make_sampler(tail_item.ident.ident.as_str().into()));
                    }
                } else if let Some(builder) = UniformBuilder::try_new(ty.clone()) {
                    self.uniforms.push(
                        self.make_value(list.head.name.as_ref().unwrap().as_str().into(), &builder),
                    );
                    for tail_item in list.tail.iter() {
                        self.uniforms
                            .push(self.make_value(tail_item.ident.ident.as_str().into(), &builder));
                    }
                }
            }
            _ => {}
        }

        Visit::Parent
    }
}

impl StepProgram {
    pub fn new(
        gl: &glow::Context,
        shader_source: &str,
        program: <glow::Context as HasContext>::Program,
    ) -> Result<Self, CompileError> {
        let mut uniform_samplers = Vec::new();
        let mut uniforms = Vec::new();

        // Parse shader
        let mut stage = glsl::syntax::ShaderStage::parse(shader_source)?;
        // Extract uniforms
        let mut visitor = UniformVisitor {
            uniforms: &mut uniforms,
            uniform_samplers: &mut uniform_samplers,
            gl,
            program,
        };

        stage.visit(&mut visitor);

        Ok(Self {
            program,
            uniform_samplers,
            uniforms,
        })
    }
}

impl GlDroppable for StepProgram {
    fn drop(&mut self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
        }
    }
}
