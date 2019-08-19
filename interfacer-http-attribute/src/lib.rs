#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemTrait};

#[proc_macro_attribute]
pub fn http_service(_args: TokenStream, input: TokenStream) -> TokenStream {
    interface_impl::implement(parse_macro_input!(input as ItemTrait)).into()
}

// TODO: remove when const generics is stable
#[proc_macro_derive(ToContent)]
pub fn derive_to_content(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote!(
        impl #impl_generics interfacer_http::ToContent for #name #ty_generics #where_clause {
            type Err = interfacer_http::FromContentError;
            #[inline]
            fn to_content(&self, content_type: &interfacer_http::mime::Mime) -> core::result::Result<Vec<u8>, Self::Err> {
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
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote!(
        impl #impl_generics interfacer_http::FromContent for #name #ty_generics #where_clause {
            type Err = interfacer_http::FromContentError;
            #[inline]
            fn from_content(data: Vec<u8>, content_type: &interfacer_http::mime::Mime) -> core::result::Result<Self, Self::Err> {
                use interfacer_http::polyfill::*;
                Self::_from_content(data, content_type)
            }
        }
    ).into()
}

mod attr;
mod interface_impl;
mod method;
mod param;
mod parse;
