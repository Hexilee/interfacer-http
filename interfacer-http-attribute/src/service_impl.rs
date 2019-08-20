use crate::method::gen_block;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ImplItem, ImplItemMethod, ItemImpl, ItemTrait, TraitItem, Visibility};

pub fn implement(item_trait: ItemTrait) -> TokenStream {
    let trait_name = item_trait.ident.clone();
    let mut super_traits = item_trait.supertraits.clone();
    super_traits.push(parse_quote!(interfacer_http::HttpClient));

    let mut item_impl: ItemImpl = parse_quote!(
        #[interfacer_http::async_trait]
        impl<T: #super_traits> #trait_name for T {

        }
    );

    for item in item_trait.items.iter() {
        match item {
            TraitItem::Method(method) => item_impl.items.push(ImplItem::Method(ImplItemMethod {
                attrs: method.attrs.clone(),
                defaultness: None,
                vis: Visibility::Inherited,
                sig: method.sig.clone(),
                block: gen_block(method).unwrap_or_else(|err| {
                    err.emit();
                    std::process::exit(1)
                }),
            })),
            TraitItem::Type(typ) => {
                let ident = &typ.ident;
                item_impl.items.push(parse_quote!(
                    type #ident = <Self as interfacer_http::HttpClient>::Err;
                ));
            }
            _ => (),
        };
    }

    return quote! {
        #[interfacer_http::async_trait]
        #item_trait
        #item_impl
    };
}
