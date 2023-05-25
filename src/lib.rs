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
/// # Help! I want to customize stuff manually!
/// ```
/// code
/// ```
/// 

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
//use syn::Item;
use syn::{self, Item::*, parse_macro_input};
use std::fs::File;
use std::io::Write;

use regex::Regex;


#[cfg(feature = "default")]
#[proc_macro_attribute]
pub fn crpc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ts_item = item.clone();
    // Parse the input tokens into a Rust syntax tree
    // Generate the output tokens
    let item = syn::parse_macro_input!(item as syn::Item);
    match item {
        Fn(item) if attr.to_string() == "fn" => { // For fn
            quote! {
                #[crpc_fn]
                #item
            }
            .into()
        },
        Mod(item) if attr.to_string() == "mod" => { // For mod
            quote! {
                #[crpc_mod]
                #item
            }
            .into()
        },
        Struct(item) if attr.to_string() == "struct" => { // For param test
            quote! {
                #[crpc_param]
                #item
            }
            .into()
        },
        _ => {
            eprint!("Error in {:?}: crpc can only be used on fn, mod and struct", ts_item.to_string());
            ts_item
        },
    }
}


/// This is where the magic happens.
#[cfg(feature = "default")]
#[proc_macro_attribute]
pub fn crpc_fn(_attr: TokenStream, //This is the bracket attr!
               item: TokenStream //An other style!
            ) -> TokenStream {
    println!("_attr is: {}", _attr.to_string());
    println!("{}", item.to_string());

    // Parse the input tokens into a Rust syntax tree
    let item = parse_macro_input!(item as syn::Item);

    // Check for fn, public and output a type T ( where T: Into<String> )
    // Parse name, args and their comments with "item.sig"
    // Get command comments with "item.attrs" and "item.sig.ident"

    // Return something like this:
    // ```
    // pub struct Mycommand {
    //    arg1: T,
    //   arg2: S,
    // }
    // 
    // pub impl FromStr for Mycommand {
    //   fn from_str(s: &str) -> Result<Self, Self::Err> {
    //       todo!()
    //   }
    // }
    // 
    // pub impl Mycommand {
    //  pub fn run(&self) -> Result<String> {
    //    todo!()
    //    // code of the item
    //  }
    // }


    // TODO how to make a cmd and subcommands for cmd? mb with a "core" method for 
    

    // TODO idea for output Display deriving: https://stackoverflow.com/questions/30353462/how-to-derive-display-for-a-struct-containing-a-string
    // WTF copilot?!
    // making a new type equal to the type of the item - but since this is defined in this module, we are allowed to derive stuff from there - would make #[crpc_param] obsolete

    if let Fn(item) = &item {
        if let syn::Visibility::Public(_) = item.vis {
            println!("Public function");
        }
        else {
            eprintln!("An item marked with #[crpc_fn] must be public.");
        } 
    }
    else {
        eprintln!("An item marked with #[crpc_fn] must be a function.");
    }

    let code = item.to_token_stream().to_string();

    // Modify the syntax tree as needed
    // For example, you can add additional code or metadata to the item
    // In this simple example, we are just generating a debug print statement

    if let Ok(mut file) = File::create("foo.txt") {
        let res = file.write_all(b"Hello, world!");
        if let Ok(re) = res {
            println!("Ok");
            println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
        } else {
            println!("Not ok");
        }
    }

    // Generate the output tokens
    let output = quote! {
        // Add a debug print statement
        fn pre() {
            let x = 10;
        }
        #item
    };

    // Return the output tokens as a TokenStream
    output.into()
}


/// This is where the magic happens.
#[cfg(feature = "default")]
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
    }.into()
}


/// This is where the magic happens.
#[cfg(feature = "default")]
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
            }
            else {
                panic!("No struct name found")
            }
        } else {
           panic!("No struct found")
        }
    }.as_str();

    quote! {
        // Add a debug print statement
        #[derive(Debug, Display, From, Into)]
        #item

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }
    }.into()
}


#[cfg(features = "callback")]
#[proc_macro]
pub fn callback(input: TokenStream) -> TokenStream {
    let mut sc = parse_macro_input!(input as syn::Item)
        .into_token_stream()
        .to_string();
    for s in ["::", "(", ")"] {
        sc = sc.replace(s, " ");
    }
    sc.to_token_stream().to_token_stream();

    quote!(
        let handle = std::thread::spawn(|| {
            // Run the shell command here
            let output = std::process::Command::new( #sc )
                .output()
                .expect("Failed to execute command");
            println!("{}", std::string::String::from_utf8_lossy(&output.stdout));
        });
        handle.join().unwrap();
    ).into()
}