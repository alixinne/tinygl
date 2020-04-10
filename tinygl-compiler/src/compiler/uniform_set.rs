use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::iter::FromIterator;
use std::path::Path;

use heck::CamelCase;
use heck::SnakeCase;

use super::{WrappedItem, WrappedProgram};
use crate::reflect::FoundUniform;

pub struct WrappedUniformSet<'p, 's> {
    /// Identifier for this uniform set
    id: String,
    /// Name of the Rust wrapper file for this set
    rs_file_name: String,
    /// Name of the target trait
    trait_name: String,
    /// List of wrapped programs
    programs: Vec<&'p WrappedProgram<'s>>,
}

impl<'p, 's> WrappedUniformSet<'p, 's> {
    pub fn new(programs: &[&'p WrappedProgram<'s>], id: &str) -> Self {
        let id = id.to_snake_case();
        let trait_name = (id.clone() + "_uniform_set").to_camel_case();
        let rs_file_name = trait_name.to_snake_case() + ".rs";

        Self {
            id,
            rs_file_name,
            trait_name,
            programs: programs.to_vec(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    fn write_rust_wrapper(&self, dest: impl AsRef<Path>) -> crate::Result<()> {
        // Write Rust program code
        let output_rs = File::create(&Path::new(dest.as_ref()).join(&self.rs_file_name))?;
        let mut wr = BufWriter::new(output_rs);

        // Compute uniform set intersection
        let uniform_sets: Vec<HashSet<&FoundUniform>> = self
            .programs
            .iter()
            .map(|program| {
                HashSet::<&FoundUniform>::from_iter(
                    program
                        .shaders_with_uniforms()
                        .flat_map(|shader| shader.uniforms()),
                )
            })
            .collect();

        // Clone the first set
        let mut unified = uniform_sets
            .first()
            .map(Clone::clone)
            .unwrap_or_else(HashSet::new);
        for others in uniform_sets.iter().skip(1) {
            unified = HashSet::from_iter(others.intersection(&unified).copied());
        }

        // Turn it into a vec, sort by name
        let mut unified: Vec<_> = unified.into_iter().collect();
        unified.sort_by_key(|f| &f.name);

        // Write trait declaration
        writeln!(wr, "pub trait {} {{", self.trait_name)?;
        // Write methods
        //
        // TODO: Some uniforms might have bindings and other might not. How should this be
        // represented in this API?
        for uniform in &unified {
            let ty = uniform.ty.unwrap();

            if uniform.binding.is_some() {
                writeln!(
                    wr,
                    "    fn get_{uniform_sc_name}_binding(&self) -> {type_name};",
                    uniform_sc_name = uniform.name.to_snake_case(),
                    type_name = ty.rstype()
                )?;
            }

            writeln!(
                wr,
                "    fn set_{uniform_sc_name}(&self, gl: &::tinygl::Context, value: {type_name});",
                uniform_sc_name = uniform.name.to_snake_case(),
                type_name = ty.cgmath_name()
            )?;
        }
        writeln!(wr, "}}")?;

        // Write implementations for the known programs
        for program in &self.programs {
            writeln!(
                wr,
                "impl {trait_name} for {program_struct_name} {{",
                trait_name = self.trait_name,
                program_struct_name = program.struct_name()
            )?;

            for uniform in &unified {
                let ty = uniform.ty.unwrap();

                if uniform.binding.is_some() {
                    writeln!(
                        wr,
                        "    fn get_{uniform_sc_name}_binding(&self) -> {type_name} {{",
                        uniform_sc_name = uniform.name.to_snake_case(),
                        type_name = ty.rstype()
                    )?;
                    writeln!(
                        wr,
                        "        {struct_name}::get_{uniform_sc_name}_binding(self)",
                        struct_name = program.struct_name(),
                        uniform_sc_name = uniform.name.to_snake_case()
                    )?;
                    writeln!(wr, "    }}")?;
                }

                writeln!(
                    wr,
                    "    fn set_{uniform_sc_name}(&self, gl: &::tinygl::Context, value: {type_name}) {{",
                    uniform_sc_name = uniform.name.to_snake_case(),
                    type_name = ty.cgmath_name()
                )?;
                writeln!(
                    wr,
                    "        {struct_name}::set_{uniform_sc_name}(self, gl, value)",
                    struct_name = program.struct_name(),
                    uniform_sc_name = uniform.name.to_snake_case()
                )?;
                writeln!(wr, "    }}")?;
            }

            writeln!(wr, "}}")?;
        }

        Ok(())
    }
}

impl WrappedItem for WrappedUniformSet<'_, '_> {
    fn write(&self, dest: &Path) -> Result<(), crate::Error> {
        self.write_rust_wrapper(dest)
    }

    fn write_root_include(&self, wr: &mut dyn std::io::Write) -> Result<(), crate::Error> {
        writeln!(wr, "// {}", self.id)?;
        writeln!(wr, "include!(\"{}\");", self.rs_file_name)?;
        Ok(())
    }
}
