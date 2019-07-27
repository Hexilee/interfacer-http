use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, AttrStyle, Attribute, MetaList, TraitItemMethod};

use crate::args::*;

const METHODS: [&'static str; 9] = [
    "get", "post", "put", "delete", "head", "options", "connect", "patch", "trace",
];
const EXPECT: &'static str = "expect";

fn gen_meta(attr: Attribute) -> proc_macro::TokenStream {
    let name = attr.path.segments.last().unwrap().value().ident.clone();
    let tts = attr.tts.clone();
    quote!(#name#tts).into()
}

fn check_duplicate(
    method_name: &str,
    attr: &Option<proc_macro::TokenStream>,
) -> Result<(), Diagnostic> {
    match attr {
        None => Ok(()),
        Some(_) => Err(Diagnostic::new(
            Level::Error,
            format!("method `{}` has duplicate attribute", method_name,),
        )),
    }
}

fn filter_method(raw_method: &TraitItemMethod) -> Result<ArgsTokens, Diagnostic> {
    let method_name = raw_method.sig.ident.to_string();
    let mut tokens = ArgsTokens {
        req: None,
        expect: None,
    };
    for attr in raw_method.attrs.clone() {
        if let AttrStyle::Outer = attr.style {
            let name = attr.path.segments.last().unwrap().value().ident.to_string();
            if name.as_str() == EXPECT {
                check_duplicate(method_name.as_str(), &tokens.expect)?;
                tokens.expect = Some(gen_meta(attr))
            } else if METHODS.contains(&name.as_str()) {
                check_duplicate(method_name.as_str(), &tokens.req)?;
                tokens.req = Some(gen_meta(attr))
            }
        }
    }

    match tokens.req {
        Some(_) => Ok(tokens),
        None => Err(Diagnostic::new(
            Level::Error,
            format!("method `{}` has no request attribute", method_name,),
        )),
    }
}

fn gen_args(req_meta: MetaList, expect_meta: Option<MetaList>) -> Result<Args, Diagnostic> {
    let mut args = Args::default();
    args.req.load_meta(&req_meta)?;
    if let Some(ref meta) = expect_meta {
        args.expect.load_meta(meta)?;
    }
    Ok(args)
}

macro_rules! parse_args {
    ($args:ident, $raw_method:ident) => {
        let ArgsTokens { req, expect } = filter_method(&$raw_method).unwrap_or_else(|err| {
            err.emit();
            std::process::exit(1);
        });
        let req_token = req.unwrap();
        let req_meta = parse_macro_input!(req_token as MetaList);
        let expect_meta = match expect {
            Some(token) => Some(parse_macro_input!(token as MetaList)),
            None => None,
        };
        let $args = gen_args(req_meta, expect_meta).unwrap_or_else(|err| {
            err.emit();
            std::process::exit(1);
        });
    };
}

macro_rules! define_idents {
    ($($idents:ident),*) => {
        $(let $idents = quote!($idents);)*
    };
}

// TODO: remove use interfacer_http::polyfill when const generics is stable
// TODO: finish check_resp
pub fn transform_method(mut raw_method: TraitItemMethod) -> proc_macro::TokenStream {
    parse_args!(args, raw_method);
    define_idents!(req_ident, parts_ident, body_ident);
    let import = quote!(
        use interfacer_http::{polyfill::*, RequestFail, ContentType, http::StatusCode};
    );
    let req_define = build_request(&req_ident, &args, &raw_method);
    let send_request = quote!(
        let (#parts_ident, #body_ident) = self.get_client().request(#req_ident).await?.into_parts();
    );
    let check_resp = quote!();
    let body = quote!(
        #import
        #req_define
        #send_request
        #check_resp

    );
    raw_method.semi_token = None;
    raw_method.default = Some(parse_quote!({
        #body
    }));
    quote!(#raw_method).into()
}

// TODO: complete build request; using generic Body type
fn build_request(
    req_ident: &TokenStream,
    args: &Args,
    _raw_method: &TraitItemMethod,
) -> TokenStream {
    let path = args.req.path.as_str();
    let method = args.req.method.as_str();
    quote!(
        let mut builder = interfacer_http::http::Request::builder();
        let #req_ident = builder
            .uri(#path)
            .method(#method)
            .body(vec![])?;
    )
}
