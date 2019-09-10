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

#[derive(Debug, Serialize, Deserialize)]
pub struct StepProgram {
    #[serde(skip)]
    pub program: <glow::Context as HasContext>::Program,
    pub uniform_samplers: Vec<Sampler>,
}

#[derive(Default)]
struct UniformVisitor {
    pub uniform_samplers: Vec<String>,
}

impl Visitor for UniformVisitor {
    fn visit_external_declaration(
        &mut self,
        declaration: &mut glsl::syntax::ExternalDeclaration,
    ) -> Visit {
        match declaration {
            glsl::syntax::ExternalDeclaration::Declaration(
                glsl::syntax::Declaration::InitDeclaratorList(list),
            ) => {
                if list.head.ty.ty.ty == glsl::syntax::TypeSpecifierNonArray::Sampler2D {
                    self.uniform_samplers
                        .push(list.head.name.as_ref().unwrap().as_str().into());
                    for tail_item in list.tail.iter() {
                        self.uniform_samplers
                            .push(tail_item.ident.ident.as_str().into());
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
        let mut stage = glsl::syntax::ShaderStage::parse(shader_source)?;
        let mut uniforms = UniformVisitor::default();
        stage.visit(&mut uniforms);

        Ok(Self {
            program,
            uniform_samplers: uniforms
                .uniform_samplers
                .into_iter()
                .map(|name| {
                    let location = unsafe { gl.get_uniform_location(program, &name) };
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

                    Sampler {
                        location,
                        source,
                        name,
                    }
                })
                .collect(),
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
