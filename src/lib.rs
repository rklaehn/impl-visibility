use proc_macro::*;
use syn::{token::Pub, ImplItem, VisPublic, Visibility};
use quote::ToTokens;

#[proc_macro_attribute]
pub fn public_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(input).unwrap();
    let impl_item = match &mut item {
        syn::Item::Impl(impl_item) => impl_item,
        _ => panic!("this macro is only valid for impl items"),
    };
    if impl_item.trait_.is_some() {
        panic!("adding pub does not make sense in impl trait items")
    }
    let vis_pub: Visibility = Visibility::Public(VisPublic {
        pub_token: Pub::default(),
    });
    for item in &mut impl_item.items {
        match item {
            ImplItem::Method(method_item) => method_item.vis = vis_pub.clone(),
            ImplItem::Const(const_item) => const_item.vis = vis_pub.clone(),
            ImplItem::Type(type_item) => type_item.vis = vis_pub.clone(),
            _ => {}
        }
    }
    item.into_token_stream().into()
}
