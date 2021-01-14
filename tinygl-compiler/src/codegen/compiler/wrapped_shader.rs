use heck::SnakeCase;
use quote::{format_ident, quote};

use crate::{
    codegen::types::{CodegenExt, UniformValueExt},
    model::AsOutputFormat,
    Error, WrappedShader, WrappedShaderDetails,
};

use super::WrappedItem;

fn filter_src<T: AsOutputFormat>(this: &WrappedShader<T>) -> String {
    use std::fmt::Write;

    // We skipped SPIR-V generation so just fix invalid stuff for OpenGL ES targets
    // WebGL is more sensitive to leftovers from includes and stuff
    // TODO: This is an ugly hack, maybe forbid skip_spirv + ES 3.00?
    let mut output = String::new();

    for l in this.result().object().as_source().unwrap().as_str().lines() {
        if l.starts_with("#extension GL_GOOGLE_include_directive") {
            continue;
        } else if l.starts_with("#line") {
            writeln!(output, "//{}", l).ok();
        } else {
            writeln!(output, "{}", l).ok();
        }
    }

    output
}

#[cfg(feature = "spirv")]
fn get_shader_tokens<T: AsOutputFormat>(
    this: &WrappedShader<T>,
) -> crate::Result<proc_macro2::TokenStream> {
    if this.prefer_spirv() && this.result().as_spirv().is_some() {
        // Just write spv file
        let res = syn::LitByteStr::new(
            &this
                .result()
                .object()
                .as_spirv()
                .ok_or(Error::SpirVObjectRequired)?
                .as_bytes_u8(),
            proc_macro2::Span::call_site(),
        );
        Ok(quote! { #res })
    } else {
        let out = filter_src(this);
        Ok(quote! { #out })
    }
}

#[cfg(not(feature = "spirv"))]
fn get_shader_tokens<T: AsOutputFormat>(
    this: &WrappedShader<T>,
) -> crate::Result<proc_macro2::TokenStream> {
    let out = filter_src(this);
    Ok(quote! { #out })
}

fn to_cstr(s: &str) -> proc_macro2::TokenStream {
    let mut s_null_terminated = s.to_string();
    s_null_terminated.push('\0');

    let s_lit = syn::LitByteStr::new(s_null_terminated.as_bytes(), proc_macro2::Span::call_site());
    quote! { ::std::mem::transmute(#s_lit as *const u8) }
}

impl<T: AsOutputFormat> WrappedItem for WrappedShader<T> {
    fn generate(&self) -> crate::Result<proc_macro2::TokenStream> {
        let shader_tokens = get_shader_tokens(self)?;
        let is_source = !self.prefer_spirv() && self.result().as_source().is_some();

        // Shader resource structure
        let struct_name = format_ident!("{}", self.shader_struct_name());
        let kind_constant_name =
            format_ident!("{}", self.result().object().info().kind.constant_name());
        let st: syn::Type = syn::parse_str(if is_source {
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
        if is_source {
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

                if is_source {
                    // Source shader: find uniform locations from variable names
                    let uniform_name = to_cstr(uniform.name.as_str());
                    quote! { #name: unsafe { let loc = gl.get_uniform_location(program, #uniform_name); if loc < 0 { None } else { Some(loc) } } }
                } else {
                    // Binary shader: assume locations form reflection on SPIR-V
                    let location = syn::Lit::Int(syn::LitInt::new(&uniform.location.to_string(),
                                                                  proc_macro2::Span::call_site()));
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
            let sc = uniform.name.to_snake_case();

            if let Some(binding) = uniform.binding {
                let meth_ident = format_ident!("get_{}_binding", sc);
                let binding = binding as u32;

                res.push(quote! {
                    pub fn #meth_ident(&self) -> #type_name {
                        #binding
                    }
                });
            }

            if let Some(format) = uniform.format() {
                let ident = format_ident!("get_{}_format", sc);

                res.push(quote! {
                    pub fn #ident(&self) -> u32 {
                        #format
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
                let count = syn::LitInt::new(&format!("{}", count), proc_macro2::Span::call_site());
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

        Ok(quote! {
            #(#parts)*

            impl #struct_name {
                #(#methods)*
            }
        })
    }
}
