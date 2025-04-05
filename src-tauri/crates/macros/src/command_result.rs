use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn wrapper(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    let func_name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let vis = &input.vis;
    let block = &input.block;
    let attrs = &input.attrs;
    
    let expanded = quote! {
        #(#attrs)*
        #vis fn #func_name(#inputs) #output {
            let __result: Result<_, ErrorCode> = (|| #block)();
            
            __result.map_err(|e| CE {
                code: e.code(),
                message: e.to_string(),
                func: stringify!(#func_name).to_string(),
                data: None,
            })
        }
    };
    
    expanded.into()
}