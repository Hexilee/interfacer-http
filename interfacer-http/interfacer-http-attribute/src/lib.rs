#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait};
use synstructure::decl_derive;

#[proc_macro_attribute]
pub fn http_service(_args: TokenStream, input: TokenStream) -> TokenStream {
    http_service_impl::implement(parse_macro_input!(input as ItemTrait)).into()
}

// TODO: remove when const generics is stable
decl_derive!([ToContent] => to_content_derive);
fn to_content_derive(input: synstructure::Structure) -> proc_macro2::TokenStream {
    let name = input.ast().ident.clone();
    quote!(
        impl interfacer_http::ToContent for #name {
            type Err = interfacer_http::ToContentFail;
            #[inline]
            fn to_content(&self, content_type: &interfacer_http::ContentType) -> Result<Vec<u8>, Self::Err> {
                use interfacer_http::polyfill::*;
                self._to_content(content_type)
            }
        }
    )
}

// TODO: remove when const generics is stable
decl_derive!([FromContent] => from_content_derive);
fn from_content_derive(input: synstructure::Structure) -> proc_macro2::TokenStream {
    let name = input.ast().ident.clone();
    quote!(
        impl interfacer_http::FromContent for #name {
            type Err = interfacer_http::FromContentFail;
            #[inline]
            fn from_content(data: Vec<u8>, content_type: &interfacer_http::ContentType) -> StdResult<Self, Self::Err> {
                use interfacer_http::polyfill::*;
                Self::_from_content(data, content_type)
            }
        }
    )
}

mod args;
mod http_service_impl;
mod method;
