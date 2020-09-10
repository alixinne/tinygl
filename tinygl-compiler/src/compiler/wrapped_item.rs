pub trait WrappedItem {
    fn generate(&self) -> crate::Result<proc_macro2::TokenStream>;
}
