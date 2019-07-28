use crate::method::transform_method;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemTrait, TraitItem};

pub fn implement(mut item_trait: ItemTrait) -> TokenStream {
    let trait_name = item_trait.ident.clone();
    let methods = item_trait
        .items
        .clone()
        .into_iter()
        .map(|item| match item {
            TraitItem::Method(method) => transform_method(method).into(),
            _ => quote!(),
        })
        .collect::<TokenStream>();
    item_trait
        .supertraits
        .push(parse_quote!(interfacer_http::HttpService));
    let super_traits = item_trait.supertraits.clone();
    quote! {
            #[interfacer_http::async_trait]
            #item_trait

            #[interfacer_http::async_trait]
            impl<T: #super_traits> #trait_name for T
    //            where interfacer_http::RequestFail: From<<<T as interfacer_http::HttpService>::Client as interfacer_http::HttpClient>::Err>
            {
                #(#methods)*
            }
        }
}
