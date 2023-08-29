extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{ExprClosure, LitStr, parse_macro_input};
use syn::token::Comma;

#[proc_macro]
pub fn instruction(_input: TokenStream) -> TokenStream {

    let mut cloned_input = _input.clone().into_iter();

    if cloned_input.size_hint().0 < 3 {
        // PARSE ERROR
    }

    let first: TokenStream = TokenStream::from(cloned_input.nth(0).unwrap());
    let second: TokenStream = TokenStream::from(cloned_input.nth(1).unwrap());
    let closure_stream: TokenStream = cloned_input.enumerate()
        .filter(|index| index.0 > 2)
        .map(|x| x.1)
        .collect();

    let function_name: LitStr = parse_macro_input!(first as syn::LitStr);
    let function_comma: Comma = parse_macro_input!(second as syn::token::Comma);
    let function_closure: ExprClosure = parse_macro_input!(closure_stream as syn::ExprClosure);

    let tokens = quote! {

    };

    TokenStream::from(tokens)
}
