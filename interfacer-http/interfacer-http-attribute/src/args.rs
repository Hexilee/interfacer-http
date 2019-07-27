use interfacer_http_util::{content_types, http::StatusCode};
use proc_macro::{Diagnostic, Level};
use syn::{Lit, Meta, MetaList, MetaNameValue, NestedMeta};

const PATH: &'static str = "path";
const CONTENT_TYPE: &'static str = "content_type";
const CHARSET: &'static str = "charset";
const STATUS: &'static str = "status";

pub trait LoadMeta {
    fn load_meta(&mut self, meta: &MetaList) -> Result<(), Diagnostic>;
}

#[derive(Debug)]
pub struct ContentType {
    pub content_type: String,
    pub charset: Option<String>,
}

#[derive(Debug)]
pub struct Expect {
    pub status: StatusCode,
    pub content_type: ContentType,
}

#[derive(Debug)]
pub struct ReqArgs {
    pub path: String,
    pub content_type: ContentType,
}

pub struct ArgsTokens {
    pub req: Option<proc_macro::TokenStream>,
    pub expect: Option<proc_macro::TokenStream>,
}

#[derive(Default)]
pub struct Args {
    pub req: ReqArgs,
    pub expect: Expect,
}

impl Default for ContentType {
    fn default() -> Self {
        Self {
            content_type: content_types::APPLICATION_JSON.into(),
            charset: None,
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
                        self.content_type = token.value();
                    }
                    CHARSET => {
                        self.charset = Some(token.value());
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

impl Default for ReqArgs {
    fn default() -> Self {
        Self {
            path: "/".into(),
            content_type: Default::default(),
        }
    }
}

impl LoadMeta for ReqArgs {
    fn load_meta(&mut self, meta: &MetaList) -> Result<(), Diagnostic> {
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
