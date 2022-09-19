use proc_macro::*;
use syn::{ImplItem, Visibility, parse_macro_input};
use quote::ToTokens;

/// **Override**s the visibility of the annotated impl section with the one
/// given as an argument.
/// ```
#[proc_macro_attribute]
pub fn make(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let vis: Visibility = parse_macro_input!(attrs);
    let mut item: syn::Item = syn::parse(input).unwrap();
    let impl_item = match &mut item {
        syn::Item::Impl(impl_item) => impl_item,
        _ => panic!("this macro is only valid for impl items"),
    };
    if impl_item.trait_.is_some() {
        panic!("adding pub does not make sense in impl trait items")
    }
    for item in &mut impl_item.items {
        match item {
            ImplItem::Method(method_item) => method_item.vis = vis.clone(),
            ImplItem::Const(const_item) => const_item.vis = vis.clone(),
            ImplItem::Type(type_item) => type_item.vis = vis.clone(),
            _ => {}
        }
    }
    item.into_token_stream().into()
}
