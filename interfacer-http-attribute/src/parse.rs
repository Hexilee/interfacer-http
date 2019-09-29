use proc_macro::{Diagnostic, Level};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, Parser};
use syn::{punctuated::Punctuated, Attribute, Meta, NestedMeta, Path, Token};

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use syn::ItemFn;

    #[test]
    fn try_parse_test() -> Result<(), Diagnostic> {
        let func: ItemFn = try_parse(quote!(
            fn foo() {}
        ))?;
        assert_eq!("foo", &func.sig.ident.to_string());
        assert!(try_parse::<ItemFn>(quote!(let a = 1;))
            .unwrap_err()
            .message()
            .contains(&format!("parse token({}) fail", quote!(let a = 1;))));
        Ok(())
    }

    fn parse_attribute(token: TokenStream) -> Result<Attribute, Diagnostic> {
        let func: ItemFn = try_parse(quote!(
            #token
            fn foo(){}
        ))?;
        func.attrs
            .into_iter()
            .next()
            .ok_or(Diagnostic::new(Level::Error, "no recognized attributes"))
    }

    #[test]
    fn attr_meta_try_from() -> Result<(), Diagnostic> {
        let meta: AttrMeta = parse_attribute(quote!(#[foo]))?.try_into()?;
        assert_eq!("foo", meta.name().to_string());
        assert!(AttrMeta::try_from(parse_attribute(quote!(#[foo{}]))?)
            .unwrap_err()
            .message()
            .contains(&format!("attr ({}) is not a meta", quote!(#[foo{}]))));
        assert_eq!(
            format!("path({}) is not a ident", quote!(foo::bar)),
            AttrMeta::try_from(parse_attribute(quote!(#[foo::bar]))?)
                .unwrap_err()
                .message()
        );
        assert_eq!(
            format!("attr ({}) is not a valid AttrMeta", quote!(#[foo = 1])),
            AttrMeta::try_from(parse_attribute(quote!(#[foo = 1]))?)
                .unwrap_err()
                .message()
        );

        Ok(())
    }
}
