use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};

use syn::{
    Item::{Fn, Mod, Struct},
    __private::Span,
    parse_macro_input,
    token::Brace,
};

mod build;
mod build_command;
mod build_nested;
mod checks;
mod return_token;

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
/// marked as not working false
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

/// This macro can be used to parse the input and call the right function without
/// having to create a pipe. This shortens the code and makes it easier to understand.
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

// lazy variant! Just mark the main module with [crpc_main], then everything gets expaneded recursively! -> less control..

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
    if let Mod(mut item) = item {
        if let syn::Visibility::Public(_) = item.vis {
            let (name, sc_name) = build_nested::names(item.ident.to_string());
            let subcommands = build_nested::subcommands_with_help(item.clone());
            let sc_enum = build_nested::subcommand_enum(
                &name.to_string()[..name.to_string().len()].to_string(),
                subcommands.clone(),
                sc_name.clone(),
            );
            let sc_match = build_nested::delegate_match_expr(subcommands.clone(), &sc_name);

            let (_, mut _body) = item.content.clone().unwrap();
            _body.insert(0, build::item_use());
            item.content = Some((Brace(Span::call_site()), _body));

            // Generate the output token
            return_token::main_token(name, sc_name, sc_enum, sc_match, item.clone())
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

/// This attribute can be used to mark modules that should be available in the cli.
/// This works like a subcommand with nested commands; the module name is used as
/// the subcommand name.
#[proc_macro_attribute]
pub fn crpc_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);
    if let Mod(mut item) = item {
        if let syn::Visibility::Public(_) = item.vis {
            let (name, sc_name) = build_nested::names(item.ident.to_string());
            let subcommands = build_nested::subcommands_with_help(item.clone());
            let sc_enum = build_nested::subcommand_enum(
                &name.to_string()[..name.to_string().len()].to_string(),
                subcommands.clone(),
                sc_name.clone(),
            );
            let sc_match = build_nested::delegate_match_expr(subcommands.clone(), &sc_name);

            let (_, mut _body) = item.content.clone().unwrap();
            _body.insert(0, build::item_use());
            item.content = Some((Brace(Span::call_site()), _body));

            // Generate the output token
            return_token::mod_token(name, sc_name, sc_enum, sc_match, item.clone())
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

/// This attribute can be used to mark functions that should be available in the cli.
/// This works lika a subcommand without nested commands; function parameters are used
/// as arguments for the cli. The expansion consists of a struct that holds the arguments
/// and a function that calls the original function with the arguments.
#[proc_macro_attribute]
pub fn crpc_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);

    // Function check
    if let Fn(item) = &item {
        // Public check
        if let syn::Visibility::Public(_) = item.vis {
            // Output type check
            checks::output_check(&item.sig.output); //.clone().into_token_stream().to_string());

            // Input type checks              TODO
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
            let new_function = build_command::to_impl_item(item.clone());

            let fields = build_command::fields(item.clone());
            let struct_item = build_command::to_struct(name_struct.clone(), fields);

            return_token::fn_token(name_struct, struct_item, new_function)
        } else {
            eprintln!("An item marked with #[crpc_fn] must be public.");
            return item.to_token_stream().into();
        }
    } else {
        eprintln!("An item marked with #[crpc_fn] must be a function.");
        return item.to_token_stream().into();
    }
}
