use proc_macro::{Diagnostic, Level, TokenStream};
use proc_macro2::Ident;
use quote::quote;
use syn::parse::{Parse, Parser};
use syn::{punctuated::Punctuated, Attribute, MetaList, NestedMeta, Token};

#[derive(Clone)]
pub struct AttrMeta {
    pub name: Ident,
    pub nested: Punctuated<NestedMeta, Token![,]>,
}

pub fn try_parse<T: Parse>(token: TokenStream) -> Result<T, Diagnostic> {
    let copy = token.clone();
    <T as Parse>::parse.parse(token).map_err(|err| {
        Diagnostic::new(
            Level::Error,
            format!("parse token({}) fail: {}", copy.to_string(), err),
        )
    })
}

pub fn gen_meta_list(attr: &Attribute) -> Result<MetaList, Diagnostic> {
    let name = &attr.path;
    let tokens = &attr.tokens;
    try_parse::<MetaList>(quote!(#name#tokens).into())
}
