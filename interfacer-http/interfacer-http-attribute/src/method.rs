use crate::parse::parse_args;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, TraitItemMethod};

use crate::args::Args;
use crate::format_uri::gen_uri_format_expr;
use proc_macro::Diagnostic;

macro_rules! use_idents {
    ($($idents:ident),*) => {
        $(let $idents = quote!($idents);)*
    };
}

// TODO: this function is too complicated, decouple it
pub fn transform_method(raw_method: &mut TraitItemMethod) -> Result<(), Diagnostic> {
    let args = parse_args(raw_method)?;
    use_idents!(
        req_ident,
        parts_ident,
        body_ident,
        expect_content_type_ident,
        uri_format_ident,
        final_uri_ident
    );
    let import = quote!(
        use interfacer_http::{
            RequestFail, ContentType,
            http::{StatusCode, header::CONTENT_TYPE},
            IntoStruct, ToContent, HttpClient, StringError,
        };
    );
    let uri_format_expr = gen_uri_format_expr(&args.req.path)?;
    let uri_format_statement = quote!(let #uri_format_ident = #uri_format_expr;);
    let define_final_uri =
        quote!(let #final_uri_ident = self.get_base_url().join(&#uri_format_ident)?;);
    let expect_content_base_type = args.expect.content_type.base_type.as_str();
    let define_expect_content_type = match args.expect.content_type.encoding.as_ref() {
        Some(encoding) => quote!(
            let #expect_content_type_ident = ContentType::new(#expect_content_base_type, Some(#encoding), None);
        ),
        None => quote!(
            let #expect_content_type_ident = ContentType::new(#expect_content_base_type, None, None);
        ),
    };
    let req_define = build_request(&args, &raw_method);
    let send_request = quote!(
        let (#parts_ident, #body_ident) = self.get_client().request(#req_ident).await.map_err(|err| err.into())?.into_parts();
    );
    let expect_status = args.expect.status.as_u16();
    let check_resp = quote!(
        RequestFail::expect_status(StatusCode::from_u16(#expect_status).unwrap(), #parts_ident.status)?;
        let ret_content_type = parts_ident.headers.get(CONTENT_TYPE).ok_or(
            StringError::new("cannot get Content-Type from response headers")
        )?;
        #expect_content_type_ident.expect(&ContentType::from_header(ret_content_type)?)?;
    );
    let ret = quote!(
        Ok(#body_ident.into_struct(&#expect_content_type_ident)?)
    );
    let body = quote!(
        #import
        #uri_format_statement
        #define_final_uri
        #define_expect_content_type
        #req_define
        #send_request
        #check_resp
        #ret
    );
    raw_method.semi_token = None;
    raw_method.default = Some(parse_quote!({
        #body
    }));
    Ok(())
}

// TODO: complete build request; using generic Body type
fn build_request(args: &Args, _raw_method: &TraitItemMethod) -> TokenStream {
    let method = args.req.method.as_str();
    use_idents!(req_ident, final_uri_ident);
    quote!(
        let mut builder = interfacer_http::http::Request::builder();
        let #req_ident = builder
            .uri(#final_uri_ident.as_str())
            .method(#method)
            .body(Vec::new())?;
    )
}
