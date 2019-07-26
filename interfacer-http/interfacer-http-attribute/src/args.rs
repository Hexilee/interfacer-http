use interfacer_http_service::content_type;
use interfacer_http_service::StatusCode;
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, AttrStyle, Attribute, Lit, Meta, MetaList, MetaNameValue,
    NestedMeta, TraitItemMethod,
};

pub trait LoadMeta {
    fn load_meta(&mut self, meta: &MetaList) -> Result<(), Diagnostic>;
}

#[derive(Debug)]
pub struct ContentType {
    pub content_type: String,
    pub charset: String,
}

impl Default for ContentType {
    fn default() -> Self {
        Self {
            content_type: content_type::APPLICATION_JSON.into(),
            charset: content_type::CHARSET_UTF8.into(),
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
                        self.charset = token.value();
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Expect {
    pub status: StatusCode,
    pub content_type: ContentType,
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

#[derive(Debug)]
pub struct ReqArgs {
    pub path: String,
    pub content_type: ContentType,
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

pub struct ArgsTokens {
    pub req: Option<proc_macro::TokenStream>,
    pub expect: Option<proc_macro::TokenStream>,
}

#[derive(Default)]
pub struct Args {
    pub req: ReqArgs,
    pub expect: Expect,
}