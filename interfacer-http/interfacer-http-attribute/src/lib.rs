#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use interfacer_http_service::{HttpTryFrom, Method};
use proc_macro::{Level, TokenStream};
use syn::{parse_macro_input, AttributeArgs, ItemTrait, TraitItemMethod};

#[proc_macro_attribute]
pub fn http_service(_args: TokenStream, input: TokenStream) -> TokenStream {
    http_service_impl::implement(parse_macro_input!(input as ItemTrait)).into()
}

mod http_service_impl;
mod method;
