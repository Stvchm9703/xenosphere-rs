// File: src/lib.rs


// use proc_macro2::{ Span};
use proc_macro::TokenStream;
// use quote::quote;
use syn::{parse_macro_input, DeriveInput};



#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    input
}
