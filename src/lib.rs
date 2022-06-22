use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_str, Type};

#[proc_macro]
pub fn impl_from(input: TokenStream) -> TokenStream {
    // vvv this is dirty but looks better than matching Punct
    let input_as_string: String = input.to_string();
    let input_as_vec_str: Vec<&str> = input_as_string.split(",").collect();
    let from_error = parse_str::<Type>(input_as_vec_str[0]).unwrap();
    let for_error_full = parse_str::<Type>(input_as_vec_str[1]).unwrap();
    let for_error_as_str = &input_as_vec_str[1][..(input_as_vec_str[1].rfind(":").unwrap() - 1)];
    let for_error = parse_str::<Type>(for_error_as_str).unwrap();

    TokenStream::from(quote! {
        impl From<#from_error> for #for_error {
            fn from(err: #from_error) -> Self {
                #for_error_full(err.to_string())
            }
        }
    })
}

#[proc_macro]
pub fn impl_error(input: TokenStream) -> TokenStream {
    let for_error = parse::<Type>(input).unwrap();
    TokenStream::from(quote! {
        impl std::fmt::Display for #for_error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", &self)
            }
        }

        impl std::error::Error for #for_error {}
    })
}
