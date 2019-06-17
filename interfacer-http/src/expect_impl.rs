use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, TraitItemMethod};

pub fn implement(args: AttributeArgs, signature: TraitItemMethod) -> TokenStream {
    quote! {
        #signature
    }
}
