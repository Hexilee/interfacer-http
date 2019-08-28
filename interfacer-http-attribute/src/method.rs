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
    let define_content_type_stmt = define_content_type(&context.attr);
    let send_request_stmt = send_request(build_request(&context)?);
    let check_response_stmt = check_response(&context.attr.expect);
    let return_stmt = return_response(&context.attr.expect);
    try_parse(quote!({
        #import_stmt
        #define_content_type_stmt
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
            mime::Mime,
            http::{StatusCode, header::CONTENT_TYPE, Response},
            ContentInto, ToContent, Unexpected,
        };
    )
}

fn define_content_type(Attr { req, expect }: &Attr) -> TokenStream {
    use_idents!(_req_content_type, _expect_content_type);
    let define_req_content_type = match &req.content_type {
        Some(content_type) => quote!(let #_req_content_type: Mime = #content_type;),
        None => quote!(),
    };
    let define_expect_content_type = match &expect.content_type {
        Some(content_type) => quote!(let #_expect_content_type: Mime = #content_type;),
        None => quote!(),
    };
    quote!(
        #define_req_content_type
        #define_expect_content_type
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
    use_idents!(_resp, _expect_content_type);
    let check_content_type = match content_type {
        Some(_) => quote!(
        match #_resp.headers().get(CONTENT_TYPE) {
            None => return Err(Unexpected::new((CONTENT_TYPE, "Content-Type not found".to_owned()).into(), #_resp).into()),
            Some(content_type) if !self.helper().match_mime(&#_expect_content_type, content_type) =>
                return Err(Unexpected::new((CONTENT_TYPE, String::new()).into(), #_resp).into()),
            _ => (),
        }),
        None => quote!(),
    };
    quote!(
        if #status != #_resp.status() {
            return Err(Unexpected::new(#status.into(), #_resp).into());
        }
        #check_content_type
    )
}

fn return_response(expect: &Expect) -> TokenStream {
    use_idents!(_resp, _expect_content_type);
    let resp = match expect.content_type {
        Some(_) => quote!({
            let (_parts, _body) = #_resp.into_parts();
            Response::from_parts(
                _parts,
                _body.content_into(&#_expect_content_type)?,
            )
        }),
        None => quote!(Response::from_parts(#_resp.into_parts().0,())),
    };
    quote!(
        Ok(#resp)
    )
}

// TODO: using generic Body type
fn build_request(Context { attr, params }: &Context) -> Result<TokenStream, Diagnostic> {
    use_idents!(_req_content_type);
    let method = attr.req.method.as_str();
    let mut headers = gen_headers(params);
    match attr.req.content_type {
        Some(_) => headers.push(quote!(header(CONTENT_TYPE, #_req_content_type.as_ref()))),
        None => (),
    };
    let body = match (params.body.as_ref(), &attr.req.content_type) {
        (Some(body), Some(_)) => quote!(#body.to_content(&#_req_content_type)?),
        _ => quote!(Vec::new()),
    };
    let uri_format_expr = gen_uri_format_expr(&attr.req.path, params)?;
    Ok(quote!(
        self
            .helper()
            .request()
            .uri(self.helper().parse_uri(&#uri_format_expr)?.as_str())
            #(.#headers)*
            .method(#method)
            .body(#body)?
    ))
}

fn gen_headers(params: &Parameters) -> Vec<TokenStream> {
    params
        .headers
        .iter()
        .map(|(key, value)| quote!(header(#key, #value)))
        .collect()
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

    const DYN_URI_PATTERN: &str = r#"(?P<value>\{\w+})"#;
    const VAL_NAME: &str = "value";

    pub fn gen_uri_format_expr(raw_uri: &str, params: &Parameters) -> Result<Macro, Diagnostic> {
        lazy_static! {
            static ref URI_REGEX: Regex = Regex::new(DYN_URI_PATTERN).unwrap();
        };
        let mut uri_template = raw_uri.to_owned();
        let mut format_expr = try_parse::<Macro>(quote!(format!()))?;
        let mut values = Vec::new();
        let mut param_list = Punctuated::<Expr, Token![,]>::new();
        for capture in URI_REGEX.captures_iter(raw_uri) {
            let pattern: &str = &capture[VAL_NAME];
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
        use super::{gen_uri_format_expr, Ident, Parameters, Span, DYN_URI_PATTERN, VAL_NAME};
        use proc_macro2::TokenStream;
        use quote::quote;
        use regex::Regex;

        #[test]
        fn test_dyn_uri_match() {
            fn dyn_uri_match(raw_uri: &str, value_list: &[&str]) {
                assert_eq!(
                    value_list
                        .iter()
                        .map(|value_name| (*value_name).to_owned())
                        .collect::<Vec<String>>(),
                    Regex::new(DYN_URI_PATTERN)
                        .unwrap()
                        .captures_iter(raw_uri)
                        .map(|cap| cap[VAL_NAME].to_owned())
                        .collect::<Vec<String>>()
                );
            }
            dyn_uri_match("/api/user/{id}?age={age}", &["{id}", "{age}"][..]);
            dyn_uri_match("/api/user/", &[][..]);
            dyn_uri_match("/api/user/{id}", &["{id}"][..]);
            dyn_uri_match("/api/user/{id}/name", &["{id}"][..]);
            dyn_uri_match("/api/user-{id}/name", &["{id}"][..]);
        }

        fn assert_gen_uri_format_expr(uri: &str, values: &[&str], expect_token: TokenStream) {
            let parameters = Parameters {
                values: values
                    .iter()
                    .map(|value| Ident::new(*value, Span::call_site()))
                    .collect(),
                headers: Vec::new(),
                body: None,
            };
            let token = gen_uri_format_expr(uri, &parameters).unwrap();
            assert_eq!(expect_token.to_string(), quote!(#token).to_string());
        }

        #[test]
        fn test_gen_uri_format_expr() {
            assert_gen_uri_format_expr(
                "/api/user/{id}?age={age}",
                &["id", "age"][..],
                quote!(format!("/api/user/{}?age={}", id, age)),
            );
            assert_gen_uri_format_expr(
                "/api/user/{id}",
                &["id", "age"][..],
                quote!(format!("/api/user/{}", id)),
            );
            assert_gen_uri_format_expr(
                "/api/user",
                &["id", "age"][..],
                quote!(format!("/api/user")),
            );
            assert_gen_uri_format_expr(
                "/api/user-{id}",
                &["id", "age"][..],
                quote!(format!("/api/user-{}", id)),
            );
        }

        #[test]
        #[should_panic]
        fn test_gen_uri_format_expr_no_param_support() {
            assert_gen_uri_format_expr(
                "/api/user/{id}?age={age}",
                &["id"][..],
                quote!(format!("/api/user/{}?age={}", id, age)),
            );
        }
    }
}
