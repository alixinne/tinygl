use heck::SnakeCase;

use rspirv::dr as rr;

use crate::types::*;

#[derive(Debug, Default, Eq, Clone)]
pub struct FoundUniform {
    pub name: String,
    pub location: u32,
    pub ty: Option<ItemOrArrayType>,

    pub binding: Option<i32>,

    location_name: String,
}

impl PartialEq for FoundUniform {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.ty == other.ty
    }
}

use std::hash::{Hash, Hasher};
impl Hash for FoundUniform {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.ty.hash(state);
    }
}

impl FoundUniform {
    pub fn location_name(&self) -> &str {
        &self.location_name
    }
}

pub fn find_uniforms(shader_path: &str, module: &rr::Module) -> crate::Result<Vec<FoundUniform>> {
    // Find constants
    let mut constants = std::collections::HashMap::new();

    // Find types
    let mut types: std::collections::HashMap<spirv_headers::Word, ItemOrArrayType> =
        std::collections::HashMap::new();

    for type_global_value in &module.types_global_values {
        let id = type_global_value.result_id.unwrap_or(0);

        match type_global_value.class.opcode {
            spirv_headers::Op::Constant => {
                if let rr::Operand::LiteralInt32(value) = type_global_value.operands[0] {
                    constants.insert(id, value);
                }
            }
            spirv_headers::Op::TypeInt => {
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
            spirv_headers::Op::TypeFloat => {
                if let rr::Operand::LiteralInt32(32) = type_global_value.operands[0] {
                    types.insert(id, ItemOrArrayType::atom(AtomType::Float));
                } else if let rr::Operand::LiteralInt32(64) = type_global_value.operands[0] {
                    types.insert(id, ItemOrArrayType::atom(AtomType::Double));
                } else {
                    panic!("unsupported float width");
                }
            }
            spirv_headers::Op::TypeBool => {
                // TODO: Check TypeBool syntax
                types.insert(id, ItemOrArrayType::atom(AtomType::Bool));
            }
            spirv_headers::Op::TypeVector => {
                if let rr::Operand::IdRef(type_id) = type_global_value.operands[0] {
                    if let rr::Operand::LiteralInt32(components) = type_global_value.operands[1] {
                        types.insert(id, ItemOrArrayType::vector(types[&type_id], components));
                    }
                }
            }
            spirv_headers::Op::TypeMatrix => {
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
            spirv_headers::Op::TypeArray => {
                if let rr::Operand::IdRef(type_id) = type_global_value.operands[0] {
                    if let rr::Operand::IdRef(constant_id) = type_global_value.operands[1] {
                        if types.get(&type_id).is_some() {
                            types.insert(
                                id,
                                ItemOrArrayType::array(types[&type_id], constants[&constant_id]),
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
            spirv_headers::Op::TypeSampledImage | spirv_headers::Op::TypeImage => {
                // TODO: Store texture details in reflection data?
                // TODO: Store binding details
                types.insert(id, ItemOrArrayType::atom(AtomType::UInt));
            }
            _ => (),
        }
    }

    // Find names and locations
    let mut names: std::collections::HashMap<spirv_headers::Word, FoundUniform> =
        std::collections::HashMap::new();

    // Enumerate known names from debug info
    for debug in &module.debugs {
        if let spirv_headers::Op::Name = debug.class.opcode {
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

    // Enumerate locations
    for annotation in &module.annotations {
        if let spirv_headers::Op::Decorate = annotation.class.opcode {
            if let rr::Operand::Decoration(spirv_headers::Decoration::Location) =
                annotation.operands[1]
            {
                if let rr::Operand::IdRef(id) = annotation.operands[0] {
                    if let rr::Operand::LiteralInt32(location) = annotation.operands[2] {
                        names.get_mut(&id).unwrap().location = location;
                    }
                }
            } else if let rr::Operand::Decoration(spirv_headers::Decoration::Binding) =
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
            spirv_headers::Op::TypePointer => {
                if let rr::Operand::IdRef(type_id) = type_global_value.operands[1] {
                    type_pointers.insert(type_global_value.result_id.unwrap(), type_id);
                } else {
                    panic!("failed to get type_id");
                }
            }
            spirv_headers::Op::Variable => {
                if let rr::Operand::StorageClass(spirv_headers::StorageClass::UniformConstant) =
                    type_global_value.operands[0]
                {
                    let result_id = type_global_value.result_id.unwrap();
                    if let Some(v) = names.get_mut(&result_id) {
                        let tp = type_global_value.result_type.unwrap();

                        match types.get(&type_pointers[&tp]) {
                            Some(ty) => {
                                v.ty = Some(*ty);
                                v.location_name = (v.name.clone() + "_location").to_snake_case();
                            }
                            None => {
                                println!(
                                    "cargo:warning={}: {}: unsupported type, it will not be wrapped",
                                    shader_path,
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
