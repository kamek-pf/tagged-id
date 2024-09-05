extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, NestedMeta};

#[proc_macro_derive(Identify, attributes(id_type))]
pub fn derive_identify(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let name = &input.ident;

    // Extract the id_type attribute
    let id_type = input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("id_type"))
        .and_then(|attr| attr.parse_meta().ok())
        .and_then(|meta| match meta {
            Meta::List(meta_list) => Some(meta_list.nested),
            _ => None,
        })
        .and_then(|nested| {
            nested.iter().find_map(|nested| match nested {
                NestedMeta::Meta(Meta::Path(path)) => Some(path.clone()),
                _ => None,
            })
        })
        .expect("id_type attribute is required");

    let expanded = quote! {
        impl Identify for #name {
            type InnerId = #id_type;
        }
    };

    TokenStream::from(expanded)
}
