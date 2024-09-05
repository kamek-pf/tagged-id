extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta, NestedMeta, Path};

#[proc_macro_derive(Identify, attributes(id_type))]
pub fn derive_identify(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let name = &input.ident;

    // Extract the id_type attribute
    let mut id_type = None;
    for attr in input.attrs {
        if attr.path.is_ident("id_type") {
            if let Ok(meta) = attr.parse_meta() {
                if let Meta::List(meta_list) = meta {
                    for nested_meta in meta_list.nested {
                        if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
                            id_type = Some(path);
                        }
                    }
                }
            }
        }
    }

    let id_type = id_type.expect("id_type attribute is required");

    // Generate the implementation
    let expanded = quote! {
        impl Identify for #name {
            type InnerId = #id_type;
        }
    };

    TokenStream::from(expanded)
}
