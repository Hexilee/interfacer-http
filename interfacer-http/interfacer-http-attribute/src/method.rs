use crate::parse::parse_args;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, TraitItemMethod};

use crate::args::Args;
use crate::format_uri::gen_uri_format_expr;
use proc_macro::Diagnostic;

pub fn transform_method(raw_method: &mut TraitItemMethod) -> Result<(), Diagnostic> {
    let args = parse_args(raw_method)?;
    let import_stmt = import();
    let define_final_uri = gen_final_uri(&args)?;
    let define_expect_content_type = gen_expect_content_type(&args);
    let define_request = build_request(&args, &raw_method);
    let send_request_stmt = send_request();
    let check_response_stmt = check_response(&args);
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
            http::{StatusCode, header::CONTENT_TYPE},
            IntoStruct, ToContent, HttpClient, StringError,
        };
    )
}

fn gen_final_uri(args: &Args) -> Result<TokenStream, Diagnostic> {
    use_idents!(final_uri_ident);
    let uri_format_expr = gen_uri_format_expr(&args.req.path)?;
    Ok(quote!(
        let #final_uri_ident = self.get_base_url().join(&#uri_format_expr)?;
    ))
}

fn gen_expect_content_type(args: &Args) -> TokenStream {
    use_idents!(expect_content_type_ident);
    let expect_content_base_type = args.expect.content_type.base_type.as_str();
    match args.expect.content_type.encoding.as_ref() {
        Some(encoding) => quote!(
            let #expect_content_type_ident = ContentType::new(#expect_content_base_type, Some(#encoding), None);
        ),
        None => quote!(
            let #expect_content_type_ident = ContentType::new(#expect_content_base_type, None, None);
        ),
    }
}

fn send_request() -> TokenStream {
    use_idents!(request_ident, parts_ident, body_ident);
    quote!(
        let (#parts_ident, #body_ident) = self.get_client().request(#request_ident).await.map_err(|err| err.into())?.into_parts();
    )
}

fn check_response(args: &Args) -> TokenStream {
    use_idents!(parts_ident, expect_content_type_ident);
    let expect_status = args.expect.status.as_u16();
    quote!(
        RequestFail::expect_status(StatusCode::from_u16(#expect_status).unwrap(), #parts_ident.status)?;
        let ret_content_type = parts_ident.headers.get(CONTENT_TYPE).ok_or(
            StringError::new("cannot get Content-Type from response headers")
        )?;
        #expect_content_type_ident.expect(&ContentType::from_header(ret_content_type)?)?;
    )
}

fn ret() -> TokenStream {
    use_idents!(body_ident, expect_content_type_ident);
    quote!(
        Ok(#body_ident.into_struct(&#expect_content_type_ident)?)
    )
}

// TODO: complete build request; using generic Body type
fn build_request(args: &Args, _raw_method: &TraitItemMethod) -> TokenStream {
    let method = args.req.method.as_str();
    use_idents!(request_ident, final_uri_ident);
    quote!(
        let mut builder = interfacer_http::http::Request::builder();
        let #request_ident = builder
            .uri(#final_uri_ident.as_str())
            .method(#method)
            .body(Vec::new())?;
    )
}
