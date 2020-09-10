use heck::SnakeCase;

use super::WrappedProgram;

pub struct WrappedUniformSet<'p, 's> {
    /// Identifier for this uniform set
    id: String,
    /// List of wrapped programs
    programs: Vec<&'p WrappedProgram<'s>>,
}

impl<'p, 's> WrappedUniformSet<'p, 's> {
    pub fn new(programs: &[&'p WrappedProgram<'s>], id: &str) -> Self {
        let id = id.to_snake_case();

        Self {
            id,
            programs: programs.to_vec(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn programs(&self) -> &[&'p WrappedProgram<'s>] {
        &self.programs
    }
}
