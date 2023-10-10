use quote::{quote, ToTokens};
use std::fs::File;
use std::io::Write;
use syn::{self, parse_macro_input, parse_quote, FnArg, Item::*, PatType, ReturnType};

use regex::Regex;

pub mod checks;
pub use checks::*;

pub mod build;
pub use build::*;
