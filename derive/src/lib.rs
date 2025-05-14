// src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(AbacEntity)]
pub fn derive_abac_entity(input: TokenStream) -> TokenStream {
    derive_
}
