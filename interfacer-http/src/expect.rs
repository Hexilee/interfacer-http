use interfacer::http::{Method, Request};
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::quote;
use syn::export::{Debug, ToTokens};
use syn::{AttributeArgs, Ident, Lit, LitStr, Meta, NestedMeta, TraitItemMethod};

pub struct Expect {
    pub status_code: i32,
    pub content_type: Option<Box<dyn ToTokens>>,
}