use heck::SnakeCase;
use quote::{format_ident, quote};

use crate::{codegen::types::CodegenExt, WrappedProgram};

use super::WrappedItem;

impl WrappedItem for WrappedProgram<'_> {
    fn generate(&self) -> crate::Result<proc_macro2::TokenStream> {
        // Write program struct
        let struct_name = format_ident!("{}", self.struct_name());
        let uniform_locations_name: Vec<_> = self
            .shaders_with_uniforms()
            .map(|shader| format_ident!("{}", shader.uniform_locations_name()))
            .collect();
        let uniform_struct_name: Vec<_> = self
            .shaders_with_uniforms()
            .map(|shader| format_ident!("{}", shader.uniform_struct_name()))
            .collect();

        let prog_struct = quote! {
            pub struct #struct_name {
                // Program name handle
                name: ::tinygl::gl::Program,
                // Uniform handles
                #(#uniform_locations_name: #uniform_struct_name),*
            }
        };

        let mut methods = Vec::new();

        let shader_variable_name: Vec<_> = self
            .shaders()
            .map(|s| format_ident!("{}", s.shader_variable_name()))
            .collect();
        let shader_struct_name: Vec<_> = self
            .shaders()
            .map(|s| format_ident!("{}", s.shader_struct_name()))
            .collect();

        // Constructor function
        methods.push(quote! {
            pub fn new(gl: &::tinygl::Context, #(#shader_variable_name: &#shader_struct_name),*) -> ::tinygl::Result<Self> {
                let program_name = ::tinygl::wrappers::RuntimeProgramBuilder::new(gl)
                    #(.shader(#shader_variable_name))*
                    .build()?
                    .into_inner();

                Ok(Self {
                    name: program_name,
                    #(#uniform_locations_name: #uniform_struct_name::new(gl, program_name)),*
                })
            }
        });

        // Write builder (constructs shaders and then calls the constructor)
        methods.push(quote! {
            pub fn build(gl: &::tinygl::Context) -> ::tinygl::Result<Self> {
                #(let #shader_variable_name = ::tinygl::wrappers::GlRefHandle::new(gl, #shader_struct_name::build(gl)?);)*

                Ok(Self::new(
                    gl,
                    #(#shader_variable_name.as_ref()),*
                )?)
            }
        });

        // List of seen uniforms, since uniform names are unique
        let mut known = std::collections::HashSet::new();

        // Uniform getters/setters for the included shaders
        for shader in self.shaders_with_uniforms() {
            for uniform in shader.uniforms() {
                let ty = uniform.ty.unwrap();
                let sc = uniform.name.to_snake_case();
                let type_name: syn::Type = syn::parse_str(&ty.rust_value_type()).unwrap();

                // Skip this uniform if it has been added already
                if known.contains(&uniform.name) {
                    continue;
                } else {
                    known.insert(&uniform.name);
                }

                let ident = format_ident!("set_{}", sc);
                let extra_args = ty.uniform_method_extra_args_with_ty().into_iter();
                let extra_values = ty.uniform_method_extra_args_no_ty().into_iter();
                let location_name = format_ident!("{}", shader.uniform_locations_name());

                if extra_args.clone().next().is_some() {
                    methods.push(quote! {
                        pub fn #ident(&self, gl: &::tinygl::Context, #(#extra_args),* value: #type_name) {
                            self.#location_name.#ident(gl, self.name, #(#extra_values)*,, value);
                        }
                    });
                } else {
                    methods.push(quote! {
                        pub fn #ident(&self, gl: &::tinygl::Context, value: #type_name) {
                            self.#location_name.#ident(gl, self.name, value);
                        }
                    });
                }

                if let Some(binding) = uniform.binding {
                    let ident = format_ident!("get_{}_binding", sc);
                    let binding = binding as u32;

                    methods.push(quote! {
                        pub fn #ident(&self) -> #type_name {
                            #binding
                        }
                    });
                }

                if let Some(format) = uniform.format() {
                    let ident = format_ident!("get_{}_format", sc);

                    methods.push(quote! {
                        pub fn #ident(&self) -> u32 {
                            #format
                        }
                    });
                }
            }
        }

        Ok(quote! {
            #prog_struct

            impl #struct_name {
                #(#methods)*
            }

            impl ::tinygl::wrappers::ProgramCommon for #struct_name {
                fn name(&self) -> ::tinygl::gl::Program {
                    self.name
                }
            }

            impl ::tinygl::wrappers::GlDrop for #struct_name {
                unsafe fn drop(&mut self, gl: &::tinygl::Context) {
                    use ::tinygl::wrappers::ProgramCommon;
                    gl.delete_program(self.name());
                }
            }
        })
    }
}
