use std::path::Path;

#[derive(Debug)]
pub struct ShaderKindInfo {
    pub shaderc_kind: shaderc::ShaderKind,
    pub constant_name: &'static str,
    pub extension: &'static str,
}

impl ShaderKindInfo {
    pub fn from_path(p: impl AsRef<Path>) -> Option<Self> {
        if let Some(ext) = p.as_ref().extension() {
            return Some(match ext.to_str() {
                Some("vert") => Self::from(shaderc::ShaderKind::Vertex),
                Some("comp") => Self::from(shaderc::ShaderKind::Compute),
                Some("frag") => Self::from(shaderc::ShaderKind::Fragment),

                // TODO: Add other shader types
                _ => panic!("{}: unknown shader type", p.as_ref().to_string_lossy()),
            });
        }

        None
    }
}

impl From<shaderc::ShaderKind> for ShaderKindInfo {
    fn from(kind: shaderc::ShaderKind) -> Self {
        use shaderc::ShaderKind::*;

        match kind {
            Vertex => Self {
                shaderc_kind: kind,
                constant_name: "VERTEX_SHADER",
                extension: "vert",
            },
            Compute => Self {
                shaderc_kind: kind,
                constant_name: "COMPUTE_SHADER",
                extension: "comp",
            },
            Fragment => Self {
                shaderc_kind: kind,
                constant_name: "FRAGMENT_SHADER",
                extension: "frag",
            },

            // TODO: Add other shader types
            _ => panic!("{:?}: unsupported shader type", kind),
        }
    }
}
