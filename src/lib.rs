extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn rs_route_ts(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;
    let ident = sig.ident.to_string();
    let output = match &sig.output {
        syn::ReturnType::Default => "undefined".to_owned(),
        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            syn::Type::Path(path) => path.path.segments.last().unwrap().ident.to_string(),
            _ => "unknown".to_owned(),
        },
    };

    println!("doing work {} {}\nother attrs {}", ident, output, attr);
    (quote! {#input}).into()
}
