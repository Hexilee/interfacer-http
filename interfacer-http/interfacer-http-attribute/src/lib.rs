#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemTrait};

#[proc_macro_attribute]
pub fn http_service(_args: TokenStream, input: TokenStream) -> TokenStream {
    http_service_impl::implement(parse_macro_input!(input as ItemTrait)).into()
}

// TODO: remove when const generics is stable
#[proc_macro_derive(ToContent)]
pub fn derive_to_content(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    quote!(
        impl interfacer_http::ToContent for #name {
            type Err = interfacer_http::ToContentFail;
            #[inline]
            fn to_content(&self, content_type: &interfacer_http::ContentType) -> Result<Vec<u8>, Self::Err> {
                use interfacer_http::polyfill::*;
                self._to_content(content_type)
            }
        }
    ).into()
}

// TODO: remove when const generics is stable
#[proc_macro_derive(FromContent)]
pub fn derive_from_content(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    quote!(
        impl interfacer_http::FromContent for #name {
            type Err = interfacer_http::FromContentFail;
            #[inline]
            fn from_content(data: Vec<u8>, content_type: &interfacer_http::ContentType) -> Result<Self, Self::Err> {
                use interfacer_http::polyfill::*;
                Self::_from_content(data, content_type)
            }
        }
    ).into()
}

mod args;
mod http_service_impl;
mod method;
