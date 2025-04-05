use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn cmd_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as ItemFn);
    
  let func_name = &input.sig.ident;
  let inputs = &input.sig.inputs;
  let output = &input.sig.output;
  let block = &input.block;
  
  // 获取原始返回类型
  let return_type = match output {
      ReturnType::Type(_, ty) => quote! { #ty },
      _ => quote! { () },
  };

  let expanded = quote! {
      #input
      {
          let __result: Result<#return_type, ErrorCode> = #block;
          
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