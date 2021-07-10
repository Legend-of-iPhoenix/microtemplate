use proc_macro;
use proc_macro::*;

use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Substitutions)]
pub fn template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &input.data
    {
        let names = fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().unwrap());

        (quote! {
            impl microtemplate::Context for #struct_name<'_> {
                fn get_field(&self, field_name: &str) -> &str {
                    match field_name {
                        #( stringify!(#names) => &self.#names, )*
                        _ => "",
                    }
                }
            }
        })
        .into()
    } else {
        panic!("Expected struct with named fields.")
    }
}
