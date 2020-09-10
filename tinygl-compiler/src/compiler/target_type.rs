use std::fmt;

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum GlslVersion {
    V1_10,
    V1_20,
    V1_30,
    V1_40,
    V1_50,
    V3_30,
    V4_00,
    V4_10,
    V4_20,
    V4_30,
    V4_40,
    V4_50,
    V4_60,
    V1_00Es,
    V3_00Es,
}

impl fmt::Display for GlslVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V1_10 => write!(f, "1.10"),
            Self::V1_20 => write!(f, "1.20"),
            Self::V1_30 => write!(f, "1.30"),
            Self::V1_40 => write!(f, "1.40"),
            Self::V1_50 => write!(f, "1.50"),
            Self::V3_30 => write!(f, "3.30"),
            Self::V4_00 => write!(f, "4.00"),
            Self::V4_10 => write!(f, "4.10"),
            Self::V4_20 => write!(f, "4.20"),
            Self::V4_30 => write!(f, "4.30"),
            Self::V4_40 => write!(f, "4.40"),
            Self::V4_50 => write!(f, "4.50"),
            Self::V4_60 => write!(f, "4.60"),
            Self::V1_00Es => write!(f, "1.00 es"),
            Self::V3_00Es => write!(f, "3.00 es"),
        }
    }
}

#[cfg(feature = "spirv_cross")]
impl From<spirv_cross::glsl::Version> for GlslVersion {
    fn from(version: spirv_cross::glsl::Version) -> Self {
        match version {
            spirv_cross::glsl::Version::V1_10 => Self::V1_10,
            spirv_cross::glsl::Version::V1_20 => Self::V1_20,
            spirv_cross::glsl::Version::V1_30 => Self::V1_30,
            spirv_cross::glsl::Version::V1_40 => Self::V1_40,
            spirv_cross::glsl::Version::V1_50 => Self::V1_50,
            spirv_cross::glsl::Version::V3_30 => Self::V3_30,
            spirv_cross::glsl::Version::V4_00 => Self::V4_00,
            spirv_cross::glsl::Version::V4_10 => Self::V4_10,
            spirv_cross::glsl::Version::V4_20 => Self::V4_20,
            spirv_cross::glsl::Version::V4_30 => Self::V4_30,
            spirv_cross::glsl::Version::V4_40 => Self::V4_40,
            spirv_cross::glsl::Version::V4_50 => Self::V4_50,
            spirv_cross::glsl::Version::V4_60 => Self::V4_60,
            spirv_cross::glsl::Version::V1_00Es => Self::V1_00Es,
            spirv_cross::glsl::Version::V3_00Es => Self::V3_00Es,
        }
    }
}

#[cfg(feature = "spirv_cross")]
impl Into<spirv_cross::glsl::Version> for GlslVersion {
    fn into(self) -> spirv_cross::glsl::Version {
        match self {
            Self::V1_10 => spirv_cross::glsl::Version::V1_10,
            Self::V1_20 => spirv_cross::glsl::Version::V1_20,
            Self::V1_30 => spirv_cross::glsl::Version::V1_30,
            Self::V1_40 => spirv_cross::glsl::Version::V1_40,
            Self::V1_50 => spirv_cross::glsl::Version::V1_50,
            Self::V3_30 => spirv_cross::glsl::Version::V3_30,
            Self::V4_00 => spirv_cross::glsl::Version::V4_00,
            Self::V4_10 => spirv_cross::glsl::Version::V4_10,
            Self::V4_20 => spirv_cross::glsl::Version::V4_20,
            Self::V4_30 => spirv_cross::glsl::Version::V4_30,
            Self::V4_40 => spirv_cross::glsl::Version::V4_40,
            Self::V4_50 => spirv_cross::glsl::Version::V4_50,
            Self::V4_60 => spirv_cross::glsl::Version::V4_60,
            Self::V1_00Es => spirv_cross::glsl::Version::V1_00Es,
            Self::V3_00Es => spirv_cross::glsl::Version::V3_00Es,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetType {
    SpirV,
    Glsl(GlslVersion),
}

impl TargetType {
    pub fn is_source(self) -> bool {
        match self {
            TargetType::Glsl(_) => true,
            TargetType::SpirV => false,
        }
    }
}
