use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderKind {
    Vertex,
    Fragment,
    Compute,
}

#[cfg(feature = "shaderc")]
impl Into<shaderc::ShaderKind> for ShaderKind {
    fn into(self) -> shaderc::ShaderKind {
        match self {
            Self::Vertex => shaderc::ShaderKind::Vertex,
            Self::Fragment => shaderc::ShaderKind::Fragment,
            Self::Compute => shaderc::ShaderKind::Compute,
        }
    }
}

impl ShaderKind {
    pub fn from_path(p: impl AsRef<Path>) -> Option<Self> {
        if let Some(ext) = p.as_ref().extension() {
            return Some(match ext.to_str() {
                Some("vert") => Self::Vertex,
                Some("comp") => Self::Compute,
                Some("frag") => Self::Fragment,

                // TODO: Add other shader types
                _ => panic!("{}: unknown shader type", p.as_ref().to_string_lossy()),
            });
        }

        None
    }

    pub fn constant_name(&self) -> &'static str {
        match self {
            Self::Vertex => "VERTEX_SHADER",
            Self::Fragment => "FRAGMENT_SHADER",
            Self::Compute => "COMPUTE_SHADER",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::Vertex => "vert",
            Self::Fragment => "comp",
            Self::Compute => "frag",
        }
    }
}
