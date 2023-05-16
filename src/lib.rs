/// Hey! Nice you like to write with the `cprc` module. It refers to clap, thank you for your great work with that!
/// 
/// # Quick setup
/// ```
/// code
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
pub fn crpc_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("_attr is: {}", _attr.to_string());
    println!("{}", item.to_string());

    // Parse the input tokens into a Rust syntax tree
    let item = parse_macro_input!(item as syn::Item);
    let code = item.to_token_stream().to_string();

    let name = code.split("fn").collect::<Vec<&str>>()[1].split("(").collect::<Vec<&str>>()[0].trim();

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
    let item = parse_macro_input!(item as syn::Item);
    quote! {
        // Add a debug print statement
        #[derive(Debug, Display, From, Into)]
        #item
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