use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};

#[proc_macro_attribute]
pub fn extract_comments(_: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as ItemMod);
    let comments = module
        .attrs
        .iter()
        .filter_map(|attr| {
            if let Ok(meta) = attr.parse_meta() {
                if meta.path().is_ident("doc") {
                    if let Some(comment) = meta
                        .nested
                        .iter()
                        .filter_map(|nested_meta| match nested_meta {
                            syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                                if name_value.path.is_ident("value") {
                                    Some(name_value.lit.clone())
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                        .next()
                    {
                        return Some(comment);
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();

    let output = quote! {
        #module

        #[allow(dead_code)]
        const COMMENTS: &[&str] = &[#(#comments),*];
    };

    output.into()
}
