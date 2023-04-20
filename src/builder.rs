use std::{path::PathBuf};
use std::io::Result as IOResult;
use std::hash::Hash;
use std::str::FromStr;

use crate::*;



extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;


#[derive(Hash)]
pub struct Settings {
    gen_missed_docs: bool,
    separator: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gen_missed_docs: true,
            separator: " ".to_string(),
        }
    }
}

/// This is where the magic happens. Call this in your `build.rs` file.
/// 
/// The canonical thing for that is
/// ```
/// use command_rpc::{Settings, builder};
/// builder(Settings::default());
/// ```
pub fn builder(settings: Settings) -> IOResult<()> {
    let mut curdir = std::env::current_dir()?;
    curdir.push("src/crpc.rs");
    let mut crpc = crate::Crpc::default();
    for fnc_str in fn_iterator(curdir)? {
        // mk Vec<Fnc>
        let fnc = Fnc::from_str(fnc_str.as_str()).unwrap();
        crpc.add_fnc(fnc);
        // expand_methods
        // check stuff
    }

    // Erase `#[crpc]` and name crpc obj

    // Mk docs if wanted
    if settings.gen_missed_docs {
        todo!()
    }

    // Write everything in `main.rs`

    Ok(())
}
/**7
#[cfg(test)]
fn regex_test() {
    let pattern = Regex::new(r"fn\s+(\w+)\s*\((.*)\)\s*->\s*\w+\s*{").unwrap();
    let line = "fn add(x: i32, y: i32) -> i32 {";
    if let Some(captures) = pattern.captures(line) {
        for i in 1.. {
            let el = captures.get(i);
            if el == None {
                break;
            }
            println!("{}", el.map_or("", |m| m.as_str()));
        }
    }
} */

pub fn fn_iterator(path: PathBuf) -> IOResult<Vec<String>> {
    let mut out: Vec<String> = vec![];
    if let Ok(entries) = std::fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    println!("File: {:?}", entry_path);
                } else if entry_path.is_dir() {
                    let new_path = std::env::join_paths(
                        [path.clone(), entry_path]).unwrap().into();
                    let inner_file = fn_iterator(new_path);
                    out.append(&mut inner_file.unwrap());
                }
            }
        }
    }

    Ok(vec![])
}

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

/*
Example:

#[crpc]
mod bsp {

    #[crpc]
    fn bsp {
        todo!()
    }

    #[crpc]
    mod t2 {
        fn t2 {

        }
    }
    
}
*/