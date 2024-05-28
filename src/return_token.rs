use proc_macro::TokenStream;
use quote::quote;
use syn::{ExprMatch, Ident, ImplItem, Item, ItemEnum, ItemMod};

pub fn main_token(
    name: Ident,
    sc_name: Ident,
    sc_enum: ItemEnum,
    sc_match: ExprMatch,
    item: ItemMod,
) -> TokenStream {
    quote! {
        use clap::{Parser, Subcommand};

        #[derive(Debug, Clone, Parser)]
        #[clap(author, version, about)] // Maybe later version more comfort..
        pub struct #name {
            #[clap(subcommand)]
            pub subcommand: #sc_name,
        }
        impl #name {
            pub fn delegate(self) {
                self.subcommand.delegate();
            }
        }

        #[derive(Debug, Clone, Subcommand)]
        #sc_enum

        impl #sc_name {
            pub fn delegate(self) {
                #sc_match
            }
        }

        #item
    }
    .into()
}

pub fn mod_token(
    name: Ident,
    sc_name: Ident,
    sc_enum: ItemEnum,
    sc_match: ExprMatch,
    item: ItemMod,
) -> TokenStream {
    quote! {
        #[derive(Debug, Clone, Args)]
        pub struct #name {
            #[clap(subcommand)]
            pub subcommand: #sc_name,
        }
        impl #name {
            pub fn delegate(self) {
                self.subcommand.delegate();
            }
        }

        #[derive(Debug, Clone, Subcommand)]
        #sc_enum

        impl #sc_name {
            pub fn delegate(self) {
                #sc_match
            }
        }

        #item
    }
    .into()
}

pub fn fn_token(name: Ident, struct_item: Item, new_function: ImplItem) -> TokenStream {
    quote! {
        #struct_item

        impl #name {
            #new_function
        }
    }
    .into()
}
