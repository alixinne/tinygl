use heck::SnakeCase;
use rspirv::dr as rr;

use super::*;
use crate::types::*;

#[derive(Default)]
pub struct SpirVBackend {}

impl SpirVBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find_uniforms(&self, module: &rspirv::dr::Module) -> crate::Result<Vec<FoundUniform>> {
        // Find names and locations
        let mut names: std::collections::HashMap<rspirv::spirv::Word, FoundUniform> =
            std::collections::HashMap::new();

        // Enumerate known names from debug info
        for debug in &module.debug_names {
            if let rspirv::spirv::Op::Name = debug.class.opcode {
                if let rr::Operand::IdRef(id) = debug.operands[0] {
                    if let rr::Operand::LiteralString(name) = &debug.operands[1] {
                        names.insert(
                            id,
                            FoundUniform {
                                name: name.to_owned(),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }

        // Find constants
        let mut constants = std::collections::HashMap::new();

        // Find types
        let mut types: std::collections::HashMap<rspirv::spirv::Word, ItemOrArrayType> =
            std::collections::HashMap::new();

        for type_global_value in &module.types_global_values {
            let id = type_global_value.result_id.unwrap_or(0);

            match type_global_value.class.opcode {
                rspirv::spirv::Op::Constant => {
                    if let rr::Operand::LiteralInt32(value) = type_global_value.operands[0] {
                        constants.insert(id, value);
                    }
                }
                rspirv::spirv::Op::TypeInt => {
                    if let rr::Operand::LiteralInt32(32) = type_global_value.operands[0] {
                        if let rr::Operand::LiteralInt32(0) = type_global_value.operands[1] {
                            types.insert(id, ItemOrArrayType::atom(AtomType::UInt));
                        } else {
                            types.insert(id, ItemOrArrayType::atom(AtomType::Int));
                        }
                    } else {
                        panic!("unsupported integer width");
                    }
                }
                rspirv::spirv::Op::TypeFloat => {
                    if let rr::Operand::LiteralInt32(32) = type_global_value.operands[0] {
                        types.insert(id, ItemOrArrayType::atom(AtomType::Float));
                    } else if let rr::Operand::LiteralInt32(64) = type_global_value.operands[0] {
                        types.insert(id, ItemOrArrayType::atom(AtomType::Double));
                    } else {
                        panic!("unsupported float width");
                    }
                }
                rspirv::spirv::Op::TypeBool => {
                    // TODO: Check TypeBool syntax
                    types.insert(id, ItemOrArrayType::atom(AtomType::Bool));
                }
                rspirv::spirv::Op::TypeVector => {
                    if let rr::Operand::IdRef(type_id) = type_global_value.operands[0] {
                        if let rr::Operand::LiteralInt32(components) = type_global_value.operands[1]
                        {
                            types.insert(id, ItemOrArrayType::vector(types[&type_id], components));
                        }
                    }
                }
                rspirv::spirv::Op::TypeMatrix => {
                    if let rr::Operand::IdRef(type_id) = type_global_value.operands[0] {
                        if let rr::Operand::LiteralInt32(n) = type_global_value.operands[1] {
                            if let ItemOrArrayType::Item(GenericType::Vector(VectorType {
                                base_type,
                                components,
                            })) = types[&type_id]
                            {
                                if base_type.is_float_type() {
                                    if components == n {
                                        types.insert(
                                            id,
                                            ItemOrArrayType::matrix(
                                                ItemOrArrayType::atom(base_type),
                                                components,
                                            ),
                                        );
                                    } else {
                                        // TODO: Support rectangular matrices
                                    }
                                }
                            }
                        }
                    }
                }
                rspirv::spirv::Op::TypeArray => {
                    if let rr::Operand::IdRef(type_id) = type_global_value.operands[0] {
                        if let rr::Operand::IdRef(constant_id) = type_global_value.operands[1] {
                            if types.get(&type_id).is_some() {
                                types.insert(
                                    id,
                                    ItemOrArrayType::array(
                                        types[&type_id],
                                        constants[&constant_id],
                                    ),
                                );
                            } else {
                                println!(
                                    "cargo:warning=failed to discover array element type for {}",
                                    type_id
                                );
                            }
                        } else {
                            panic!("failed to get components");
                        }
                    } else {
                        panic!("failed to get type_id");
                    }
                }
                rspirv::spirv::Op::TypeSampledImage | rspirv::spirv::Op::TypeImage => {
                    // Store texture format for image bindings
                    let format = if let Some(rr::Operand::ImageFormat(format)) =
                        type_global_value.operands.last()
                    {
                        match format {
                            rspirv::spirv::ImageFormat::Rgba32f => Some(crate::gl::RGBA32F),
                            rspirv::spirv::ImageFormat::Rgba16f => Some(crate::gl::RGBA16F),
                            rspirv::spirv::ImageFormat::Rg32f => Some(crate::gl::RG32F),
                            rspirv::spirv::ImageFormat::Rg16f => Some(crate::gl::RG16F),
                            rspirv::spirv::ImageFormat::R11fG11fB10f => {
                                Some(crate::gl::R11F_G11F_B10F)
                            }
                            rspirv::spirv::ImageFormat::R32f => Some(crate::gl::R32F),
                            rspirv::spirv::ImageFormat::R16f => Some(crate::gl::R16F),
                            rspirv::spirv::ImageFormat::Rgba32ui => Some(crate::gl::RGBA32UI),
                            rspirv::spirv::ImageFormat::Rgba16ui => Some(crate::gl::RGBA16UI),
                            rspirv::spirv::ImageFormat::Rgb10a2ui => Some(crate::gl::RGB10_A2UI),
                            rspirv::spirv::ImageFormat::Rgba8ui => Some(crate::gl::RGBA8UI),
                            rspirv::spirv::ImageFormat::Rg32ui => Some(crate::gl::RG32UI),
                            rspirv::spirv::ImageFormat::Rg16ui => Some(crate::gl::RG16UI),
                            rspirv::spirv::ImageFormat::Rg8ui => Some(crate::gl::RG8UI),
                            rspirv::spirv::ImageFormat::R32ui => Some(crate::gl::R32UI),
                            rspirv::spirv::ImageFormat::R16ui => Some(crate::gl::R16UI),
                            rspirv::spirv::ImageFormat::R8ui => Some(crate::gl::R8UI),
                            rspirv::spirv::ImageFormat::Rgba32i => Some(crate::gl::RGBA32I),
                            rspirv::spirv::ImageFormat::Rgba16i => Some(crate::gl::RGBA16I),
                            rspirv::spirv::ImageFormat::Rgba8i => Some(crate::gl::RGBA8I),
                            rspirv::spirv::ImageFormat::Rg32i => Some(crate::gl::RG32I),
                            rspirv::spirv::ImageFormat::Rg16i => Some(crate::gl::RG16I),
                            rspirv::spirv::ImageFormat::Rg8i => Some(crate::gl::RG8I),
                            rspirv::spirv::ImageFormat::R32i => Some(crate::gl::R32I),
                            rspirv::spirv::ImageFormat::R16i => Some(crate::gl::R16I),
                            rspirv::spirv::ImageFormat::R8i => Some(crate::gl::R8I),
                            rspirv::spirv::ImageFormat::Rgba16 => Some(crate::gl::RGBA16),
                            rspirv::spirv::ImageFormat::Rgb10A2 => Some(crate::gl::RGB10_A2),
                            rspirv::spirv::ImageFormat::Rgba8 => Some(crate::gl::RGBA8),
                            rspirv::spirv::ImageFormat::Rg16 => Some(crate::gl::RG16),
                            rspirv::spirv::ImageFormat::Rg8 => Some(crate::gl::RG8),
                            rspirv::spirv::ImageFormat::R16 => Some(crate::gl::R16),
                            rspirv::spirv::ImageFormat::R8 => Some(crate::gl::R8),
                            rspirv::spirv::ImageFormat::Rgba16Snorm => {
                                Some(crate::gl::RGBA16_SNORM)
                            }
                            rspirv::spirv::ImageFormat::Rgba8Snorm => Some(crate::gl::RGBA8_SNORM),
                            rspirv::spirv::ImageFormat::Rg16Snorm => Some(crate::gl::RG16_SNORM),
                            rspirv::spirv::ImageFormat::Rg8Snorm => Some(crate::gl::RG8_SNORM),
                            rspirv::spirv::ImageFormat::R16Snorm => Some(crate::gl::R16_SNORM),
                            rspirv::spirv::ImageFormat::R8Snorm => Some(crate::gl::R8_SNORM),
                            _ => None,
                        }
                    } else {
                        None
                    };

                    types.insert(id, ItemOrArrayType::Image { format });
                }
                _ => (),
            }
        }

        // Enumerate locations
        for annotation in &module.annotations {
            if let rspirv::spirv::Op::Decorate = annotation.class.opcode {
                if let rr::Operand::Decoration(rspirv::spirv::Decoration::Location) =
                    annotation.operands[1]
                {
                    if let rr::Operand::IdRef(id) = annotation.operands[0] {
                        if let rr::Operand::LiteralInt32(location) = annotation.operands[2] {
                            names.get_mut(&id).unwrap().location = location;
                        }
                    }
                } else if let rr::Operand::Decoration(rspirv::spirv::Decoration::Binding) =
                    annotation.operands[1]
                {
                    if let rr::Operand::IdRef(id) = annotation.operands[0] {
                        if let rr::Operand::LiteralInt32(binding) = annotation.operands[2] {
                            names.get_mut(&id).unwrap().binding = Some(binding as i32);
                        }
                    }
                }
            }
        }

        // Find global uniform variables and assign types
        let mut type_pointers = std::collections::HashMap::new();

        for type_global_value in &module.types_global_values {
            match type_global_value.class.opcode {
                rspirv::spirv::Op::TypePointer => {
                    if let rr::Operand::IdRef(type_id) = type_global_value.operands[1] {
                        type_pointers.insert(type_global_value.result_id.unwrap(), type_id);
                    } else {
                        panic!("failed to get type_id");
                    }
                }
                rspirv::spirv::Op::Variable => {
                    if let rr::Operand::StorageClass(rspirv::spirv::StorageClass::UniformConstant) =
                        type_global_value.operands[0]
                    {
                        let result_id = type_global_value.result_id.unwrap();
                        if let Some(v) = names.get_mut(&result_id) {
                            let tp = type_global_value.result_type.unwrap();

                            match types.get(&type_pointers[&tp]) {
                                Some(ty) => {
                                    v.ty = Some(*ty);
                                    v.location_name =
                                        (v.name.clone() + "_location").to_snake_case();
                                }
                                None => {
                                    println!(
                                    "cargo:warning={}: unsupported type, it will not be wrapped",
                                    v.name
                                );
                                }
                            }
                        } else {
                            panic!("failed to get result_id");
                        }
                    }
                }
                _ => {}
            }
        }

        let mut v = names
            .drain()
            .map(|(_k, v)| v)
            .filter(|v| v.ty.is_some())
            .collect::<Vec<_>>();

        v.sort_by_key(|item| item.location);
        Ok(v)
    }
}

impl SpirVReflectBackend for SpirVBackend {
    fn reflect<'s>(&self, input: &rspirv::dr::Module) -> crate::Result<Vec<FoundUniform>> {
        Ok(self.find_uniforms(input)?)
    }
}
