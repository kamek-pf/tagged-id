use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Path};

/// Derive the Identify trait with a `#[tagged_id(T)]` attribute.
#[proc_macro_derive(Id, attributes(tagged_id))]
pub fn derive_identify(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let name = &input.ident;

    // Extract the tagged_id attribute
    let tagged_id = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("tagged_id"))
        .and_then(|attr| attr.parse_args::<Path>().ok())
        .expect("tagged_id attribute is required");

    let expanded = quote! {
        impl tagged_id::Identify for #name {
            type InnerId = #tagged_id;
        }
    };

    TokenStream::from(expanded)
}
