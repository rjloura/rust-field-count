extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, ItemStruct};

#[proc_macro_derive(FieldCount)]
pub fn derive_field_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let field_count = input.fields.iter().count();

    let name = &input.ident;

    let output = quote! {
        impl #name {
            pub fn field_count() -> usize {
                #field_count
            }
        }
    };

    TokenStream::from(output)
}
