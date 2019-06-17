use http::Method;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, TraitItemMethod};

#[derive(Debug)]
pub struct Args {
    pub path: String,
    pub content_type: Option<String>,
}

pub fn request(method: Method, args: AttributeArgs, signature: TraitItemMethod) -> TokenStream {
    quote!(#signature)
}
