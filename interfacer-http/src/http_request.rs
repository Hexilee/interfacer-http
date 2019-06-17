use interfacer::http::{Method, Request};
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::quote;
use syn::export::{Debug, ToTokens};
use syn::{AttributeArgs, Ident, Lit, LitStr, Meta, NestedMeta, TraitItemMethod};

pub struct Args {
    pub path: String,
    pub content_type: Option<Box<dyn ToTokens>>,
    pub send: bool,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            path: "".into(),
            content_type: None,
            send: true,
        }
    }
}

impl Args {
    fn new(mut raw_args: AttributeArgs) -> Self {
        let mut args: Args = Default::default();
        args.try_set_path(raw_args.get(0));
        args.try_set_content_type(raw_args.get(1));
        args.try_set_send(raw_args.get(2));
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

    pub fn try_set_send(&mut self, attr: Option<&NestedMeta>) {
        if let Some(send) = attr {
            if let NestedMeta::Meta(Meta::NameValue(kv)) = send {
                if kv.ident == "send" {
                    if let Lit::Bool(send) = &kv.lit {
                        self.send = send.value;
                        return;
                    }
                }
            }
        }
        Diagnostic::new(Level::Error, "send should be 'send=<true | false>'").emit()
    }
}

pub fn request(method: Method, raw_args: AttributeArgs, signature: TraitItemMethod) -> TokenStream {
    let args = Args::new(raw_args);
    quote!(#signature)
}
