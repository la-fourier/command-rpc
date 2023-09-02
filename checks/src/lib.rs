use quote::{quote, ToTokens};
//use syn::Item;
use syn::{self, Item::*, parse_macro_input, parse_quote};
use std::fs::File;
use std::io::Write;

use regex::Regex;


pub mod type_checks {
    use syn::{ReturnType, FnArg, PatType};


    pub fn output_check(rt: &ReturnType) {
        if let syn::ReturnType::Type(_, boxed) = rt {            
            if let syn::Type::Path(path) = *boxed.clone() {
                if let Some(ident) = path.path.get_ident() {
                    if ident.to_string() == "String" {
                        println!("Your cli returns a String. This is ok but might cause speed issues.");
                    }
                }
            if match &*boxed {
                syn::Type::Path(type_path) => {
                    if let Some(segment) = type_path.path.segments.last() {
                        segment.ident.to_string() == String::from("std::str::ToStr") // TODO fix! Or it shall be something convertable to a str
                    } else {
                        false
                    }
                },
                _ => false,
                } {
                    println!("Your cli returns a type that does not implement ToStr but the crpc macro needs it to parse the arguments. Please implement ToStr for your type.");
                };
            }
        }
    }

    pub fn input_check(arg: &FnArg) -> bool {
        if match &*pat_type.ty {
            syn::Type::Path(types) => {
                if let Some(segment) = types.path.segments.last() {
                    segment.ident.to_string() != String::from("std::str::FromStr")
                    } else {
                        true
                    }
                },
            syn::Type::Slice(types) => {
                if let syn::Type::Path(type_path) = *types.clone().elem {
                    if let Some(segment) = type_path.path.segments.last() {
                        segment.ident.to_string() != String::from("std::str::FromStr")
                    } else {
                        true
                    }
                } else {
                    true
                }
            },
            syn::Type::Tuple(types) => {
                let mut overall = true;
                types.elems.iter().for_each(|elem| {
                    if ! input_check(&FnArg::Typed(PatType{
                        attrs: types.attrs.clone(),
                        pat: types.pat.clone(),
                        colon_token: types.paren_token.clone(),
                        ty: Box(elem.clone()),
                    })) {
                        overall = false;
                        println!("Your cli takes a tuple with a type that does not implement FromStr but the crpc macro needs it to parse the arguments. Please implement FromStr for your type.");
                    }
                });
                overall
            },
            syn::Type::BareFn(_) => {
                println!("Your cli cannot take a function because you can´t give code to your cli at runtime.");
                false
            },
            syn::Type::Group(_) => {
                println!("Your cli cannot take a group because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            },
            syn::Type::Paren(_) => {
                println!("Your cli cannot take a paren because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            },
            syn::Type::Reference(_) => {
                println!("Your cli cannot take a reference because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            },
            syn::Type::TraitObject(_) => {
                println!("Your cli cannot take a trait object because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            },
            syn::Type::ImplTrait(_) => {
                println!("Your cli cannot take a impl trait because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            },
            syn::Type::Infer(_) => {
                println!("Your cli cannot take a infer because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            },
            syn::Type::Macro(_) => {
                println!("Your cli cannot take a macro because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            },
            syn::Type::Never(_) => {
                println!("Your cli cannot take a never because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            },
            syn::Type::Array(_) => false,
            _ => {
                println!("This is not finished yet, please be careful with your input types. ");
                false
            },
        } {
            // TODO feature: own default parser
            println!("Your cli takes a type that does not implement FromStr but the crpc macro needs it to parse the arguments. Please implement FromStr for your type.");
        };
    }
}