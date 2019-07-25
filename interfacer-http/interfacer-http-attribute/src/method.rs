use darling::FromMeta;
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, AttrStyle, Attribute, AttributeArgs, ReturnType, Token,
    TraitItemMethod, Type,
};

const METHODS: [&'static str; 9] = [
    "get", "post", "put", "delete", "head", "options", "connect", "patch", "trace",
];

#[derive(Debug, FromMeta)]
struct Expect {
    status: i32,
    #[darling(default)]
    content_type: Option<String>,
}

#[derive(Debug, FromMeta, Default)]
pub struct Args {
    #[darling(default)]
    path: Option<String>,

    #[darling(default)]
    content_type: Option<String>,

    #[darling(default)]
    expect: Option<Expect>,
}

// TODO: exit when error
fn filter_method(raw_method: &TraitItemMethod) -> (String, proc_macro::TokenStream) {
    let attrs = raw_method.attrs.as_slice();
    if attrs.len() != 1 {
        Diagnostic::new(
            Level::Error,
            format!(
                "method {} has multiple attribute",
                &raw_method.sig.ident.to_string()
            ),
        )
        .emit();
    };
    let attr = attrs.first().unwrap().to_owned();
    if let AttrStyle::Inner(_) = attr.style {
        Diagnostic::new(
            Level::Error,
            format!(
                "the attribute({}) of method {} should be Outer",
                stringify!(&attr),
                &raw_method.sig.ident.to_string()
            ),
        )
        .emit();
    };

    let length = attr.path.segments.len();
    let attr_name = if length > 0 {
        attr.path
            .segments
            .first()
            .unwrap()
            .value()
            .ident
            .to_string()
    } else {
        "".into()
    };

    if length != 1 || !METHODS.contains(&attr_name.as_str()) {
        Diagnostic::new(
            Level::Error,
            format!(
                "the attribute name of method {} should be in {:?}",
                &raw_method.sig.ident.to_string(),
                &METHODS
            ),
        )
        .emit();
    };
    (attr_name, proc_macro::TokenStream::from(attr.tts.clone()))
}

pub fn transform_method(mut raw_method: TraitItemMethod) -> proc_macro::TokenStream {
    let (http_method, raw_args) = filter_method(&raw_method);
    let args: Args = Args::from_list(&parse_macro_input!(raw_args as AttributeArgs))
        .unwrap_or_else(|err| {
            Diagnostic::new(
                Level::Error,
                format!("parse service method fails: {}", err.to_string()),
            )
            .emit();
            Default::default()
        });
    let req_ident = quote!(req);
    let req_define = build_request(&req_ident, http_method.as_str(), &args, &raw_method);
    let body = quote!(
        #req_define
    );
    raw_method.semi_token = None;
    raw_method.default = Some(parse_quote!({
        #body
    }));
    quote!(raw_method).into()
}

// TODO: complete build request; using generic Body type
fn build_request(
    req_ident: &TokenStream,
    method: &str,
    args: &Args,
    raw_method: &TraitItemMethod,
) -> TokenStream {
    let path = match args.path {
        Some(ref path) => path,
        None => "/",
    };
    quote!(
        let mut builder = interfacer::http::Request::builder();
        let #req_ident = builder
            .uri(#path)
            .method(#method)
            .body(vec![])?;
    )
}
