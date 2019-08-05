use crate::parse::AttrMeta;
use proc_macro::{Diagnostic, Level};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use syn::{punctuated::Punctuated, FnArg, Lit, Meta, NestedMeta, Pat, Token};

const VAL: &str = "val";
const HEADER: &str = "header";
const BODY: &str = "body";

pub struct Parameters {
    pub values: HashMap<String, Ident>,
    pub headers: Vec<(TokenStream, Ident)>,
    pub body: Option<Ident>,
}

enum Parameter {
    Value(Option<Ident>),
    Header(TokenStream),
    Body,
}

impl Parameter {
    fn value(nested: Punctuated<NestedMeta, Token![,]>) -> Result<Option<Ident>, Diagnostic> {
        match nested.first() {
            Some(NestedMeta::Meta(Meta::Path(path))) => {
                if path.segments.len() != 1 {
                    Err(Diagnostic::new(
                        Level::Error,
                        "invalid value parameter name",
                    ))
                } else {
                    Ok(Some(path.segments.first().unwrap().ident.clone()))
                }
            }
            None => Ok(None),
            _ => Err(Diagnostic::new(
                Level::Error,
                "invalid value parameter, rename should be ident",
            )),
        }
    }

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
            VAL => match meta {
                AttrMeta::Name(_) => Ok(Parameter::Value(None)),
                AttrMeta::List { name: _, nested } => Ok(Parameter::Value(Self::value(nested)?)),
            },
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
                format!("unsupported attribute name `{}`", meta.name()),
            )),
        }
    }
}

impl TryFrom<Punctuated<FnArg, Token![,]>> for Parameters {
    type Error = Diagnostic;
    fn try_from(args: Punctuated<FnArg, Token![,]>) -> Result<Self, Self::Error> {
        let mut values = HashMap::new();
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
                            // TODO: check duplicate rename
                            values.insert(name.ident.to_string(), name.ident.clone());
                        }
                        1 => match params.into_iter().nth(0).unwrap() {
                            Parameter::Value(ref rename) => {
                                // TODO: check duplicate rename
                                match rename {
                                    Some(rename) => {
                                        values.insert(rename.to_string(), name.ident.clone())
                                    }
                                    None => {
                                        // TODO: check duplicate rename
                                        values.insert(name.ident.to_string(), name.ident.clone())
                                    }
                                };
                            }

                            Parameter::Header(rename) => headers.push((rename, name.ident.clone())),
                            Parameter::Body => {
                                check_duplicate(&name.ident, &body)?;
                                body = Some(name.ident.clone());
                            }
                        },
                        _ => {
                            return Err(Diagnostic::new(
                                Level::Error,
                                "parameter can only be one of 'var', 'header' or 'body'",
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
