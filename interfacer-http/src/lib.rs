#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
use crate::http_request::request;
use http::{HttpTryFrom, Method};
use proc_macro::{Diagnostic, Level, TokenStream};
use syn::{parse_macro_input, AttributeArgs, ItemTrait, TraitItemMethod};

#[proc_macro_attribute]
pub fn http_service(_args: TokenStream, input: TokenStream) -> TokenStream {
    http_service_impl::implement(parse_macro_input!(input as ItemTrait)).into()
}

#[proc_macro_attribute]
pub fn expect(args: TokenStream, input: TokenStream) -> TokenStream {
    expect_impl::implement(
        parse_macro_input!(args as AttributeArgs),
        parse_macro_input!(input as TraitItemMethod),
    )
    .into()
}

macro_rules! define_request {
    ($($method:ident),*) => {
        $(
            #[proc_macro_attribute]
            pub fn $method(args: TokenStream, input: TokenStream) -> TokenStream {
                let method = Method::try_from(stringify!($method)).unwrap_or_else(|err| {
                    Diagnostic::new(
                        Level::Error,
                        format!("{}, fallback to GET", err.to_string()),
                    )
                    .emit();
                    Method::GET
                });
                request(
                    method,
                    parse_macro_input!(args as AttributeArgs),
                    parse_macro_input!(input as TraitItemMethod),
                )
                .into()
            }
        )*
    };
}

define_request!(get, post, put, delete, head, options, connect, patch, trace);

mod expect_impl;
mod http_request;
mod http_service_impl;
