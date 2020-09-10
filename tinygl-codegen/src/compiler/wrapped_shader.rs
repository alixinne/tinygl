use heck::SnakeCase;

use quote::{format_ident, quote};

use tinygl_compiler::{TargetType, WrappedShader};

use super::WrappedItem;
use crate::types::{CodegenExt, UniformValueExt};

impl WrappedItem for WrappedShader {
    fn generate(&self) -> crate::Result<proc_macro2::TokenStream> {
        let shader_tokens = match self.output_type() {
            TargetType::SpirV => {
                // Just write spv file
                let res =
                    syn::LitByteStr::new(&self.binary_result_u8(), proc_macro2::Span::call_site());
                quote! { #res }
            }
            TargetType::Glsl(version) => {
                let out = if self.skip_spirv() {
                    use std::fmt::Write;

                    // We skipped SPIR-V generation so just fix invalid stuff for OpenGL ES targets
                    // WebGL is more sensitive to leftovers from includes and stuff
                    // TODO: This is an ugly hack, maybe forbid skip_spirv + ES 3.00?
                    let mut output = String::new();

                    for l in self.text_result().lines() {
                        if l.starts_with("#extension GL_GOOGLE_include_directive") {
                            continue;
                        } else if l.starts_with("#line") {
                            writeln!(output, "//{}", l).ok();
                        } else {
                            writeln!(output, "{}", l).ok();
                        }
                    }

                    output
                } else {
                    self.transpile(version)?
                };

                quote! { #out }
            }
            _ => unreachable!(),
        };

        // Shader resource structure
        let struct_name = format_ident!("{}", self.shader_struct_name());
        let kind_constant_name = format_ident!("{}", self.kind().constant_name);
        let st: syn::Type = syn::parse_str(if self.output_type().is_source() {
            "::tinygl::wrappers::SourceShader"
        } else {
            "::tinygl::wrappers::BinaryShader"
        })
        .unwrap();

        let mut parts = Vec::new();

        parts.push(quote! {
            pub struct #struct_name {
                name: tinygl::gl::Shader,
            }

            impl #struct_name {
                pub fn build(gl: &::tinygl::Context) -> ::tinygl::Result<Self> {
                    Ok(Self {
                        name: <Self as #st>::build(gl, ::tinygl::gl::#kind_constant_name)?
                    })
                }
            }

            impl ::tinygl::wrappers::ShaderCommon for #struct_name {
                fn kind(&self) -> u32 {
                    ::tinygl::gl::#kind_constant_name
                }

                fn name(&self) -> ::tinygl::gl::Shader {
                    self.name
                }
            }

            impl ::tinygl::wrappers::GlDrop for #struct_name {
                unsafe fn drop(&mut self, gl: &::tinygl::Context) {
                    use ::tinygl::wrappers::ShaderCommon;
                    gl.delete_shader(self.name());
                }
            }
        });

        // Implement the right shader trait for the given output type
        if self.output_type().is_source() {
            parts.push(quote! {
                impl ::tinygl::wrappers::SourceShader<'static> for #struct_name {
                    fn get_source() -> &'static str {
                        #shader_tokens
                    }
                }
            });
        } else {
            parts.push(quote! {
                impl ::tinygl::wrappers::BinaryShader<'static> for #struct_name {
                    fn get_binary() -> &'static [u8] {
                        #shader_tokens
                    }
                }
            });
        }

        // Write struct for holding uniform locations
        let struct_name = format_ident!("{}", self.uniform_struct_name());
        let uniform_location_name: Vec<_> = self
            .uniforms()
            .iter()
            .map(|u| format_ident!("{}", u.location_name()))
            .collect();

        parts.push(quote! {
            #[derive(Default)]
            pub struct #struct_name {
                #(#uniform_location_name: Option<::tinygl::gl::UniformLocation>,)*
            }
        });

        let mut methods = Vec::new();

        // Write constructor
        let uset = self
            .uniforms()
            .iter()
            .map(|uniform| {
                let name = format_ident!("{}", uniform.location_name());

                if self.output_type().is_source() {
                    // Source shader: find uniform locations from variable names
                    let uniform_name = uniform.name.as_str();
                    quote! { #name: unsafe { gl.get_uniform_location(program, #uniform_name) } }
                } else {
                    // Binary shader: assume locations form reflection on SPIR-V
                    let location = uniform.location;
                    quote! { #name: Some(#location) }
                }
            })
            .collect::<Vec<_>>();

        methods.push(quote! {
            pub fn new(gl: &::tinygl::Context, program: ::tinygl::gl::Program) -> Self {
                Self {
                    #(#uset),*
                }
            }
        });

        // Write getter/setter methods
        methods.extend(self.uniforms().iter().flat_map(|uniform| {
            let mut res = Vec::new();
            let ty = uniform.ty.unwrap();
            let type_name: syn::Type = syn::parse_str(&ty.rust_value_type()).unwrap();

            if let Some(binding) = uniform.binding {
                let meth_ident = format_ident!("get_{}_binding", uniform.name.to_snake_case());

                res.push(quote! {
                    pub fn #meth_ident(&self) -> #type_name {
                        #binding
                    }
                });
            }

            let meth_ident = format_ident!("set_{}", uniform.name.to_snake_case());
            let location = format_ident!("{}", uniform.location_name());
            let program_uniform = format_ident!("program_uniform{}", ty.uniform_method_name());
            let mut meth_args = vec![
                quote! { value: #type_name }
            ];

            if let Some(extra) = ty.uniform_method_extra_args_with_ty() {
                meth_args.insert(0, extra);
            }

            let mut call_args = Vec::new();

            if let Some(count) = ty.uniform_count_arg() {
                call_args.push(quote! { #count });
            }

            if let Some(extra) = ty.uniform_method_extra_args_no_ty() {
                call_args.push(extra);
            }

            call_args.push(ty.uniform_value(&format_ident!("value")));

            res.push(quote! {
                pub fn #meth_ident(&self, gl: &::tinygl::Context, program: ::tinygl::gl::ProgramName, #(#meth_args),*) {
                    if let Some(location) = self.#location {
                        unsafe {
                            gl.#program_uniform(program, location, #(#call_args),*);
                        }
                    }
                }
            });

            res
        }));

        let struct_name = self.uniform_struct_name();
        Ok(quote! {
            impl #struct_name {
                #(#methods)*
            }
        })
    }
}
