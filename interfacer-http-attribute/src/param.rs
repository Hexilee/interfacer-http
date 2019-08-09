use crate::parse::AttrMeta;
use proc_macro::{Diagnostic, Level};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use syn::{punctuated::Punctuated, FnArg, Lit, Meta, NestedMeta, Pat, Token};

const HEADER: &str = "header";
const BODY: &str = "body";

pub struct Parameters {
    pub values: HashSet<Ident>,
    pub headers: Vec<(TokenStream, Ident)>,
    pub body: Option<Ident>,
}

enum Parameter {
    Header(TokenStream),
    Body,
}

impl Parameter {
    fn header(nested: Punctuated<NestedMeta, Token![,]>) -> Result<TokenStream, Diagnostic> {
        match nested.first() {
            Some(NestedMeta::Meta(Meta::Path(path))) => Ok(quote!(#path)),
            Some(NestedMeta::Literal(Lit::Str(lit))) => Ok(quote!(#lit)),
            _ => Err(Diagnostic::new(
                Level::Error,
                "header parameter name should be path or str literal",
            )),
        }
    }
}

impl TryFrom<AttrMeta> for Parameter {
    type Error = Diagnostic;
    fn try_from(meta: AttrMeta) -> Result<Self, Self::Error> {
        match meta.name().to_string().as_str() {
            HEADER => match meta {
                AttrMeta::List { name: _, nested } => Ok(Parameter::Header(Self::header(nested)?)),
                _ => Err(Diagnostic::new(
                    Level::Error,
                    "header parameter attribute must be MetaList",
                )),
            },
            BODY => Ok(Parameter::Body),
            _ => Err(Diagnostic::new(
                Level::Error,
                format!("unsupported attribute `{}`", meta.name()),
            )),
        }
    }
}

impl TryFrom<Punctuated<FnArg, Token![,]>> for Parameters {
    type Error = Diagnostic;
    fn try_from(args: Punctuated<FnArg, Token![,]>) -> Result<Self, Self::Error> {
        let mut values = HashSet::new();
        let mut headers = Vec::new();
        let mut body = None;
        for arg in args.iter() {
            if let FnArg::Typed(pat) = arg {
                if let Pat::Ident(name) = pat.pat.as_ref() {
                    let params = pat
                        .attrs
                        .iter()
                        .map(|attr| {
                            let meta: AttrMeta = attr.clone().try_into()?;
                            meta.try_into()
                        })
                        .filter_map(|result: Result<Parameter, Diagnostic>| {
                            result.map_err(|err| err.emit()).ok()
                        })
                        .collect::<Vec<Parameter>>();
                    match params.len() {
                        0 => {
                            values.insert(name.ident.clone());
                        }
                        1 => match params.into_iter().nth(0).unwrap() {
                            Parameter::Header(rename) => headers.push((rename, name.ident.clone())),
                            Parameter::Body => {
                                check_duplicate(&name.ident, &body)?;
                                body = Some(name.ident.clone());
                            }
                        },
                        _ => {
                            return Err(Diagnostic::new(
                                Level::Error,
                                "parameter can only be one of 'value', 'header' or 'body'",
                            ));
                        }
                    }
                }
            }
        }
        Ok(Parameters {
            values,
            headers,
            body,
        })
    }
}

fn check_duplicate(param_name: &Ident, body: &Option<Ident>) -> Result<(), Diagnostic> {
    match body {
        None => Ok(()),
        Some(_) => Err(Diagnostic::new(
            Level::Error,
            format!("param_name `{}` has duplicate body", param_name),
        )),
    }
}