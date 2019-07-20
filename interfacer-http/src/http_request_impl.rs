use interfacer::http::{Method, Request};
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::quote;
use syn::export::{Debug, ToTokens};
use syn::{AttributeArgs, Ident, Lit, LitStr, Meta, NestedMeta, TraitItemMethod};
use crate::expect::Expect;

pub struct Args {
    pub path: String,
    pub content_type: Option<Box<dyn ToTokens>>,
    pub expect: Option<Expect>,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            path: "".into(),
            content_type: None,
            expect: None,
        }
    }
}

impl Args {
    fn new(mut raw_args: AttributeArgs) -> Self {
        let mut args: Args = Default::default();
        args.try_set_path(raw_args.get(0));
        args.try_set_content_type(raw_args.get(1));
        args
    }

    pub fn try_set_path(&mut self, attr: Option<&NestedMeta>) {
        if let Some(path) = attr {
            if let NestedMeta::Literal(Lit::Str(lit_str)) = path {
                self.path = lit_str.value()
            } else {
                Diagnostic::new(Level::Error, "request path should be string literal").emit()
            }
        }
    }

    pub fn try_set_content_type(&mut self, attr: Option<&NestedMeta>) {
        if let Some(content_type) = attr {
            match content_type {
                NestedMeta::Literal(lit) => {
                    if let Lit::Str(_) = lit {
                        self.content_type = Some(Box::new(lit.clone()))
                    } else {
                        Diagnostic::new(Level::Error, "content type should be string literal")
                            .emit()
                    }
                }
                NestedMeta::Meta(meta) => self.content_type = Some(Box::new(meta.name())),
            }
        }
    }
}

pub fn request(method: &str, raw_args: AttributeArgs, raw_method: TraitItemMethod) -> TokenStream {
    let args = Args::new(raw_args);
    let raw_sig = &raw_method.sig;
    let attr = &raw_method.attrs;
    let req_ident = quote!(req);
    let req_define = build_request(&req_ident, method, &args, &raw_method);
    let return_type = quote!(<<Self as interfacer::http::HttpService>::Client as interfacer::http::HttpClient>::Response);
    let return_block = quote!(self.get_client().request(#req_ident));
    quote!(
        #($attr)*
        #raw_sig -> #return_type {
            #req_define
            #return_block
        }
    )
}

// TODO: complete build request; replace unwrap with try; using generic Body type
fn build_request(
    req_ident: &TokenStream,
    method: &str,
    args: &Args,
    raw_method: &TraitItemMethod,
) -> TokenStream {
    let path = &args.path;
    quote!(
        let mut builder = interfacer::http::Request::builder();
        let #req_ident = builder
            .uri(#path)
            .method(#method)
            .body(vec![])
            .unwrap();
    )
}
