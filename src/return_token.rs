use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn main_token(
    name: Ident,
    sc_name: Ident,
    sc_enum: ItemEnum,
    sc_match: ExprMatch,
    body: ItemMod,
) -> TokenStream {
    quote! {
        use clap::{ Args, Parser, Subcommand };

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

        #body
    }
    .into()
}

pub fn mod_token(
    name: Ident,
    sc_name: Ident,
    sc_enum: ItemEnum,
    sc_match: ExprMatch,
    body: ItemMod,
) -> TokenStream {
    quote! {
        use clap::{ Args, Parser, Subcommand };

        #[derive(Debug, Clone, Args)]
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

        #body
    }
    .into()
}

pub fn fn_token(
    name: Ident,
    item_call: ExprCall,
    fields: Fields,
    new_function: ItemFn,
) -> TokenStream {
    quote! {
        #[derive(Debug, Clone, Args)]
        pub struct #name {
            #fields
        }

        impl #name {
            pub fn delegate(&self) {
                eprintln!("{}", #item_call);
            }

            #new_function
        }
    }
    .into()
}
