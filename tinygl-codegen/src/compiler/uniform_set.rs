use std::collections::HashSet;
use std::iter::FromIterator;

use heck::{CamelCase, SnakeCase};

use quote::{format_ident, quote};

use tinygl_compiler::{reflect::FoundUniform, WrappedUniformSet};

use super::WrappedItem;
use crate::types::CodegenExt;

impl WrappedItem for WrappedUniformSet<'_, '_> {
    fn generate(&self) -> crate::Result<proc_macro2::TokenStream> {
        // Compute uniform set intersection
        let uniform_sets: Vec<HashSet<&FoundUniform>> = self
            .programs()
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
        let mut methods = Vec::new();

        // Write methods
        //
        // TODO: Some uniforms might have bindings and other might not. How should this be
        // represented in this API?
        for uniform in &unified {
            let ty = uniform.ty.unwrap();
            let sc = uniform.name.to_snake_case();
            let type_name: syn::Type = syn::parse_str(&ty.rust_value_type()).unwrap();

            if uniform.binding.is_some() {
                let ident = format_ident!("get_{}_binding", sc);

                methods.push(quote! {
                    fn #ident(&self) -> #type_name;
                });
            }

            let ident = format_ident!("set_{}", sc);
            let extra = ty.uniform_method_extra_args_with_ty().into_iter();

            if extra.clone().next().is_some() {
                methods.push(quote! {
                    fn #ident(&self, gl: &::tinygl::Context, #(#extra),* value: #type_name);
                });
            } else {
                methods.push(quote! {
                    fn #ident(&self, gl: &::tinygl::Context, value: #type_name);
                });
            }
        }

        let trait_name = format_ident!("{}UniformSet", self.id().to_camel_case());
        let set_trait = quote! {
            pub trait #trait_name {
                #(#methods)*
            }
        };

        // Write implementations for the known programs
        let mut set_impl = Vec::new();

        for program in self.programs().iter() {
            let mut methods = Vec::new();

            for uniform in &unified {
                let sc = uniform.name.to_snake_case();
                let ty = uniform.ty.unwrap();
                let type_name: syn::Type = syn::parse_str(&ty.rust_value_type()).unwrap();
                let struct_name = format_ident!("{}", program.struct_name());

                if uniform.binding.is_some() {
                    let ident = format_ident!("get_{}_binding", sc);

                    methods.push(quote! {
                        fn #ident(&self) -> #type_name {
                            #struct_name::#ident(self)
                        }
                    });
                }

                let ident = format_ident!("set_{}", sc);

                let extra_args = ty.uniform_method_extra_args_with_ty().into_iter();
                let extra_values = ty.uniform_method_extra_args_no_ty().into_iter();

                if extra_args.clone().next().is_some() {
                    methods.push(quote! {
                        fn #ident(&self, gl: &::tinygl::Context, #(#extra_args),*, value: #type_name) {
                            #struct_name::#ident(self, gl, #(#extra_values),*, value)
                        }
                    });
                } else {
                    methods.push(quote! {
                        fn #ident(&self, gl: &::tinygl::Context, value: #type_name) {
                            #struct_name::#ident(self, gl, value)
                        }
                    });
                }
            }

            let program_struct_name = format_ident!("{}", program.struct_name());

            set_impl.push(quote! {
                impl #trait_name for #program_struct_name {
                    #(#methods)*
                }
            });
        }

        Ok(quote! {
            #set_trait
            #(#set_impl)*
        })
    }
}
