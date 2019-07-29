use crate::parse::try_parse;
use interfacer_http_util::{content_types, http::StatusCode};
use proc_macro::{Diagnostic, Level, TokenStream};
use proc_macro2::{Ident, Literal, Span};
use quote::quote;
use std::convert::{TryFrom, TryInto};
use syn::{
    punctuated::Punctuated, AttrStyle, Attribute, Lit, LitStr, Meta, MetaList, MetaNameValue,
    NestedMeta, Path, Token, TraitItemMethod,
};

const METHODS: [&'static str; 9] = [
    "get", "post", "put", "delete", "head", "options", "connect", "patch", "trace",
];
const EXPECT: &'static str = "expect";

const DEFAULT_PATH: &'static str = "/";

#[derive(Clone)]
pub enum AttrExpr {
    Path(Path),
    Lit(Lit),
}

#[derive(Clone)]
pub struct AttrMeta {
    name: Ident,
    nested: Punctuated<NestedMeta, Token![,]>,
}

#[derive(Clone)]
pub struct Expect {
    pub status: AttrExpr,
    pub content_type: AttrExpr,
}

#[derive(Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub content_type: AttrExpr,
}

#[derive(Clone)]
pub struct AttrMetas {
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
        let metas = meta.nested.into_iter().collect::<Vec<NestedMeta>>();
        if metas.len() > 2 {
            Err(Diagnostic::new(
                Level::Error,
                "expect attribute has two args at most",
            ))?;
        }

        if metas.len() > 0 {
            match metas[0].clone() {
                NestedMeta::Literal(Lit::Int(lit)) => {
                    expect.status =
                        AttrExpr::Lit(Lit::new(Literal::u16_suffixed(lit.value() as u16)))
                }
                NestedMeta::Meta(Meta::Path(path)) => expect.status = AttrExpr::Path(path),
                _ => Err(Diagnostic::new(
                    Level::Error,
                    "status should be string literal",
                ))?,
            }
        }

        if metas.len() > 1 {
            load_content_type(&mut expect.content_type, metas[1].clone())?;
        }
        Ok(expect)
    }
}

impl Default for Expect {
    fn default() -> Self {
        let status = AttrExpr::Lit(Lit::new(Literal::u16_suffixed(StatusCode::OK.as_u16()))); // TODO: replace with path
        let content_type =
            AttrExpr::Lit(Lit::new(Literal::string(content_types::APPLICATION_JSON)));
        Self {
            status,
            content_type,
        }
    }
}

impl Default for Request {
    fn default() -> Self {
        let method = "get".to_owned();
        let path = DEFAULT_PATH.to_owned();
        let content_type =
            AttrExpr::Lit(Lit::new(Literal::string(content_types::APPLICATION_JSON)));
        Self {
            method,
            path,
            content_type,
        }
    }
}

impl TryFrom<AttrMeta> for Request {
    type Error = Diagnostic;
    fn try_from(meta: AttrMeta) -> Result<Self, Self::Error> {
        let mut request = Self::default();
        let method = meta.name.to_string();
        let metas = meta.nested.into_iter().collect::<Vec<NestedMeta>>();
        if metas.len() > 2 {
            Err(Diagnostic::new(
                Level::Error,
                "request attribute has two args at most",
            ))?;
        }

        if metas.len() > 0 {
            if let NestedMeta::Literal(Lit::Str(token)) = &metas[0] {
                request.path = token.value()
            } else {
                Err(Diagnostic::new(
                    Level::Error,
                    "path should be string literal",
                ))?
            }
        }

        if metas.len() > 1 {
            load_content_type(&mut request.content_type, metas[1].clone())?;
        }

        Ok(request)
    }
}

impl Attr {
    pub fn from_raw(raw_method: &TraitItemMethod) -> Result<Attr, Diagnostic> {
        let AttrMetas { req, expect } = filter_method(raw_method)?;
        let req = req.clone().try_into()?;
        let expect = match expect {
            Some(meta) => meta.try_into()?,
            None => Default::default(),
        };
        Ok(Attr { req, expect })
    }
}

fn load_content_type(content_type: &mut AttrExpr, meta: NestedMeta) -> Result<(), Diagnostic> {
    match meta {
        NestedMeta::Literal(Lit::Str(token)) => {
            Ok(*content_type = AttrExpr::Lit(Lit::new(Literal::string(token.value().as_str()))))
        }
        NestedMeta::Meta(Meta::Path(path)) => Ok(*content_type = AttrExpr::Path(path)),
        _ => Err(Diagnostic::new(
            Level::Error,
            "content_type should be string literal or path",
        )),
    }
}

fn gen_meta_tokens(attr: Attribute) -> TokenStream {
    let name = &attr.path;
    let tokens = &attr.tokens;
    quote!(#name#tokens).into()
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
    for attr in raw_method.attrs.clone() {
        match try_parse::<MetaList>(gen_meta_tokens(attr)) {
            Ok(meta) => {
                let name = meta.path.segments.last().unwrap().ident.clone();
                if &name.to_string() == EXPECT {
                    check_duplicate(method_name.as_str(), &expect)?;
                    expect = Some(AttrMeta {
                        name,
                        nested: meta.nested,
                    })
                } else if METHODS.contains(&name.to_string().as_str()) {
                    check_duplicate(method_name.as_str(), &req)?;
                    req = Some(AttrMeta {
                        name,
                        nested: meta.nested,
                    })
                }
            }
            _ => (),
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
