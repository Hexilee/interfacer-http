use proc_macro2::TokenStream;
use quote::quote;
use syn::{TraitItemMethod, ReturnType, Token, parse_macro_input, Type};
use darling::FromMeta;

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

pub fn request(method: &str, args: Args, mut raw_method: TraitItemMethod) -> TokenStream {
    let attr = &raw_method.attrs;
    let req_ident = quote!(req);
    let req_define = build_request(&req_ident, method, &args, &raw_method);
    let return_type = TokenStream::from(quote!(<<Self as interfacer::http::HttpService>::Client as interfacer::http::HttpClient>::Response));
    let output = &mut raw_method.sig.decl.output;
    let raw_sig = match output {
        ReturnType::Default => {
            *output = ReturnType::Type(Token![->], Box::new(parse_macro_input!(return_type as Type)));
        }

        ReturnType::Type(_, typ) => {
            *output = ReturnType::Type(Token![->], Box::new(parse_macro_input!(return_type as Type)));
        }
    };
    let return_block = quote!(self.get_client().request(#req_ident));
    quote!(
        #($attr)*
        #raw_sig {
            #req_define
            #return_block
        }
    )
}

// TODO: complete build request; replace unwrap with try; using generic Body type
fn build_request(
    req_ident: &TokenStream,
    method: &str,
    args: &Args,
    raw_method: &TraitItemMethod,
) -> TokenStream {
    let path = match args.path {
        Some(ref path) => path,
        None => "/"
    };
    quote!(
        let mut builder = interfacer::http::Request::builder();
        let #req_ident = builder
            .uri(#path)
            .method(#method)
            .body(vec![])
            .unwrap();
    )
}
