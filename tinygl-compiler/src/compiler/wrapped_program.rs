use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use heck::CamelCase;
use heck::SnakeCase;

use super::wrapped_shader::*;
use super::WrappedItem;

use crate::types::prelude::*;

pub struct WrappedProgram<'s> {
    id: String,
    struct_name: String,
    rs_file_name: String,
    attached_shaders: Vec<&'s WrappedShader>,
}

impl<'s> WrappedProgram<'s> {
    pub fn new(program_name: &str, attached_shaders: &[&'s WrappedShader]) -> Self {
        let id = program_name.to_snake_case();
        let struct_name = program_name.to_camel_case() + "Program";
        let rs_file_name = struct_name.to_snake_case() + ".rs";

        Self {
            id,
            struct_name,
            rs_file_name,
            attached_shaders: attached_shaders.to_vec(),
        }
    }

    pub fn struct_name(&self) -> &str {
        &self.struct_name
    }

    pub fn shaders(&self) -> impl Iterator<Item = &&'s WrappedShader> {
        self.attached_shaders.iter()
    }

    pub fn shaders_with_uniforms(&self) -> impl Iterator<Item = &&'s WrappedShader> {
        self.attached_shaders
            .iter()
            .filter(|s| !s.uniforms().is_empty())
    }

    fn write_rust_wrapper(&self, dest: impl AsRef<Path>) -> crate::Result<()> {
        // Write Rust program code
        let output_rs = File::create(&Path::new(dest.as_ref()).join(&self.rs_file_name))?;
        let mut wr = BufWriter::new(output_rs);

        writeln!(wr, "pub struct {} {{", self.struct_name)?;
        // Program name handle
        writeln!(wr, "    name: ::tinygl::gl::Program,")?;
        // Write uniform handles
        for shader in self.shaders_with_uniforms() {
            writeln!(
                wr,
                "    {}: {},",
                shader.uniform_locations_name(),
                shader.uniform_struct_name()
            )?;
        }
        writeln!(wr, "}}")?;

        writeln!(wr, "impl {} {{", self.struct_name)?;
        // Constructor function
        writeln!(wr, "    pub fn new(gl: &::tinygl::Context,")?;
        // Add shader parameters
        for shader in self.shaders() {
            writeln!(
                wr,
                "               {param_name}: &{param_type},",
                param_name = shader.shader_variable_name(),
                param_type = shader.shader_struct_name()
            )?;
        }
        writeln!(wr, "              ) -> ::tinygl::Result<Self> {{")?;
        writeln!(
            wr,
            "        let program_name = ::tinygl::wrappers::RuntimeProgramBuilder::new(gl)"
        )?;
        for shader in self.shaders() {
            writeln!(wr, "            .shader({})", shader.shader_variable_name())?;
        }
        writeln!(wr, "            .build()?")?;
        writeln!(wr, "            .into_inner();")?;
        writeln!(wr, "        Ok(Self {{")?;
        writeln!(wr, "            name: program_name,")?;
        for shader in self.shaders_with_uniforms() {
            writeln!(
                wr,
                "            {}: {}::new(gl, program_name),",
                shader.uniform_locations_name(),
                shader.uniform_struct_name()
            )?;
        }
        writeln!(wr, "        }})")?;
        writeln!(wr, "    }}")?;
        // Write builder (constructs shaders and then calls the constructor)
        writeln!(
            wr,
            "    pub fn build(gl: &::tinygl::Context) -> ::tinygl::Result<Self> {{"
        )?;
        for shader in self.shaders() {
            writeln!(
                wr,
                "        let {} = ::tinygl::wrappers::GlRefHandle::new(gl, {}::build(gl)?);",
                shader.shader_variable_name(),
                shader.shader_struct_name()
            )?;
        }
        writeln!(wr, "        Ok(Self::new(")?;
        writeln!(wr, "            gl,")?;
        for shader in self.shaders() {
            writeln!(
                wr,
                "            {name}.as_ref(),",
                name = shader.shader_variable_name(),
            )?;
        }
        writeln!(wr, "        )?)")?;
        writeln!(wr, "    }}")?;

        // List of seen uniforms, since uniform names are unique
        let mut known = std::collections::HashSet::new();

        // Uniform getters/setters for the included shaders
        for shader in self.shaders_with_uniforms() {
            for uniform in shader.uniforms() {
                let ty = uniform.ty.unwrap();

                // Skip this uniform if it has been added already
                if known.contains(&uniform.name) {
                    continue;
                } else {
                    known.insert(&uniform.name);
                }

                writeln!(
                    wr,
                    "    pub fn set_{uniform_sc_name}(&self, gl: &::tinygl::Context, {extra}value: {type_name}) {{",
                    uniform_sc_name = uniform.name.to_snake_case(),
                    type_name = ty.rust_value_type(),
                    extra = ty.uniform_method_extra_args_with_ty().map_or_else(|| String::new(), |x| format!("{}, ", x)),
                )?;

                writeln!(
                    wr,
                    "        self.{location_name}.set_{uniform_sc_name}(gl, self.name, {extra}value);",
                    location_name = shader.uniform_locations_name(),
                    uniform_sc_name = uniform.name.to_snake_case(),
                    extra = ty
                        .uniform_method_extra_args_no_ty()
                        .map_or_else(|| String::new(), |x| format!("{}, ", x)),
                )?;

                writeln!(wr, "    }}")?;

                if let Some(binding) = uniform.binding {
                    writeln!(
                        wr,
                        "    pub fn get_{uniform_sc_name}_binding(&self) -> {type_name} {{",
                        uniform_sc_name = uniform.name.to_snake_case(),
                        type_name = ty.rust_value_type(),
                    )?;

                    writeln!(wr, "        {}", binding)?;
                    writeln!(wr, "    }}")?;
                }
            }
        }
        writeln!(wr, "}}")?;

        // Implement ProgramCommon
        writeln!(
            wr,
            "impl ::tinygl::wrappers::ProgramCommon for {} {{",
            self.struct_name
        )?;
        // Name getter
        writeln!(wr, "    fn name(&self) -> ::tinygl::gl::Program {{")?;
        writeln!(wr, "        self.name")?;
        writeln!(wr, "    }}")?;
        writeln!(wr, "}}")?;

        // Implement GlDrop
        writeln!(
            wr,
            "impl ::tinygl::wrappers::GlDrop for {} {{",
            self.struct_name
        )?;
        writeln!(
            wr,
            "    unsafe fn drop(&mut self, gl: &::tinygl::Context) {{"
        )?;
        writeln!(wr, "        use ::tinygl::wrappers::ProgramCommon;")?;
        writeln!(wr, "        gl.delete_program(self.name());")?;
        writeln!(wr, "    }}")?;
        writeln!(wr, "}}")?;

        Ok(())
    }
}

impl WrappedItem for WrappedProgram<'_> {
    fn write(&self, dest: &Path) -> Result<(), crate::Error> {
        self.write_rust_wrapper(dest)
    }

    fn write_root_include(&self, wr: &mut dyn Write) -> Result<(), crate::Error> {
        writeln!(wr, "// {} program", self.id)?;
        writeln!(wr, "include!(\"{}\");", self.rs_file_name)?;
        Ok(())
    }
}
