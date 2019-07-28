use crate::args::{Args, ArgsTokens, LoadMeta};
use proc_macro::{Diagnostic, Level, TokenStream};
use quote::quote;
use syn::parse::{Parse, Parser};
use syn::{AttrStyle, Attribute, MetaList, TraitItemMethod};

const METHODS: [&'static str; 9] = [
    "get", "post", "put", "delete", "head", "options", "connect", "patch", "trace",
];
const EXPECT: &'static str = "expect";

pub fn parse_args(raw_method: &TraitItemMethod) -> Result<Args, Diagnostic> {
    let mut args = Args::default();
    let ArgsTokens { req, expect } = filter_method(raw_method)?;
    args.req.load_meta(&try_parse::<MetaList>(req)?)?;
    if let Some(token) = expect {
        args.expect.load_meta(&try_parse::<MetaList>(token)?)?;
    }
    Ok(args)
}

fn try_parse<T: Parse>(token: TokenStream) -> Result<T, Diagnostic> {
    let copy = token.clone();
    <T as Parse>::parse.parse(token).map_err(|err| {
        Diagnostic::new(
            Level::Error,
            format!("parse token({}) fail: {}", copy.to_string(), err),
        )
    })
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

fn filter_method(raw_method: &TraitItemMethod) -> Result<ArgsTokens, Diagnostic> {
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
        Some(req) => Ok(ArgsTokens { req, expect }),
        None => Err(Diagnostic::new(
            Level::Error,
            format!("method `{}` has no request attribute", method_name,),
        )),
    }
}
