use crate::types::ItemOrArrayType;

#[derive(Debug, Default, Eq, Clone)]
pub struct FoundUniform {
    pub name: String,
    pub location: u32,
    pub ty: Option<ItemOrArrayType>,

    pub binding: Option<i32>,

    pub location_name: String,
}

impl FoundUniform {
    pub fn format(&self) -> Option<u32> {
        match self.ty {
            Some(ItemOrArrayType::Image { format }) => format,
            _ => None,
        }
    }
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
