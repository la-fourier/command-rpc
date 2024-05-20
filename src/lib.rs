use proc_macro::{Literal, Span, TokenStream, TokenTree};
use quote::*;

use std::collections::HashMap;
// use syn::{self, parse_macro_input, AttrStyle, Attribute, Item, Item::*, ItemFn, ItemMod};
use syn::{Item::*, __private::*, punctuated::Punctuated, token::Box, *};

mod checks;
// use checks::*;

mod build;
// use build::*;

mod return_token;
// use return_token::*;

/// Hey! Nice you like to write with the `cprc` module. It refers to clap, thank you for your great work there!
/// Unfortunately, you have to import clap in your project yourself with *derive* feature enabled.
///
/// # Quick setup
/// ```
/// pub mod my_cli_backend {
///    pub fn greet(name: str // The name of the person you want to greet.
///                 ) {
///       eprintln!("Hello, {}!", name);
///   }
///
///  pub mod my_cli_backend_sub {
///    pub fn friendly_greet(name: str // The name of the person you want to greet.
///                          adjective: str // the adjective you want to use in the greeting.
///                         ) -> Option<String> {
///      eprintln!("Hello, {}! You are {}!", name, adjective);
///     Some("You are friendly!".to_string())
///   }
/// }
/// ```
///
/// # Prettier requests to endpoint
///
/// To make call to your endpoint in your Rust programs nicer, you can insert this snippet
/// that defines a declarative macro (this is why it can´t be just exported from this crate).
/// Calls in your program would look like this then(Same for other languages will be coming):
/// ```
/// callback!(my_cli_backend::greet("John"));
/// ```
///  TODO!
/// ```
/// macro_rules! callback {
/// ($inp:expr) => {{
///     let mut cmd = $inp.to_string();
///     cmd = cmd.replace(";", "").replace(" ", "").replace("\n", "").replace("(", "").replace(")", "").replace("::", " ").replace(",", " ");
///     std::thread::spawn(move || {
///         let output = std::process::Command::new(cmd)
///             .output()
///             .expect("Failed to execute command");
///         eprintln!("{}", std::string::String::from_utf8_lossy(&output.stdout));
///     });
/// }};
/// }
/// ```
///
///
/// ---
///
///
/// # Integration to other languages
///
/// ## Python
///
/// To make call to your endpoint in your Rust programs nicer, you can insert this snippet
/// ```
/// todo!()
/// ```
///
/// TODO: Integration: Javascript, Flutter, Java
/// TODO: Clean Code conventions
/// TODO: Docs, Error handling
/// TODO: use doc attributes for the cli by copying them
///
///
///
///

#[proc_macro_attribute]
pub fn print_ast(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);
    eprintln!("{:#?}", item);
    item.to_token_stream().into()
}

macro_rules! callback {
    // For things like `callback!(my_cli_backend::...::greet("John"));`, when you want to call a function in your rust backend
    ($inp:expr) => {{
        let mut cmd = $inp.to_string();
        cmd = cmd
            .replace(";", "")
            .replace(" ", "")
            .replace("\n", "")
            .replace("(", "")
            .replace(")", "")
            .replace("::", " ")
            .replace(",", " ");
        std::thread::spawn(move || {
            let output = std::process::Command::new(cmd)
                .output()
                .expect("Failed to execute command");
            eprintln!("{}", std::string::String::from_utf8_lossy(&output.stdout));
        });
    }};
}

macro_rules! parse {
    // For the main function, that automatically parses the input and calls the right function - you have to pass the path to the crpc_main module
    ($inp:expr) => {
        let mut path = $inp.split("::").collect();
        let mut module = path.last();
        module = module[0].uppercase() + module[1..].to_string();
        path.push(&module);
        let path = path.join("::");
        path!($path).delegate();
    };
}

// lazy variant! Just mark the main module with [crpc_main], then everything gets expaneded recursively!

#[proc_macro_attribute]
pub fn crpc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ts_item = item.clone();
    // Parse the input tokens into a Rust syntax tree
    // Generate the output tokens
    let item = syn::parse_macro_input!(item as syn::Item);
    match item {
        Fn(item) if attr.to_string() == "fn" => {
            // For fn
            quote! {
                #[crpc_fn]
                #item
            }
            .into()
        }
        Mod(item) if attr.to_string() == "mod" => {
            // For mod
            quote! {
                #[crpc_mod]
                #item
            }
            .into()
        }
        Struct(item) if attr.to_string() == "struct" => {
            // For param test
            quote! {
                #[crpc_param]
                #item
            }
            .into()
        }
        _ => {
            eprint!(
                "Error in {:?}: crpc can only be used on fn, mod and struct",
                ts_item.to_string()
            );
            ts_item
        }
    }
}

#[proc_macro_attribute]
pub fn crpc_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);
    if let Mod(item) = &item {
        if let syn::Visibility::Public(_) = item.vis {
            let (name, sc_name) = build::subcommand::names(item.ident.to_string());
            let subcommands = build::subcommand::subcommands(item.clone());
            let sc_enum = build::subcommand::subcommand_enum(
                &name.to_string(),
                subcommands.clone(),
                sc_name.clone(),
            );
            let sc_match = build::subcommand::delegate_match_expr(
                &name.to_string(),
                subcommands.clone(),
                &sc_name,
            );

            // let (_, _body) = item.content.clone().unwrap(); // TODO: as at mod
            // let body = _body.iter().map(|item| -> syn::Stmt {
            //     parse_quote!(#item)
            // }).collect::<Vec<syn::Stmt>>();

            // Generate the output token
            let ret =
                return_token::main_token(name, sc_name, sc_enum, sc_match, item.clone());
            
            return ret;
        } else {
            eprintln!(
                "An item marked with #[crpc_main] must be public and accessible to the binary."
            );
            return item.to_token_stream().into();
        }
    } else {
        eprintln!("An item marked with #[crpc_main] must be a module.");
        return item.to_token_stream().into();
    }
}

#[proc_macro_attribute]
pub fn crpc_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);
    if let Mod(item) = &item {
        if let syn::Visibility::Public(_) = item.vis {
            let (name, sc_name) = build::subcommand::names(item.ident.to_string());
            let subcommands = build::subcommand::subcommands(item.clone());
            let sc_enum = build::subcommand::subcommand_enum(
                &name.to_string(),
                subcommands.clone(),
                sc_name.clone(),
            );
            let sc_match = build::subcommand::delegate_match_expr(
                &name.to_string(),
                subcommands.clone(),
                &sc_name,
            );

            // Generate the output token
            let ret =
                return_token::mod_token(name, sc_name, sc_enum, sc_match, item.clone());
            
            return ret;
        } else {
            eprintln!(
                "An item marked with #[crpc_main] must be public and accessible to the binary."
            );
            return item.to_token_stream().into();
        }
    } else {
        eprintln!("An item marked with #[crpc_main] must be a module.");
        return item.to_token_stream().into();
    }
}

#[proc_macro_attribute]
pub fn crpc_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a Rust syntax tree
    let item = parse_macro_input!(item as syn::Item);

    // Check for fn, public and output a type T ( where T: Into<String> )
    // Parse name, args and their comments with "item.sig"
    // Get command comments with "item.attrs" and "item.sig.ident"

    // TODO idea for output Display deriving: https://stackoverflow.com/questions/30353462/how-to-derive-display-for-a-struct-containing-a-string
    // WTF copilot?!
    // making a new type equal to the type of the item - but since this is defined in this module, we are allowed to derive stuff from there - would make #[crpc_param] obsolete

    // Function check
    if let Fn(item) = &item {
        // Public check
        if let syn::Visibility::Public(_) = item.vis {
            // Output type check
            checks::output_check(&item.sig.output); //.clone().into_token_stream().to_string());

            // Input type checks              enable! only for testing
            // if item
            //     .sig
            //     .inputs
            //     .iter()
            //     .any(|arg| checks::input_check(arg).is_err())
            // {
            //     panic!("Your cli cannot take a function because you can´t give code to your cli at runtime.");
            // }

            // Building...
            let name_struct = format_ident!("{}", build::bigger(&item.sig.ident.to_string()));
            let new_function = build::command_args::modify_function(item.clone());

            let item_call = build::command_args::item_call(item.sig.ident.clone());

            let fields = build::command_args::fields(item.clone());

            let ret =
                return_token::fn_token(name_struct, item_call, fields, new_function.clone());
            
            return ret;
        } else {
            eprintln!("An item marked with #[crpc_fn] must be public.");
            return item.to_token_stream().into();
        }
    } else {
        eprintln!("An item marked with #[crpc_fn] must be a function.");
        return item.to_token_stream().into();
    }
}
