use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemTrait};

pub fn implement(mut item_trait: ItemTrait) -> TokenStream {
    let items = item_trait.items.clone();
    let trait_name = item_trait.ident.clone();
    item_trait
        .supertraits
        .push(parse_quote!(interfacer_http::HttpService));
    let supertraits = item_trait.supertraits.clone();
    quote! {
        #[interfacer_http::async_trait]
        #item_trait

        #[interfacer_http::async_trait]
        impl<T: #supertraits> #trait_name for T {
            #(#items);*
        }
    }
}
