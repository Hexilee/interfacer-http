use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, TraitItemMethod};

use crate::attr::{Attr, Request};
use crate::param::Parameters;
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

pub fn transform_method(raw_method: &mut TraitItemMethod) -> Result<(), Diagnostic> {
    let context = Context::parse(raw_method)?;
    let import_stmt = import();
    let send_request_stmt = send_request(build_request(&context)?);
    let check_response_stmt = check_response(&context);
    let ret = ret();
    let body = quote!(
            #import_stmt
    //        #define_expect_content_type
            #send_request_stmt
            #check_response_stmt
            #ret
        );
    polyfill::remove_params_attributes(raw_method); // TODO: remove it when async_trait support formal parameter attributes
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
        #[allow(unused_imports)]
        use interfacer_http::{
            RequestFail, ContentType,
            http::{StatusCode, header::CONTENT_TYPE, Response},
            IntoStruct, ToContent, HttpClient, StringError,
        };
        #[allow(unused_imports)]
        use std::convert::TryInto;
    )
}

fn expect_content_type(Context { attr, params: _ }: &Context) -> TokenStream {
    let expect_content_type = &attr.expect.content_type;
    quote!(
        #expect_content_type.parse()?;
    )
}

fn send_request(request: TokenStream) -> TokenStream {
    use_idents!(_parts, _body);
    quote!(
        let (#_parts, #_body) = self.request(#request).await.map_err(|err| err.into())?.into_parts();
    )
}

fn check_response(Context { attr, params: _ }: &Context) -> TokenStream {
    use_idents!(_parts, _expect_content_type);
    let expect_status = &attr.expect.status;
    quote!(
        RequestFail::expect_status(#expect_status, #_parts.status)?;
        let _ret_content_type = _parts.headers.get(CONTENT_TYPE).ok_or(
            StringError::new("cannot get Content-Type from response headers")
        )?;
        #_expect_content_type.expect(&ContentType::from_header(_ret_content_type)?)?;
    )
}

fn ret() -> TokenStream {
    use_idents!(_body, _expect_content_type, _parts);
    quote!(
        Ok(
            Response::from_parts(
                #_parts,
                #_body.into_struct(&#_expect_content_type)?,
            ).into(),
        )
    )
}

// TODO: using generic Body type
fn build_request(Context { attr, params }: &Context) -> Result<TokenStream, Diagnostic> {
    let method = attr.req.method.as_str();
    let headers = gen_headers(params);
    let body = match params.body.as_ref() {
        Some(body) => {
            let content_type = gen_request_content_type(&attr.req);
            quote!(#body.to_content(&#content_type)?)
        }
        None => quote!(Vec::new()),
    };
    let uri_format_expr = gen_uri_format_expr(&attr.req.path, params)?;
    Ok(quote!(
        self
            .config()
            .request()
            .uri(self.config.parse_uri(&#uri_format_expr)?)
            #headers
            .method(#method)
            .body(#body)?;
    ))
}

fn gen_request_content_type(req: &Request) -> TokenStream {
    let request_content_type = &req.content_type;
    quote!(
        #request_content_type.try_into()?
    )
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
        let mut format_expr = try_parse::<Macro>(quote!(format!()).into())?;
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

// TODO: remove it when async_trait support formal parameter attributes
mod polyfill {
    use super::*;
    use syn::FnArg;

    pub fn remove_params_attributes(raw_method: &mut TraitItemMethod) {
        for arg in raw_method.sig.inputs.iter_mut() {
            if let FnArg::Typed(pat) = arg {
                pat.attrs = Vec::new()
            }
        }
    }
}
