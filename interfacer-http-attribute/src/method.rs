use proc_macro2::TokenStream;
use quote::quote;
use syn::{Block, TraitItemMethod};

use crate::attr::{Attr, Expect};
use crate::param::Parameters;
use crate::parse::try_parse;
use format_uri::gen_uri_format_expr;
use proc_macro::Diagnostic;
use std::convert::TryInto;

struct Context {
    attr: Attr,
    params: Parameters,
}

impl Context {
    fn parse(raw_method: &TraitItemMethod) -> Result<Self, Diagnostic> {
        let attr = Attr::from_raw(raw_method)?;
        let params = raw_method.clone().sig.inputs.try_into()?;
        Ok(Self { attr, params })
    }
}

pub fn gen_block(method: &TraitItemMethod) -> Result<Block, Diagnostic> {
    let context = Context::parse(method)?;
    let import_stmt = import();
    let send_request_stmt = send_request(build_request(&context)?);
    let check_response_stmt = check_response(&context.attr.expect);
    let return_stmt = return_response(&context.attr.expect);
    try_parse(quote!({
        #import_stmt
        #send_request_stmt
        #check_response_stmt
        #return_stmt
    }))
}

macro_rules! use_idents {
    ($($idents:ident),*) => {
        $(let $idents = quote!($idents);)*
    };
}

fn import() -> TokenStream {
    quote!(
        #[allow(unused_imports)]
        use interfacer_http::{
            http::{StatusCode, header::CONTENT_TYPE, Response},
            ContentInto, ToContent, Unexpected,
        };
    )
}

fn send_request(request: TokenStream) -> TokenStream {
    use_idents!(_resp);
    quote!(
        let #_resp = self.request(#request).await?;
    )
}

fn check_response(
    Expect {
        status,
        content_type,
    }: &Expect,
) -> TokenStream {
    use_idents!(_resp);
    quote!(
        if #status != #_resp.status() {
            return Err(Unexpected::UnexpectedStatusCode(#_resp).into());
        }
        match #_resp.headers().get(CONTENT_TYPE) {
            None => return Err(Unexpected::UnexpectedContentType(#_resp).into()),
            Some(content_type) if !self.helper().match_mime(&#content_type, content_type) => return Err(Unexpected::UnexpectedContentType(#_resp).into()),
            _ => (),
        }
    )
}

fn return_response(
    Expect {
        status: _,
        content_type,
    }: &Expect,
) -> TokenStream {
    use_idents!(_resp);
    quote!(
        Ok({
            let (_parts, _body) = #_resp.into_parts();
            Response::from_parts(
                _parts,
                _body.content_into(&#content_type)?,
            )
        })
    )
}

// TODO: using generic Body type
fn build_request(Context { attr, params }: &Context) -> Result<TokenStream, Diagnostic> {
    let method = attr.req.method.as_str();
    let content_type = &attr.req.content_type;
    let add_headers = gen_headers(params);
    let body = match params.body.as_ref() {
        Some(body) => quote!(#body.to_content_map_err(&#content_type)?),
        None => quote!(Vec::new()),
    };
    let uri_format_expr = gen_uri_format_expr(&attr.req.path, params)?;
    Ok(quote!(
        self
            .helper()
            .request()
            .uri(self.helper().parse_uri(&#uri_format_expr)?.as_str())
            #add_headers
            .method(#method)
            .body(#body)?
    ))
}

fn gen_headers(params: &Parameters) -> TokenStream {
    params.headers.iter().fold(
        quote!(),
        |ret, (key, value)| quote!(#ret.header(#key, #value)),
    )
}

mod format_uri {
    use super::Parameters;
    use crate::parse::try_parse;
    use lazy_static::lazy_static;
    use proc_macro::{Diagnostic, Level};
    use proc_macro2::{Ident, Span};
    use quote::quote;
    use regex::Regex;
    use syn::{parse_quote, punctuated::Punctuated, Expr, Macro, Token};

    const DYN_URI_PATTERN: &str = r#"(?P<pattern>\{\w+})"#;

    pub fn gen_uri_format_expr(raw_uri: &str, params: &Parameters) -> Result<Macro, Diagnostic> {
        lazy_static! {
            static ref URI_REGEX: Regex = Regex::new(DYN_URI_PATTERN).unwrap();
        };
        let mut uri_template = raw_uri.to_owned();
        let mut format_expr = try_parse::<Macro>(quote!(format!()))?;
        let mut values = Vec::new();
        let mut param_list = Punctuated::<Expr, Token![,]>::new();
        for capture in URI_REGEX.captures_iter(raw_uri) {
            let pattern: &str = &capture["pattern"];
            match params.values.get(&Ident::new(
                pattern.trim_start_matches('{').trim_end_matches('}'),
                Span::call_site(),
            )) {
                Some(ident) => values.push(ident),
                None => {
                    return Err(Diagnostic::new(
                        Level::Error,
                        format!("uri template variable {} has no parameter support", pattern),
                    ));
                }
            }
            uri_template = uri_template.replace(pattern, "{}");
        }
        param_list.push(parse_quote!(#uri_template));
        for value in values {
            param_list.push(parse_quote!(#value));
        }
        format_expr.tokens = quote!(#param_list);
        Ok(format_expr)
    }

    // TODO: complete test
    #[cfg(test)]
    mod test {
        use super::DYN_URI_PATTERN;
        use regex::Regex;

        fn parse_dyn_uri(raw_uri: &str) {
            for capture in Regex::new(DYN_URI_PATTERN).unwrap().captures_iter(raw_uri) {
                println!("captured {}", &capture["pattern"]);
            }
        }

        #[test]
        fn parse_dyn_uri_test() {
            parse_dyn_uri("/api/user/{id}?age={age}");
        }
    }
}

//// TODO: remove it when async_trait support formal parameter attributes
//mod polyfill {
//    use super::*;
//    use syn::FnArg;
//
//    pub fn remove_params_attributes(raw_method: &mut TraitItemMethod) {
//        for arg in raw_method.sig.inputs.iter_mut() {
//            if let FnArg::Typed(pat) = arg {
//                pat.attrs = Vec::new()
//            }
//        }
//    }
//}
