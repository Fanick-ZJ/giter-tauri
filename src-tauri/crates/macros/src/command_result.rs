use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

pub fn wrapper(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let mut item = syn::parse_macro_input!(item as syn::Item);
    println!("{:?}", item);
    item
}