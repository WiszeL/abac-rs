// macros/src/lib.rs

mod derive;

use proc_macro::TokenStream;

use crate::derive::derive_entity_impl;

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    derive_entity_impl(input)
}
