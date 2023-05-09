// pub mod crpc {
//     pub fn test() {
//         todo!()
//     }

//     pub mod inner_sc {
//         pub fn ttt() {
//             print!("ttt");
//         }
//     }
// }

#[proc_macro_attribute]
pub fn crpc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ts_item = item.clone();
    // Parse the input tokens into a Rust syntax tree
    let item = syn::parse_macro_input!(item as syn::Item);
    match item {
        Fn(item) => { // For fn
            todo!()
        },
        Mod(item) => { // For mod
            todo!()
        },
        Struct(item) => { // For param test
            todo!()
        },
        _ => {
            eprint!("Error in {:?}: crpc can only be used on fn, mod and struct", ts_item.to_string());
        },
    }

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
#[proc_macro_attribute]
pub fn crpc_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("_attr is: {}", _attr.to_string());
    println!("{}", item.to_string());

    // Parse the input tokens into a Rust syntax tree
    let item = parse_macro_input!(item as syn::Item);

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

