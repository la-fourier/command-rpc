/// Hey! Nice you like to write with the `cprc` module. It refers to clap, thank you for your great work with that!
///
/// # Quick setup
/// ```
/// pub mod my_cli_backend {
///    pub fn greet(name: str // The name of the person you want to greet.
///                 ) {
///       println!("Hello, {}!", name);
///   }
///
///  pub mod my_cli_backend_sub {
///    pub fn friendly_greet(name: str // The name of the person you want to greet.
///                          adjective: str // the adjective you want to use in the greeting.
///                         ) -> Option<String> {
///      println!("Hello, {}! You are {}!", name, adjective);
///     Some("You are friendly!".to_string())
///   }
/// }
/// ```
///
/// # Prettier requests to endpoint
/// 
/// To make call to your endpoint in your Rust programs nicer, you can insert this snippet
/// that defines a declarative macro (this is why it can´t be just exported from this crate).
/// Calls in your program would look like this then:
/// ```	
/// callback!(my_cli_backend::greet("John"));
/// ```
/// 
/// ```
/// macro_rules! callback {
/// ($inp:expr) => {{
///     let mut cmd = $inp.to_string();
///     cmd = cmd.replace(";", "").replace(" ", "").replace("\n", "").replace("(", "").replace(")", "").replace("::", " ").replace(",", " ");
///     std::thread::spawn(move || {
///         let output = std::process::Command::new(cmd)
///             .output()
///             .expect("Failed to execute command");
///         println!("{}", std::string::String::from_utf8_lossy(&output.stdout));
///     });
/// }};
/// }
/// ```
/// 
/// 


use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::fs::File;
use std::io::Write;
use syn::{self, parse_macro_input, parse_quote, Item::*};

use regex::Regex;

use checks::*;

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
pub fn crpc_fn(
    _attr: TokenStream, //This is the bracket attr!
    item: TokenStream,  //An other style!
) -> TokenStream {
    use quote::__private::ext::RepToTokensExt;
    use syn::PathSegment;
    // Parse the input tokens into a Rust syntax tree
    let item = parse_macro_input!(item as syn::Item);

    // Check for fn, public and output a type T ( where T: Into<String> )
    // Parse name, args and their comments with "item.sig"
    // Get command comments with "item.attrs" and "item.sig.ident"
    // TODO how to make a cmd and subcommands for cmd? mb with a "core" method for -> no, just a function, has no crashes bco diffrent types

    // TODO idea for output Display deriving: https://stackoverflow.com/questions/30353462/how-to-derive-display-for-a-struct-containing-a-string
    // WTF copilot?!
    // making a new type equal to the type of the item - but since this is defined in this module, we are allowed to derive stuff from there - would make #[crpc_param] obsolete

    // Function check
    if let Fn(item) = &item {
        // Public check
        if let syn::Visibility::Public(_) = item.vis {
            item.sig.inputs.iter().for_each(|arg| {
                println!("arg: {}", arg.to_token_stream().to_string());
            });

            // Output type check
            checks::type_checks::output_check(&item.sig.output); //.clone().into_token_stream().to_string());

            // Input type checks
            item.sig.inputs.iter().for_each(|arg| {
                checks::type_checks::input_check(arg);
            });

            // Metastruct name
            let pre_name = item.sig.ident.to_string();
            let name = pre_name
                .as_str()
                .get(0..1)
                .unwrap()
                .to_uppercase()
                .to_string()
                + pre_name.as_str().get(1..).unwrap();

            // Parse function
            let code = item.to_token_stream().to_string();

            // Generate the output tokens
            return quote! {
                pub struct #name {
                        #item.sig.inputs
                    }

                    pub impl FromStr for #name {
                      fn from_str(s: &str) -> Result<Self, Self::Err> {
                        let arg = s.split(" --").collect().for_each();
                          Ok(Self {
                                arg1: todo!(),
                                arg2: todo!(),
                          })
                      }
                    }

                    pub impl #name {
                     pub fn run(&self) -> Result<String> {
                       todo!()
                       // code of the item
                     }
                    }
                #item
                #code
            }
            .to_token_stream()
            .into();
        } else {
            eprintln!("An item marked with #[crpc_fn] must be public.");
        }
    } else {
        eprintln!("An item marked with #[crpc_fn] must be a function.");
    }

    item.to_token_stream().into()
}

#[proc_macro_attribute]
pub fn crpc_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a Rust syntax tree
    let item = parse_macro_input!(item as syn::Item);

    quote! {
        // Add a debug print statement
        fn pre() {
            let x = 10;
        }
        #item
    }
    .into()
}

#[proc_macro_attribute]
pub fn crpc_param(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a Rust syntax tree
    let item_str = item.to_string();
    let item = parse_macro_input!(item as syn::Item);

    let pattern = r#"struct\s+(\w+)\s*\{\s*([^)]*)\s*\}"#;
    let regex = Regex::new(pattern).unwrap();

    let struct_name = {
        if let Some(captures) = regex.captures(&item_str) {
            // Extract struct name
            if let Some(struct_name) = captures.get(1) {
                struct_name
            } else {
                panic!("No struct name found")
            }
        } else {
            panic!("No struct found")
        }
    }
    .as_str();

    quote! {
        // Add a debug print statement
        #[derive(Debug, Display, From, Into)]
        #item

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }
    }
    .into()
}