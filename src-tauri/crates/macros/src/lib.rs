extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;

mod command_result;

#[proc_macro_attribute]
pub fn command_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    command_result::wrapper(_attr, item)
}