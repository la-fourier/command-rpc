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


#[proc_macro_attribute]
pub fn crpc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a Rust syntax tree
    let item = parse_macro_input!(item as syn::Item);

    // Modify the syntax tree as needed
    // For example, you can add additional code or metadata to the item
    // In this simple example, we are just generating a debug print statement

    // Generate the output tokens
    let output = quote! {
        // Add a debug print statement
        #item
        println!("This is a CRPC function");
    };

    // Return the output tokens as a TokenStream
    output.into()
}