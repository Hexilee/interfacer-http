use interfacer::http::Method;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, Ident, TraitItemMethod};

#[derive(Debug)]
pub struct Args {
    pub path: Ident,
    pub content_type: Option<Ident>,
    pub send: bool,
}

impl Args {
    fn new(raw_args: AttributeArgs) -> Self {
        unimplemented!()
    }
}

pub fn request(method: Method, raw_args: AttributeArgs, signature: TraitItemMethod) -> TokenStream {
    let args = Args::new(raw_args);
    quote!(#signature)
}
