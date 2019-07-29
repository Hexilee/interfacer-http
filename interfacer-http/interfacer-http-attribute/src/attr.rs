use crate::parse::try_parse;
use interfacer_http_util::{content_types, http::StatusCode};
use proc_macro::{Diagnostic, Level, TokenStream};
use quote::quote;
use syn::{AttrStyle, Attribute, Lit, Meta, MetaList, MetaNameValue, NestedMeta, TraitItemMethod};

const PATH: &'static str = "path";
const CONTENT_TYPE: &'static str = "content_type";
const CHARSET: &'static str = "charset";
const STATUS: &'static str = "status";

const METHODS: [&'static str; 9] = [
    "get", "post", "put", "delete", "head", "options", "connect", "patch", "trace",
];
const EXPECT: &'static str = "expect";

pub trait LoadMeta {
    fn load_meta(&mut self, meta: &MetaList) -> Result<(), Diagnostic>;
}

#[derive(Debug)]
pub struct ContentType {
    pub base_type: String,
    pub encoding: Option<String>,
}

#[derive(Debug)]
pub struct Expect {
    pub status: StatusCode,
    pub content_type: ContentType,
}

#[derive(Debug)]
pub struct ReqAttr {
    pub method: String,
    pub path: String,
    pub content_type: ContentType,
}

pub struct AttrTokens {
    pub req: proc_macro::TokenStream,
    pub expect: Option<proc_macro::TokenStream>,
}

#[derive(Default)]
pub struct Attr {
    pub req: ReqAttr,
    pub expect: Expect,
}

impl Default for ContentType {
    fn default() -> Self {
        Self {
            base_type: content_types::APPLICATION_JSON.into(),
            encoding: None,
        }
    }
}

impl LoadMeta for ContentType {
    fn load_meta(&mut self, meta: &MetaList) -> Result<(), Diagnostic> {
        for nested_meta in meta.nested.iter() {
            if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                ident,
                eq_token: _,
                lit: Lit::Str(token),
            })) = nested_meta
            {
                match ident.to_string().as_str() {
                    CONTENT_TYPE => {
                        self.base_type = token.value();
                    }
                    CHARSET => {
                        self.encoding = Some(token.value());
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
}

impl Default for Expect {
    fn default() -> Self {
        Self {
            status: StatusCode::OK,
            content_type: Default::default(),
        }
    }
}

impl LoadMeta for Expect {
    fn load_meta(&mut self, meta: &MetaList) -> Result<(), Diagnostic> {
        for nested_meta in meta.nested.iter() {
            if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                ident,
                eq_token: _,
                lit: Lit::Int(token),
            })) = nested_meta
            {
                match ident.to_string().as_str() {
                    STATUS => {
                        let status = token.value() as u16;
                        match StatusCode::from_u16(status) {
                            Err(err) => Err(Diagnostic::new(
                                Level::Error,
                                format!("invalid status code: {}", err.to_string()),
                            ))?,
                            Ok(code) => {
                                self.status = code;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        self.content_type.load_meta(meta)
    }
}

impl Default for ReqAttr {
    fn default() -> Self {
        Self {
            method: Default::default(),
            path: "/".into(),
            content_type: Default::default(),
        }
    }
}

impl LoadMeta for ReqAttr {
    fn load_meta(&mut self, meta: &MetaList) -> Result<(), Diagnostic> {
        self.method = meta.ident.to_string();
        for nested_meta in meta.nested.iter() {
            if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                ident,
                eq_token: _,
                lit: Lit::Str(token),
            })) = nested_meta
            {
                match ident.to_string().as_str() {
                    PATH => {
                        self.path = token.value();
                    }
                    _ => (),
                }
            }
        }
        self.content_type.load_meta(meta)
    }
}

impl Attr {
    pub fn from_raw(raw_method: &TraitItemMethod) -> Result<Attr, Diagnostic> {
        let mut attr = Attr::default();
        let AttrTokens { req, expect } = filter_method(raw_method)?;
        attr.req.load_meta(&try_parse::<MetaList>(req)?)?;
        if let Some(token) = expect {
            attr.expect.load_meta(&try_parse::<MetaList>(token)?)?;
        }
        Ok(attr)
    }
}

fn gen_meta(attr: Attribute) -> TokenStream {
    let name = attr.path.segments.last().unwrap().value().ident.clone();
    let tts = attr.tts.clone();
    quote!(#name#tts).into()
}

fn check_duplicate(method_name: &str, attr: &Option<TokenStream>) -> Result<(), Diagnostic> {
    match attr {
        None => Ok(()),
        Some(_) => Err(Diagnostic::new(
            Level::Error,
            format!("method `{}` has duplicate attribute", method_name,),
        )),
    }
}

fn filter_method(raw_method: &TraitItemMethod) -> Result<AttrTokens, Diagnostic> {
    let method_name = raw_method.sig.ident.to_string();
    let mut req = None;
    let mut expect = None;
    for attr in raw_method.attrs.clone() {
        if let AttrStyle::Outer = attr.style {
            let name = attr.path.segments.last().unwrap().value().ident.to_string();
            if name.as_str() == EXPECT {
                check_duplicate(method_name.as_str(), &expect)?;
                expect = Some(gen_meta(attr))
            } else if METHODS.contains(&name.as_str()) {
                check_duplicate(method_name.as_str(), &req)?;
                req = Some(gen_meta(attr))
            }
        }
    }

    match req {
        Some(req) => Ok(AttrTokens { req, expect }),
        None => Err(Diagnostic::new(
            Level::Error,
            format!("method `{}` has no request attribute", method_name,),
        )),
    }
}
