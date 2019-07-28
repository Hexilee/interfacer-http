use crate::parse::parse_args;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, TraitItemMethod};

use crate::args::Args;

macro_rules! define_idents {
    ($($idents:ident),*) => {
        $(let $idents = quote!($idents);)*
    };
}

// TODO: finish check_resp
pub fn transform_method(raw_method: &mut TraitItemMethod) {
    let args = parse_args(raw_method).unwrap_or_else(|err| {
        err.emit();
        std::process::exit(1);
    });
    define_idents!(req_ident, parts_ident, body_ident, _expect_content_type);
    let import = quote!(
        use interfacer_http::{
            RequestFail, ContentType, http::StatusCode, IntoStruct, ToContent, HttpClient,
        };
    );
    let expect_content_base_type = args.expect.content_type.base_type.as_str();
    let define_expect_content_type = match args.expect.content_type.encoding.as_ref() {
        Some(encoding) => quote!(
            let #_expect_content_type = ContentType::new(#expect_content_base_type, Some(#encoding), None);
        ),
        None => quote!(
            let #_expect_content_type = ContentType::new(#expect_content_base_type, None, None);
        ),
    };
    let req_define = build_request(&req_ident, &args, &raw_method);
    let send_request = quote!(
        let (#parts_ident, #body_ident) = self.get_client().request(#req_ident).await.map_err(|err| err.into())?.into_parts();
    );
    let check_resp = quote!();
    let ret = quote!(
        Ok(#body_ident.into_struct(&#_expect_content_type)?)
    );
    let body = quote!(
        #import
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
            .body(Vec::new())?;
    )
}
