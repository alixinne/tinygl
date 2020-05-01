use std::fmt;

pub mod codegen_ext;
pub mod prelude {
    pub use super::codegen_ext::*;
}

use codegen_ext::CodegenExt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AtomType {
    Int,
    Float,
    Double,
    UInt,
    Bool,
}

impl AtomType {
    pub fn is_float_type(self) -> bool {
        match self {
            Self::Float | Self::Double => true,
            _ => false,
        }
    }
}

impl fmt::Display for AtomType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.glsl_base_type())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct VectorType {
    pub base_type: AtomType,
    pub components: u32,
}

impl VectorType {
    fn new(base_type: AtomType, components: u32) -> Self {
        Self {
            base_type,
            components,
        }
    }
}

impl fmt::Display for VectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.glsl_vec_name())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MatrixType {
    pub base_type: AtomType,
    pub n: u32,
}

impl MatrixType {
    fn new(base_type: AtomType, n: u32) -> Self {
        Self { base_type, n }
    }
}

impl fmt::Display for MatrixType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.glsl_mat_name())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GenericType {
    Atom(AtomType),
    Vector(VectorType),
    Matrix(MatrixType),
}

impl fmt::Display for GenericType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Atom(atom_type) => fmt::Display::fmt(atom_type, f),
            Self::Vector(vector_type) => fmt::Display::fmt(vector_type, f),
            Self::Matrix(matrix_type) => fmt::Display::fmt(matrix_type, f),
        }
    }
}

impl GenericType {
    fn vector(inner_type: Self, components: u32) -> Self {
        match inner_type {
            Self::Atom(atom_type) if components > 1 => {
                Self::Vector(VectorType::new(atom_type, components))
            }
            _ => panic!("unsupported type combination"),
        }
    }

    fn matrix(inner_type: Self, n: u32) -> Self {
        match inner_type {
            Self::Atom(atom_type) if n > 1 && atom_type.is_float_type() => {
                Self::Matrix(MatrixType::new(atom_type, n))
            }
            _ => panic!("unsupported type combination"),
        }
    }

    fn components(&self) -> u32 {
        match self {
            Self::Atom(_) => 1,
            Self::Vector(vector) => vector.components,
            Self::Matrix(matrix) => matrix.n * matrix.n,
        }
    }

    fn glow_value(&self, name: &str) -> String {
        match self {
            Self::Atom(_) => format!("&[{}]", name),
            Self::Vector(_) => format!("{}.as_ref()", name),
            Self::Matrix(inner) => format!(
                "::std::mem::transmute::<_, &[{ty}; {n}]>({name}.as_ref())",
                name = name,
                ty = inner.rust_primitive_type(),
                n = inner.n * inner.n
            ),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemOrArrayType {
    Item(GenericType),
    Array(GenericType, u32),
}

impl fmt::Display for ItemOrArrayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Item(item_type) => fmt::Display::fmt(item_type, f),
            Self::Array(item_type, components) => write!(f, "{}[{}]", item_type, components),
        }
    }
}

impl ItemOrArrayType {
    pub fn atom(atom_type: AtomType) -> Self {
        Self::Item(GenericType::Atom(atom_type))
    }

    pub fn vector(vector_type: Self, components: u32) -> Self {
        match vector_type {
            Self::Item(inner_type) => Self::Item(GenericType::vector(inner_type, components)),
            _ => panic!(
                "unsupported type combination: {:?}[{}]",
                vector_type, components
            ),
        }
    }

    pub fn matrix(matrix_type: Self, components: u32) -> Self {
        match matrix_type {
            Self::Item(inner_type) => Self::Item(GenericType::matrix(inner_type, components)),
            _ => panic!(
                "unsupported type combination: {:?}[{}]",
                matrix_type, components
            ),
        }
    }

    pub fn array(inner_type: Self, components: u32) -> Self {
        match inner_type {
            Self::Item(inner) => Self::Array(inner, components),
            _ => panic!(
                "unsupported type combination: {:?}[{}]",
                inner_type, components
            ),
        }
    }

    fn components(&self) -> u32 {
        match self {
            Self::Item(_) => 1,
            Self::Array(inner_type, _) => inner_type.components(),
        }
    }

    pub fn glow_value(&self, name: &str) -> String {
        match self {
            Self::Item(inner) => inner.glow_value(name),
            Self::Array(inner_type, size) => format!(
                "::std::slice::from_raw_parts({name}.as_ptr() as *const {base_ty}, {size})",
                name = name,
                size = *size * self.components(),
                base_ty = inner_type.rust_primitive_type()
            ),
        }
    }
}

pub struct NamedGenericType<'a> {
    name: &'a str,
    gt: &'a ItemOrArrayType,
}

impl<'a> fmt::Display for NamedGenericType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.gt {
            ItemOrArrayType::Item(atom_type) => write!(f, "{} {}", atom_type, self.name),
            ItemOrArrayType::Array(inner_type, components) => {
                write!(f, "{} {}[{}]", inner_type, self.name, components)
            }
        }
    }
}
