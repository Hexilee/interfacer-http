use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemTrait};

pub fn implement(mut item_trait: ItemTrait) -> TokenStream {
    let items = item_trait.items.clone();
    let trait_name = item_trait.ident.clone();
    quote! {
        #[interface_http::async_trait]
        #item_trait

        #[interface_http::async_trait]
        impl<T: interfacer::http::HttpService> #trait_name for T {

        }
    }
}
