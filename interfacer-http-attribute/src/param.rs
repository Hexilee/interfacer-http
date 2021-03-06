use crate::parse::AttrMeta;
use proc_macro::{Diagnostic, Level};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use syn::{punctuated::Punctuated, FnArg, Lit, Meta, NestedMeta, Pat, Token};

const HEADER: &str = "header";
const BODY: &str = "body";

#[derive(Debug)]
pub struct Parameters {
    pub values: HashSet<Ident>,
    pub headers: Vec<(TokenStream, Ident)>,
    pub body: Option<Ident>,
}

#[derive(Debug)]
enum Parameter {
    Header(TokenStream),
    Body,
}

impl Parameter {
    fn header(nested: Punctuated<NestedMeta, Token![,]>) -> Result<TokenStream, Diagnostic> {
        match nested.first() {
            Some(NestedMeta::Meta(Meta::Path(path))) => Ok(quote!(#path)),
            Some(NestedMeta::Lit(Lit::Str(lit))) => Ok(quote!(#lit)),
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
        Some(name) => Err(Diagnostic::new(
            Level::Error,
            format!("duplicate body: {} against {}", param_name, name),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::try_parse;
    use matches::matches;
    use proc_macro2::Span;
    use std::iter::FromIterator;
    use syn::{ItemFn, LitStr};

    #[test]
    fn test_check_duplicate() -> Result<(), Diagnostic> {
        assert_eq!(
            (),
            check_duplicate(&Ident::new("foo", Span::call_site()), &None)?
        );
        let err = check_duplicate(
            &Ident::new("foo", Span::call_site()),
            &Some(Ident::new("bar", Span::call_site())),
        )
        .unwrap_err();
        assert_eq!("duplicate body: foo against bar", err.message());
        Ok(())
    }

    #[test]
    fn param_header() -> Result<(), Diagnostic> {
        assert_eq!(
            quote!("Content-Type").to_string(),
            Parameter::header(Punctuated::from_iter(
                vec![NestedMeta::Lit(Lit::Str(LitStr::new(
                    "Content-Type",
                    Span::call_site(),
                )))]
                .into_iter(),
            ))?
            .to_string()
        );
        assert_eq!(
            quote!(CONTENT_TYPE).to_string(),
            Parameter::header(Punctuated::from_iter(
                vec![NestedMeta::Meta(Meta::Path(
                    Ident::new("CONTENT_TYPE", Span::call_site()).into()
                ))]
                .into_iter(),
            ))?
            .to_string()
        );
        assert_eq!(
            "header parameter name should be path or str literal",
            Parameter::header(Punctuated::new()).unwrap_err().message()
        );
        Ok(())
    }

    #[test]
    fn param_try_from() -> Result<(), Diagnostic> {
        assert!(matches!(
            AttrMeta::Name(Ident::new(BODY, Span::call_site())).try_into()?,
            Parameter::Body
        ));
        assert!(matches!(
            AttrMeta::List {
                name: Ident::new(BODY, Span::call_site()),
                nested: Punctuated::new()
            }
            .try_into()?,
            Parameter::Body
        ));
        assert!(matches!(
            AttrMeta::List {
                name: Ident::new(HEADER, Span::call_site()),
                nested: Punctuated::from_iter(
                    vec![NestedMeta::Lit(Lit::Str(LitStr::new(
                        "",
                        Span::call_site()
                    )))]
                    .into_iter()
                )
            }
            .try_into()?,
            Parameter::Header(_)
        ));
        assert_eq!(
            "header parameter name should be path or str literal",
            Parameter::try_from(AttrMeta::List {
                name: Ident::new(HEADER, Span::call_site()),
                nested: Punctuated::new(),
            })
            .unwrap_err()
            .message()
        );
        Ok(())
    }

    fn parse_params(params_token: TokenStream) -> Result<Parameters, Diagnostic> {
        let func: ItemFn = try_parse(quote!(fn foo(#params_token){}))?;
        func.sig.inputs.try_into()
    }

    #[test]
    fn params_try_from_empty() -> Result<(), Diagnostic> {
        let params = parse_params(quote!())?;
        assert!(params.values.is_empty());
        assert!(params.headers.is_empty());
        assert!(params.body.is_none());
        Ok(())
    }

    #[test]
    fn params_try_from_basic() -> Result<(), Diagnostic> {
        let params = parse_params(quote!(name: &str))?;
        assert!(params
            .values
            .contains(&Ident::new("name", Span::call_site())));
        assert!(params.headers.is_empty());
        assert!(params.body.is_none());
        Ok(())
    }

    #[test]
    fn params_try_from_full() -> Result<(), Diagnostic> {
        let params = parse_params(quote!(
            name: &str,
            #[body]
            body: String,
            #[header(COOKIE)]
            cookie: &str
        ))?;
        assert!(params
            .values
            .contains(&Ident::new("name", Span::call_site())));
        assert_eq!(quote!(COOKIE).to_string(), params.headers[0].0.to_string());
        assert_eq!(
            &Ident::new("cookie", Span::call_site()),
            &params.headers[0].1
        );
        assert_eq!(Some(Ident::new("body", Span::call_site())), params.body);
        Ok(())
    }
}
