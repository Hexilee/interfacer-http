use crate::parse::AttrMeta;
use http::StatusCode;
use mime::Mime;
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::quote;
use std::convert::{TryFrom, TryInto};
use syn::{Lit, Meta, NestedMeta, TraitItemMethod};

const METHODS: [&str; 9] = [
    "get", "post", "put", "delete", "head", "options", "connect", "patch", "trace",
];
const EXPECT: &str = "expect";

const DEFAULT_PATH: &str = "/";

#[derive(Clone)]
pub struct Expect {
    pub status: TokenStream,
    pub content_type: Option<TokenStream>,
}

#[derive(Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub content_type: Option<TokenStream>,
}

#[derive(Clone)]
struct AttrMetas {
    pub req: AttrMeta,
    pub expect: Option<AttrMeta>,
}

#[derive(Clone)]
pub struct Attr {
    pub req: Request,
    pub expect: Expect,
}

impl TryFrom<AttrMeta> for Expect {
    type Error = Diagnostic;
    fn try_from(meta: AttrMeta) -> Result<Self, Self::Error> {
        let mut expect = Self::default();
        if let AttrMeta::List { name: _, nested } = meta {
            let metas = nested.into_iter().collect::<Vec<NestedMeta>>();
            if metas.len() > 2 {
                return Err(Diagnostic::new(
                    Level::Error,
                    "expect attribute has two args at most",
                ));
            }
            if !metas.is_empty() {
                expect.status = parse_status(&metas[0])?;
            }
            if metas.len() > 1 {
                expect.content_type = Some(parse_content_type(&metas[1])?);
            }
        }
        Ok(expect)
    }
}

impl Default for Expect {
    fn default() -> Self {
        let code = StatusCode::OK.as_u16();
        let status = quote!(StatusCode::from_u16(#code).unwrap());
        Self {
            status,
            content_type: None,
        }
    }
}

impl Request {
    pub fn new(method: &str) -> Self {
        let method = method.to_uppercase();
        let path = DEFAULT_PATH.to_owned();
        Self {
            method,
            path,
            content_type: None,
        }
    }
}

impl TryFrom<AttrMeta> for Request {
    type Error = Diagnostic;
    fn try_from(meta: AttrMeta) -> Result<Self, Self::Error> {
        let mut request = Self::new(&meta.name().to_string());
        if let AttrMeta::List { name: _, nested } = meta {
            let metas = nested.into_iter().collect::<Vec<NestedMeta>>();
            if metas.len() > 2 {
                return Err(Diagnostic::new(
                    Level::Error,
                    "request attribute has two args at most",
                ));
            }

            if !metas.is_empty() {
                request.path = parse_path(&metas[0])?;
            }

            if metas.len() > 1 {
                request.content_type = Some(parse_content_type(&metas[1])?);
            }
        }
        Ok(request)
    }
}

impl Attr {
    pub fn from_raw(raw_method: &TraitItemMethod) -> Result<Attr, Diagnostic> {
        let AttrMetas { req, mut expect } = filter_method(raw_method)?;
        let expect = match expect.take() {
            Some(meta) => meta.try_into()?,
            None => Default::default(),
        };
        let req = req.try_into()?;
        Ok(Attr { req, expect })
    }
}

fn parse_path(meta: &NestedMeta) -> Result<String, Diagnostic> {
    if let NestedMeta::Lit(Lit::Str(token)) = meta {
        Ok(token.value())
    } else {
        Err(Diagnostic::new(
            Level::Error,
            "path should be string literal",
        ))
    }
}

fn parse_content_type(meta: &NestedMeta) -> Result<TokenStream, Diagnostic> {
    match meta {
        NestedMeta::Lit(Lit::Str(token)) => {
            let value = token.value();
            let _: Mime = value.parse().map_err(|err| {
                Diagnostic::new(
                    Level::Error,
                    format!("invalid content-type('{}'): {}", &value, err),
                )
            })?;
            Ok(quote!(#value.parse().unwrap()))
        }
        NestedMeta::Meta(Meta::Path(path)) => Ok(quote!(#path)),
        _ => Err(Diagnostic::new(
            Level::Error,
            "content_type should be string literal or path",
        )),
    }
}

fn parse_status(meta: &NestedMeta) -> Result<TokenStream, Diagnostic> {
    match meta {
        NestedMeta::Lit(Lit::Int(lit)) => {
            StatusCode::from_u16(lit.base10_digits().parse().unwrap())
                .map(|code| {
                    let code = code.as_u16();
                    quote!(StatusCode::from_u16(#code).unwrap())
                })
                .map_err(|err| {
                    Diagnostic::new(Level::Error, format!("invalid status code: {}", err))
                })
        }
        NestedMeta::Meta(Meta::Path(path)) => Ok(quote!(#path)),
        _ => Err(Diagnostic::new(
            Level::Error,
            "status should be string literal",
        )),
    }
}

fn check_duplicate(method_name: &str, attr: &Option<AttrMeta>) -> Result<(), Diagnostic> {
    match attr {
        None => Ok(()),
        Some(_) => Err(Diagnostic::new(
            Level::Error,
            format!("method `{}` has duplicate attribute", method_name,),
        )),
    }
}

fn filter_method(raw_method: &TraitItemMethod) -> Result<AttrMetas, Diagnostic> {
    let method_name = raw_method.sig.ident.to_string();
    let mut req = None;
    let mut expect = None;
    for attr in raw_method.attrs.iter() {
        if let Ok(meta) = AttrMeta::try_from((*attr).clone()) {
            if meta.name() == EXPECT {
                check_duplicate(method_name.as_str(), &expect)?;
                expect = Some(meta)
            } else if METHODS.contains(&meta.name().to_string().as_str()) {
                check_duplicate(method_name.as_str(), &req)?;
                req = Some(meta)
            }
        }
    }

    match req {
        Some(req) => Ok(AttrMetas { req, expect }),
        None => Err(Diagnostic::new(
            Level::Error,
            format!("method `{}` has no request attribute", method_name,),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{Attr, DEFAULT_PATH};
    use quote::quote;
    use syn::parse_quote;

    #[test]
    #[should_panic]
    fn no_required_attr() {
        let _ = Attr::from_raw(&parse_quote!(
            #[test]
            #[xxxx]
            fn a(&self);
        ))
        .unwrap();
    }

    #[test]
    fn minimal_attr() {
        let Attr { req, expect } = Attr::from_raw(&parse_quote!(
            #[get]
            fn a(&self);
        ))
        .unwrap();
        assert_eq!("GET", &req.method);
        assert_eq!(DEFAULT_PATH, &req.path);
        assert_eq!(
            quote!(StatusCode::from_u16(200u16).unwrap()).to_string(),
            expect.status.to_string()
        );
        assert!(req.content_type.is_none());
        assert!(expect.content_type.is_none());
    }
}
