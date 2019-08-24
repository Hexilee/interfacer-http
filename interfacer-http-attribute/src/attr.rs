use crate::parse::AttrMeta;
use interfacer_http_util::http::StatusCode;
use interfacer_http_util::mime::Mime;
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
    pub content_type: TokenStream,
}

#[derive(Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub content_type: TokenStream,
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

impl Expect {
    fn load_status(&mut self, meta: &NestedMeta) -> Result<(), Diagnostic> {
        match meta {
            NestedMeta::Lit(Lit::Int(lit)) => {
                StatusCode::from_u16(lit.base10_digits().parse().unwrap())
                    .map(|code| {
                        let code = code.as_u16();
                        self.status = quote!(StatusCode::from_u16(#code).unwrap());
                    })
                    .map_err(|err| {
                        Diagnostic::new(Level::Error, format!("invalid status code: {}", err))
                    })
            }
            NestedMeta::Meta(Meta::Path(path)) => {
                self.status = quote!(#path);
                Ok(())
            }
            _ => Err(Diagnostic::new(
                Level::Error,
                "status should be string literal",
            )),
        }
    }
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
                expect.load_status(&metas[0])?;
            }
            if metas.len() > 1 {
                load_content_type(&mut expect.content_type, &metas[1])?;
            }
        }
        Ok(expect)
    }
}

impl Default for Expect {
    fn default() -> Self {
        let code = StatusCode::OK.as_u16();
        let status = quote!(StatusCode::from_u16(#code).unwrap());
        let content_type = quote!(interfacer_http::mime::APPLICATION_JSON);
        Self {
            status,
            content_type,
        }
    }
}

impl Request {
    pub fn new(method: &str) -> Self {
        let method = method.to_uppercase();
        let path = DEFAULT_PATH.to_owned();
        let content_type = quote!(interfacer_http::mime::APPLICATION_JSON);
        Self {
            method,
            path,
            content_type,
        }
    }
}

impl Request {
    fn load_path(&mut self, meta: &NestedMeta) -> Result<(), Diagnostic> {
        if let NestedMeta::Lit(Lit::Str(token)) = meta {
            self.path = token.value();
            Ok(())
        } else {
            Err(Diagnostic::new(
                Level::Error,
                "path should be string literal",
            ))
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
                request.load_path(&metas[0])?;
            }

            if metas.len() > 1 {
                load_content_type(&mut request.content_type, &metas[1])?;
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

fn load_content_type(content_type: &mut TokenStream, meta: &NestedMeta) -> Result<(), Diagnostic> {
    match meta {
        NestedMeta::Lit(Lit::Str(token)) => {
            let value = token.value();
            let _: Mime = value.parse().map_err(|err| {
                Diagnostic::new(
                    Level::Error,
                    format!("invalid content-type('{}'): {}", &value, err),
                )
            })?;
            *content_type = quote!(#value.parse().unwrap());
            Ok(())
        }
        NestedMeta::Meta(Meta::Path(path)) => {
            *content_type = quote!(#path);
            Ok(())
        }
        _ => Err(Diagnostic::new(
            Level::Error,
            "content_type should be string literal or path",
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
