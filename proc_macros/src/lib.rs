use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ErrDisplay)]
pub fn derive_err_display(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.message)
            }
        }
        impl std::error::Error for #name {
            fn description(&self) -> &str {
                self.message.as_str()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(AnkiResponse)]
pub fn derive_anki_response(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let gen = quote! {
        impl response::AnkiResponse for #name {}
    };
    gen.into()
}