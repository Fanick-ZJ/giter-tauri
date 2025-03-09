extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod command_result;

#[proc_macro_attribute]
pub fn command_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: {:?}", parse_macro_input!(_attr as syn::AttributeArgs));
    println!("item: {:?}", parse_macro_input!(item as syn::Item));
    // command_result::wrapper(_attr, item)
    // item
    TokenStream::default()
}