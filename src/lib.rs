extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemStruct};

struct MeasuresInput {
    value_type: Ident,
}

impl syn::parse::Parse for MeasuresInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value_type = input.parse::<Ident>()?;
        Ok(MeasuresInput { value_type })
    }
}

#[proc_macro_attribute]
pub fn measures(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let args = parse_macro_input!(attr as MeasuresInput);

    let value_type = args.value_type;

    let struct_name = input.clone().ident;

    // println!("item: \"{}\"", struct_name.to_string());

    TokenStream::from(quote! {
        #input

        impl #struct_name {
            const VALUE_TYPE: &'static str = stringify!(#value_type);
        }
    })
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
