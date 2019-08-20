use proc_macro::{Diagnostic, Level};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, Parser};
use syn::{punctuated::Punctuated, Attribute, Meta, NestedMeta, Path, Token};

#[derive(Clone)]
pub enum AttrMeta {
    Name(Ident),
    List {
        name: Ident,
        nested: Punctuated<NestedMeta, Token![,]>,
    },
}

pub fn try_parse<T: Parse>(token: TokenStream) -> Result<T, Diagnostic> {
    let copy = token.clone();
    <T as Parse>::parse.parse2(token).map_err(|err| {
        Diagnostic::new(
            Level::Error,
            format!("parse token({}) fail: {}", copy.to_string(), err),
        )
    })
}

impl TryFrom<Attribute> for AttrMeta {
    type Error = Diagnostic;
    fn try_from(attr: Attribute) -> Result<Self, Self::Error> {
        let raw_meta = attr.parse_meta().map_err(|err| {
            Diagnostic::new(
                Level::Error,
                format!("attr ({}) is not a meta: {}", quote!(#attr), err),
            )
        })?;
        match raw_meta {
            Meta::Path(ref path) => Ok(AttrMeta::Name(Self::parse_path(path)?)),
            Meta::List(list) => Ok(AttrMeta::List {
                name: Self::parse_path(&list.path)?,
                nested: list.nested,
            }),
            _ => Err(Diagnostic::new(
                Level::Error,
                format!("attr ({}) is not a valid AttrMeta", quote!(#attr)),
            )),
        }
    }
}

impl AttrMeta {
    pub fn name(&self) -> &Ident {
        match self {
            AttrMeta::Name(name) => name,
            AttrMeta::List { name, nested: _ } => name,
        }
    }

    pub fn parse_path(path: &Path) -> Result<Ident, Diagnostic> {
        if path.segments.len() != 1 {
            Err(Diagnostic::new(
                Level::Error,
                format!("path({}) is not a ident", quote!(#path)),
            ))
        } else {
            Ok(path.segments.first().unwrap().ident.clone())
        }
    }
}
