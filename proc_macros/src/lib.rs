use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RequestError)]
pub fn derive_request_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let expanded = quote! {
        impl std::error::Error for #ident {}
        impl crate::types::error::RequestError for #ident {}
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_derive(ResponseData)]
pub fn derive_response_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let expanded = quote! {
        impl crate::types::response::ResponseData for #ident {}
    };
    
    TokenStream::from(expanded)
}