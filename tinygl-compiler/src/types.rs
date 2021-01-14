use std::fmt;

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

    pub fn glsl_base_type(&self) -> &'static str {
        match self {
            Self::Int => "int",
            Self::Float => "float",
            Self::Double => "double",
            Self::UInt => "uint",
            Self::Bool => "bool",
        }
    }

    pub fn glsl_vec_name(&self) -> String {
        match self {
            Self::Int => "ivec",
            Self::Float => "vec",
            Self::Double => "dvec",
            Self::UInt => "uvec",
            Self::Bool => "bvec",
        }
        .into()
    }

    pub fn glsl_mat_name(&self) -> String {
        match self {
            Self::Float => "mat",
            Self::Double => "dmat",
            _ => panic!("cannot use mat_name on non-float"),
        }
        .into()
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

    pub fn glsl_vec_name(&self) -> String {
        format!("{}{}", self.base_type.glsl_vec_name(), self.components)
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

    pub fn glsl_mat_name(&self) -> String {
        format!("{}{}", self.base_type.glsl_mat_name(), self.n)
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemOrArrayType {
    Item(GenericType),
    Array(GenericType, u32),
    Image { format: Option<u32> },
}

impl fmt::Display for ItemOrArrayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Item(item_type) => fmt::Display::fmt(item_type, f),
            Self::Array(item_type, components) => write!(f, "{}[{}]", item_type, components),
            Self::Image { format: Some(fmt) } => write!(f, "image({})", fmt),
            Self::Image { format: None } => write!(f, "image(unknown format)"),
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
}
