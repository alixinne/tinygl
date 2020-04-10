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
    fn vec_name(self) -> &'static str {
        match self {
            Self::Int => "ivec",
            Self::Float => "vec",
            Self::Double => "dvec",
            Self::UInt => "uvec",
            Self::Bool => "bvec",
        }
    }

    fn mat_name(self) -> &'static str {
        match self {
            Self::Float => "mat",
            Self::Double => "dmat",
            _ => panic!("cannot use mat_name on non-float"),
        }
    }

    fn cgmath_name(self) -> &'static str {
        match self {
            Self::Int => "i32",
            Self::Float => "f32",
            Self::Double => "f64",
            Self::UInt => "u32",
            Self::Bool => "bool",
        }
    }

    pub fn is_float_type(self) -> bool {
        match self {
            Self::Float | Self::Double => true,
            _ => false,
        }
    }
}

impl fmt::Display for AtomType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Double => write!(f, "double"),
            Self::UInt => write!(f, "uint"),
            Self::Bool => write!(f, "bool"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct VectorType {
    pub base_type: AtomType,
    pub components: u32,
}

impl VectorType {
    pub fn new(base_type: AtomType, components: u32) -> Self {
        Self {
            base_type,
            components,
        }
    }

    pub fn cgmath_name(self) -> String {
        // TODO: Use a formatter
        format!(
            "::tinygl::cgmath::Vector{}<{}>",
            self.components,
            self.base_type.cgmath_name()
        )
    }

    pub fn rstype(self) -> &'static str {
        self.base_type.cgmath_name()
    }

    pub fn components(self) -> u32 {
        self.components
    }
}

impl fmt::Display for VectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.base_type.vec_name(), self.components)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MatrixType {
    pub base_type: AtomType,
    pub n: u32,
}

impl MatrixType {
    pub fn new(base_type: AtomType, n: u32) -> Self {
        Self { base_type, n }
    }

    pub fn cgmath_name(self) -> String {
        // TODO: Use a formatter
        format!(
            "::tinygl::cgmath::Matrix{}<{}>",
            self.n,
            self.base_type.cgmath_name()
        )
    }

    pub fn rstype(self) -> &'static str {
        self.base_type.cgmath_name()
    }

    pub fn components(self) -> u32 {
        self.n * self.n
    }
}

impl fmt::Display for MatrixType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.base_type.mat_name(), self.n)
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
    pub fn vector(inner_type: Self, components: u32) -> Self {
        match inner_type {
            Self::Atom(atom_type) if components > 1 => {
                Self::Vector(VectorType::new(atom_type, components))
            }
            _ => panic!("unsupported type combination"),
        }
    }

    pub fn matrix(inner_type: Self, n: u32) -> Self {
        match inner_type {
            Self::Atom(atom_type) if n > 1 && atom_type.is_float_type() => {
                Self::Matrix(MatrixType::new(atom_type, n))
            }
            _ => panic!("unsupported type combination"),
        }
    }

    pub fn cgmath_name(&self) -> String {
        // TODO: Use a formatter
        match self {
            Self::Atom(atom_type) => atom_type.cgmath_name().to_owned(),
            Self::Vector(vector_type) => vector_type.cgmath_name(),
            Self::Matrix(matrix_type) => matrix_type.cgmath_name(),
        }
    }

    pub fn rstype(&self) -> &'static str {
        match self {
            Self::Atom(atom_type) => atom_type.cgmath_name(),
            Self::Vector(vector_type) => vector_type.rstype(),
            Self::Matrix(matrix_type) => matrix_type.rstype(),
        }
    }

    pub fn components(&self) -> u32 {
        match self {
            Self::Atom(_) => 1,
            Self::Vector(vector_type) => vector_type.components(),
            Self::Matrix(matrix_type) => matrix_type.components(),
        }
    }

    pub fn glow_value(&self, name: &str) -> String {
        match self {
            Self::Atom(_) => format!("&[{}]", name),
            Self::Vector(inner_type) => format!(
                "::std::convert::AsRef::<[{base_ty}; {components}]>::as_ref(&{})",
                name,
                components = self.components(),
                base_ty = inner_type.rstype()
            ),
            Self::Matrix(inner_type) => format!(
                "::std::convert::AsRef::<[{base_ty}; {components}]>::as_ref(&{})",
                name,
                components = self.components(),
                base_ty = inner_type.rstype()
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

    #[allow(dead_code)]
    pub fn named<'a>(&'a self, name: &'a str) -> NamedGenericType<'a> {
        NamedGenericType { name, gt: self }
    }

    pub fn cgmath_name(&self) -> String {
        // TODO: Use a formatter
        match self {
            Self::Item(inner_type) => inner_type.cgmath_name(),
            Self::Array(inner_type, _size) => format!("&[{}]", inner_type.cgmath_name()),
        }
    }

    pub fn rstype(&self) -> String {
        match self {
            Self::Item(inner_type) => inner_type.cgmath_name(),
            Self::Array(vector_type, _) => vector_type.rstype().to_owned(),
        }
    }

    pub fn components(&self) -> u32 {
        match self {
            Self::Item(_) => 1,
            Self::Array(inner_type, _) => inner_type.components(),
        }
    }

    pub fn glow_value(&self, name: &str) -> String {
        match self {
            Self::Item(_) => format!("&[{}]", name),
            Self::Array(inner_type, size) => format!(
                "::std::slice::from_raw_parts({name}.as_ptr() as *const {base_ty}, {size})",
                name = name,
                size = *size * self.components(),
                base_ty = inner_type.rstype()
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
