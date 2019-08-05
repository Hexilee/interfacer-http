use crate::method::transform_method;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemTrait, TraitItem};

pub fn implement(mut item_trait: ItemTrait) -> TokenStream {
    let trait_name = item_trait.ident.clone();
    for item in item_trait.items.iter_mut() {
        if let TraitItem::Method(method) = item {
            transform_method(method).unwrap_or_else(|err| {
                err.emit();
                std::process::exit(1);
            });
        }
    }
    item_trait
        .supertraits
        .push(parse_quote!(interfacer_http::HttpService));
    item_trait
        .supertraits
        .push(parse_quote!(core::marker::Sync));
    let super_traits = item_trait.supertraits.clone();
    quote! {
    //        #[interfacer_http::async_trait]
            #item_trait

    //        #[interfacer_http::async_trait]
            impl<T: #super_traits> #trait_name for T {

            }
        }
}
