use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, TraitItemMethod};

use crate::attr::Attr;
use crate::param::Parameters;
use format_uri::gen_uri_format_expr;
use interfacer_http_util::http::StatusCode;
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

pub fn transform_method(raw_method: &mut TraitItemMethod) -> Result<(), Diagnostic> {
    let context = Context::parse(raw_method)?;
    let import_stmt = import();
    let define_final_uri = gen_final_uri(&context)?;
    let define_expect_content_type = gen_expect_content_type(&context);
    let define_request = build_request(&context);
    let send_request_stmt = send_request();
    let check_response_stmt = check_response(&context);
    let ret = ret();
    let body = quote!(
        #import_stmt
        #define_final_uri
        #define_expect_content_type
        #define_request
        #send_request_stmt
        #check_response_stmt
        #ret
    );
    raw_method.semi_token = None;
    raw_method.default = Some(parse_quote!({
        #body
    }));
    Ok(())
}

macro_rules! use_idents {
    ($($idents:ident),*) => {
        $(let $idents = quote!($idents);)*
    };
}

fn import() -> TokenStream {
    quote!(
        use interfacer_http::{
            RequestFail, ContentType,
            http::{StatusCode, header::CONTENT_TYPE, Response},
            IntoStruct, ToContent, HttpClient, StringError,
        };
        use std::convert::TryInto;
    )
}

fn gen_final_uri(Context { attr, params }: &Context) -> Result<TokenStream, Diagnostic> {
    use_idents!(final_uri_ident);
    let uri_format_expr = gen_uri_format_expr(&attr.req.path)?;
    Ok(quote!(
        let #final_uri_ident = self.get_base_url().join(&#uri_format_expr)?;
    ))
}

fn gen_expect_content_type(Context { attr, params }: &Context) -> TokenStream {
    use_idents!(expect_content_type_ident);
    let expect_content_type = &attr.expect.content_type;
    quote!(
        let #expect_content_type_ident: ContentType = #expect_content_type.try_into()?;
    )
}

fn send_request() -> TokenStream {
    use_idents!(request_ident, parts_ident, body_ident);
    quote!(
        let (#parts_ident, #body_ident) = self.get_client().request(#request_ident).await.map_err(|err| err.into())?.into_parts();
    )
}

fn check_response(Context { attr, params }: &Context) -> TokenStream {
    use_idents!(parts_ident, expect_content_type_ident);
    let expect_status = &attr.expect.status;
    quote!(
        RequestFail::expect_status(#expect_status, #parts_ident.status)?;
        let ret_content_type = parts_ident.headers.get(CONTENT_TYPE).ok_or(
            StringError::new("cannot get Content-Type from response headers")
        )?;
        #expect_content_type_ident.expect(&ContentType::from_header(ret_content_type)?)?;
    )
}

fn ret() -> TokenStream {
    use_idents!(body_ident, expect_content_type_ident, parts_ident);
    quote!(
        Ok(
            Response::from_parts(
                #parts_ident,
                #body_ident.into_struct(&#expect_content_type_ident)?,
            ).into(),
        )
    )
}

// TODO: complete build request; using generic Body type
fn build_request(Context { attr, params }: &Context) -> TokenStream {
    let method = attr.req.method.as_str();
    use_idents!(request_ident, final_uri_ident);
    quote!(
        let mut builder = interfacer_http::http::Request::builder();
        let #request_ident = builder
            .uri(#final_uri_ident.as_str())
            .method(#method)
            .body(Vec::new())?;
    )
}

mod format_uri {
    use crate::parse::try_parse;
    use lazy_static::lazy_static;
    use proc_macro::Diagnostic;
    use proc_macro2::{Ident, Span};
    use quote::quote;
    use regex::Regex;
    use syn::{parse_quote, punctuated::Punctuated, Expr, Macro, Token};

    const DYN_URI_PATTERN: &str = r#"(?P<pattern>\{\w+})"#;

    pub fn gen_uri_format_expr(raw_uri: &str) -> Result<Macro, Diagnostic> {
        lazy_static! {
            static ref URI_REGEX: Regex = Regex::new(DYN_URI_PATTERN).unwrap();
        };
        let mut uri_template = raw_uri.to_owned();
        let mut format_expr = try_parse::<Macro>(quote!(format!()).into())?;
        let mut varables = Vec::new();
        let mut param_list = Punctuated::<Expr, Token![,]>::new();
        for capture in URI_REGEX.captures_iter(raw_uri) {
            let pattern: &str = &capture["pattern"];
            varables.push(
                pattern
                    .trim_start_matches('{')
                    .trim_end_matches('}')
                    .to_owned(),
            );
            uri_template = uri_template.replace(pattern, "{}");
        }
        param_list.push(parse_quote!(#uri_template));
        for variable in varables {
            let ident = Ident::new(&variable, Span::call_site());
            param_list.push(parse_quote!(#ident));
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
                println!("captured {}", &capture["var"]);
            }
        }

        #[test]
        fn parse_dyn_uri_test() {
            parse_dyn_uri("/api/user/{id}?age={age}");
        }
    }
}
