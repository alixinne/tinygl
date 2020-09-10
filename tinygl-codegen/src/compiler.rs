pub trait WrappedItem {
    fn generate(&self) -> crate::Result<proc_macro2::TokenStream>;
}

mod uniform_set;
pub use uniform_set::*;

mod wrapped_program;
pub use wrapped_program::*;

mod wrapped_shader;
pub use wrapped_shader::*;
