extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemTrait};

#[proc_macro_attribute]
pub fn http_service(_args: TokenStream, input: TokenStream) -> TokenStream {
    http_service_impl::implement(parse_macro_input!(input as ItemTrait)).into()
}

mod http_service_impl;
