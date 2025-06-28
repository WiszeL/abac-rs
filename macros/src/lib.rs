// macros/src/lib.rs

mod derive;
mod register_entities;

use proc_macro::TokenStream;

use crate::{derive::derive_entity_impl, register_entities::register_entities_impl};

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    derive_entity_impl(input)
}

#[proc_macro]
pub fn register_entities(input: TokenStream) -> TokenStream {
    register_entities_impl(input)
}
