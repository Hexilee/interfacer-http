#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use interfacer::http::{HttpTryFrom, Method};
use proc_macro::{Diagnostic, Level, TokenStream};
use syn::{parse_macro_input, AttributeArgs, ItemTrait, TraitItemMethod};
use darling::FromMeta;

#[proc_macro_attribute]
pub fn http_service(_args: TokenStream, input: TokenStream) -> TokenStream {
    http_service_impl::implement(parse_macro_input!(input as ItemTrait)).into()
}

fn request(method: &str, raw_args: TokenStream, input: TokenStream) -> TokenStream {
    let method = Method::try_from(method).unwrap_or_else(|err| {
        Diagnostic::new(
            Level::Error,
            format!("{}, fallback to GET", err.to_string()),
        )
            .emit();
        Method::GET
    });

    let args = http_request_impl::Args::from_list(&parse_macro_input!(raw_args as AttributeArgs))
        .unwrap_or_else(|err| {
            Diagnostic::new(
                Level::Error,
                format!("parse service method fails: {}", err.to_string()),
            )
                .emit();
            Default::default()
        });

    http_request_impl::request(
        method.as_str(),
        args,
        parse_macro_input!(input as TraitItemMethod),
    )
        .into()
}

macro_rules! define_request {
    ($($method:ident),*) => {
        $(
            #[proc_macro_attribute]
            pub fn $method(args: TokenStream, input: TokenStream) -> TokenStream {
                request(stringify!($method), args, input)
            }
        )*
    };
}

define_request!(get, post, put, delete, head, options, connect, patch, trace);

mod http_request_impl;
mod http_service_impl;
