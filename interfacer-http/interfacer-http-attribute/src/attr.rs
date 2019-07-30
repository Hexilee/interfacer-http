use crate::parse::{gen_meta_list, try_parse, AttrMeta};
use interfacer_http_util::{content_types, http::StatusCode};
use proc_macro::{Diagnostic, Level};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use std::convert::{TryFrom, TryInto};
use syn::{
    punctuated::Punctuated, AttrStyle, Attribute, Lit, LitStr, Meta, MetaList, MetaNameValue,
    NestedMeta, Path, Token, TraitItemMethod,
};

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
            NestedMeta::Literal(Lit::Int(lit)) => StatusCode::from_u16(lit.value() as u16)
                .map(|code| {
                    let code = code.as_u16();
                    self.status = quote!(StatusCode::from_u16(#code).unwrap());
                })
                .map_err(|err| {
                    Diagnostic::new(Level::Error, format!("invalid status code: {}", err))
                }),
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
        let metas = meta.nested.into_iter().collect::<Vec<NestedMeta>>();
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
        Ok(expect)
    }
}

impl Default for Expect {
    fn default() -> Self {
        let code = StatusCode::OK.as_u16();
        let status = quote!(StatusCode::from_u16(#code).unwrap());
        let default_content_type = content_types::APPLICATION_JSON;
        let content_type = quote!(#default_content_type);
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
        let default_content_type = content_types::APPLICATION_JSON;
        let content_type = quote!(#default_content_type);
        Self {
            method,
            path,
            content_type,
        }
    }
}

impl Request {
    fn load_path(&mut self, meta: &NestedMeta) -> Result<(), Diagnostic> {
        if let NestedMeta::Literal(Lit::Str(token)) = meta {
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
        let mut request = Self::default();
        request.method = meta.name.to_string();
        let metas = meta.nested.into_iter().collect::<Vec<NestedMeta>>();
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

// TODO: check lit
fn load_content_type(content_type: &mut TokenStream, meta: &NestedMeta) -> Result<(), Diagnostic> {
    match meta {
        NestedMeta::Literal(Lit::Str(token)) => {
            *content_type = quote!(#token);
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
    for attr in raw_method.attrs.clone() {
        if let Ok(meta) = gen_meta_list(&attr) {
            let name = meta.path.segments.last().unwrap().ident.clone();
            if name == EXPECT {
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
    }

    match req {
        Some(req) => Ok(AttrMetas { req, expect }),
        None => Err(Diagnostic::new(
            Level::Error,
            format!("method `{}` has no request attribute", method_name,),
        )),
    }
}
