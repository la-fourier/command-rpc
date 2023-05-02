// extern crate crpc;
// extern crate crpc_mark_macro;
// extern crate traverse;

// pub use crpc;
// pub use crpc_mark_macro;
// pub use traverse;
// 
// pub fn exp() {
//     todo!()
// }



//extern crate proc_macro;
// extern crate quote;
// extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
//use syn::Item;
use syn::parse_macro_input;
use std::fs::File;
use std::io::Write;


/// This is where the magic happens.
#[proc_macro_attribute]
pub fn crpc_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a Rust syntax tree
    let item = parse_macro_input!(item as syn::Item);

    // Modify the syntax tree as needed
    // For example, you can add additional code or metadata to the item
    // In this simple example, we are just generating a debug print statement

    eprintln!("Expansion sucessfull!");
    eprintln!("Expansion sucessfull!");
    eprintln!("Expansion sucessfull!");
    eprintln!("Expansion sucessfull!");
    eprintln!("Expansion sucessfull!");
    eprintln!("Expansion sucessfull!");

    if let Ok(mut file) = File::create("foo.txt") {
        let res = file.write_all(b"Hello, world!");
        if let Ok(re) = res {
            println!("Ok");
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
#[proc_macro_attribute]
pub fn crpc_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a Rust syntax tree
    let item = parse_macro_input!(item as syn::Item);

    // Modify the syntax tree as needed
    // For example, you can add additional code or metadata to the item
    // In this simple example, we are just generating a debug print statement

    eprintln!("Expansion sucessfull!");
    eprintln!("Expansion sucessfull!");
    eprintln!("Expansion sucessfull!");
    eprintln!("Expansion sucessfull!");
    eprintln!("Expansion sucessfull!");
    eprintln!("Expansion sucessfull!");

    if let Ok(mut file) = File::create("foo.txt") {
        let res = file.write_all(b"Hello, world!");
        if let Ok(re) = res {
            println!("Ok");
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