use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;

pub fn implement(item_trait: ItemTrait) -> TokenStream {
    let attr = &item_trait.attrs;
    let vis = &item_trait.vis;
    let ident = &item_trait.ident;
    let items = &item_trait.items;
    quote! {
        #(#attr)*
        #vis trait #ident: interfacer::http::HttpService {
            #(#items)*
        }
        impl<T: interfacer::http::HttpService> #ident for T {}
    }
}
