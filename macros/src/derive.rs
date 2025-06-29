use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

pub fn derive_entity_impl(input: TokenStream) -> TokenStream {
    // 1. Parse the input
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = &ast.ident;

    // 2. Figure out whether to refer to "crate" or "abac_rs"
    let crate_name = match crate_name("abac-rs") {
        // If the crate name maps to itself, use "crate"
        Ok(FoundCrate::Itself) => "crate".to_string(),
        // If it's been renamed in Cargo.toml, use that name
        Ok(FoundCrate::Name(name)) => name,
        // If it's not in Cargo.toml (e.g. running inside the core crate itself),
        // fall back to "crate"
        Err(_) => "crate".to_string(),
    };
    let crate_ident = syn::Ident::new(&crate_name, proc_macro2::Span::call_site());

    // 3. Grab the struct’s named fields
    let fields = match &ast.data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Named(f) => &f.named,
            _ => panic!("#[derive(Entity)] only works on named‐field structs"),
        },
        _ => panic!("#[derive(Entity)] only works on structs"),
    };

    // 4. Build the field‐name slice
    let idents = fields.iter().map(|f| f.ident.as_ref().unwrap());
    let names = idents.clone().map(|i| i.to_string());

    let slice_ident = format_ident!("__{}_FIELD_NAMES", struct_ident.to_string().to_uppercase());
    let gen_field_names = quote! {
        const #slice_ident: &[&'static str] = &[ #(#names),* ];
    };

    // 5. Build the map‐insertion for into_value
    let gen_inserts = idents.clone().map(|ident| {
        let fname = ident.to_string();
        quote! {
            map.insert(
                #fname.to_string(),
                #crate_ident::serde_value::to_value(&self.#ident)
                    .expect("#crate_ident::serde_value::to_value failed")
            );
        }
    });

    // 6. Emit the impl, referring to either `crate::Entity` or `abac_rs::Entity`
    let expanded = quote! {
        #gen_field_names

        impl #crate_ident::Entity for #struct_ident {
            fn to_value(&self) -> Result<
                std::collections::HashMap<String, #crate_ident::serde_value::Value>,
                #crate_ident::Error
            > {
                let mut map = std::collections::HashMap::new();
                #(#gen_inserts)*
                Ok(map)
            }

            fn field_names(&self) -> &'static [&'static str] {
                #slice_ident
            }
        }
    };

    TokenStream::from(expanded)
}
