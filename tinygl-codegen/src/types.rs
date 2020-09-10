mod codegen_ext;
pub use codegen_ext::*;

use quote::quote;

use tinygl_compiler::types::{GenericType, ItemOrArrayType};

pub trait UniformValueExt {
    fn uniform_value(&self, name: &syn::Ident) -> proc_macro2::TokenStream;
}

impl UniformValueExt for GenericType {
    fn uniform_value(&self, name: &syn::Ident) -> proc_macro2::TokenStream {
        match self {
            Self::Atom(_) => quote! { #name },
            Self::Vector(_) => quote! { #name.as_ref().as_ptr() },
            Self::Matrix(_) => quote! { #name.as_ref().as_ptr() as *const _ },
        }
    }
}

impl UniformValueExt for ItemOrArrayType {
    fn uniform_value(&self, name: &syn::Ident) -> proc_macro2::TokenStream {
        match self {
            Self::Item(inner) => inner.uniform_value(name),
            Self::Array(_, _) => quote! { #name.as_ref().as_ptr() },
        }
    }
}
