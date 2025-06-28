use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, Ident, Token, Type};

/// (Entity, Provider)
struct Pair {
    entity: Ident,
    _comma: Token![,],
    provider: Type,
}

impl syn::parse::Parse for Pair {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        Ok(Pair {
            entity: content.parse()?,
            _comma: content.parse()?,
            provider: content.parse()?,
        })
    }
}

pub fn register_entities_impl(input: TokenStream) -> TokenStream {
    let pairs =
        parse_macro_input!(input with Punctuated::<Pair, Token![,]>::parse_terminated);

    // Enum variants like Task(Task)
    let variants = pairs.iter().map(|p| {
        let ident = &p.entity;
        quote!( #ident(#ident) )
    });

    // Tokens for the hidden PAIRS array
    let pair_tokens = pairs.iter().map(|p| {
        let entity = &p.entity;
        let provider = &p.provider;
        quote! { ( stringify!(#entity), stringify!(#provider) ) }
    });

    let enum_ident = format_ident!("Entities");

    let expanded = quote! {
        // ---- GENERATED ENUM ----
        pub enum #enum_ident {
            #(#variants),*
        }
        pub use #enum_ident as Entities;

        // ---- Stash list for later build steps ----
        #[doc(hidden)]
        pub mod __abac_pairs {
            pub const PAIRS: &[(&str, &str)] = &[
                #(#pair_tokens),*
            ];
        }
    };

    TokenStream::from(expanded)
}
