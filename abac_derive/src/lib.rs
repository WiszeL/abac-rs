use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident; // struct name
    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => panic!("#[derive(Entity)] only works on structs"),
    };

    let mut arms = Vec::new();

    if let Fields::Named(named) = fields {
        for f in named.named {
            let fname = f.ident.expect("named field");
            let key = fname.to_string();
            let ty = &f.ty;

            // Pick the abac_rs::Value variant based on the field type:
            let variant = if is_string(ty) {
                quote!(abac_rs::Value::Str(&self.#fname))
            } else if is_integer(ty) {
                quote!(abac_rs::Value::Int(self.#fname as i32))
            } else if is_float(ty) {
                quote!(abac_rs::Value::Float(self.#fname as f32))
            } else if is_uuid(ty) {
                quote!(abac_rs::Value::Uuid(self.#fname))
            } else if is_bool(ty) {
                quote!(abac_rs::Value::Bool(self.#fname))
            } else {
                // unsupported → just return None for this field
                arms.push(quote! { #key => None, });
                continue; // skip the usual push below
            };

            arms.push(quote! { #key => Some(#variant), });
        }
    }

    let expanded = quote! {
        impl abac_rs::Entity for #ident {
            fn get_field(&self, field_name: &str) -> Option<abac_rs::Value> {
                match field_name {
                    #(#arms)*
                    _ => None,
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}

// helper predicates ---------------------------------------------------------
fn is_string(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.is_ident("String"))
}

fn is_integer(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.is_ident("i32"))
}

fn is_float(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.is_ident("f32"))
}

fn is_bool(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.is_ident("bool"))
}

fn is_uuid(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p)
        if p.path.segments.last().map(|s| s.ident == "Uuid").unwrap_or(false))
}
