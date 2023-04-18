use std::io::Result as IOResult;
use std::str::FromStr;
use core::convert::Infallible;

use regex::Regex;

pub enum ArgType {
    Integer,
    FloatingNumber,
    Bool,
    Char,
    String,
}

#[derive(Default)]
pub struct Fnc {
    doc: String,
    name: String,
    param: Vec<(String, ArgType)>,
    out: String,
    mod_path: String,
}

impl FromStr for Fnc {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n");
        lines.for_each(move |mut line| {
            if let Some(capt) = Regex::new(r"\s*(\w)").unwrap().captures(line) {
                let lline = capt.get(2).unwrap();
            }

            // line = line.replace(" ", "");
                    //    .replace("   ", "")
                    //    .replace("pub ", "").as_str();
            if line.starts_with("///") || line.starts_with("//!") {
                // &fnc.doc.push_str(line);
            }
            if line.contains("fn") {
                let mut bracket_it = line.split("(").into_iter();
                let st = bracket_it.next().unwrap_or_default();
                // fnc.name = st.split(" ").into_iter().collect();
                let nd = bracket_it.next().unwrap_or_default();
            }
        });

        let pattern = Regex::new(r"fn\s+(\w+)\s*\((.*)\)\s*->\s*(\w+)\s*{").unwrap();
                                        //   fn    nam      (par)     ->

        // Input line to check
        let line = "fn add(x: i32, y: i32) -> i32 {";
    
        // Check if the line matches the function head pattern
        if let Some(captures) = pattern.captures(line) {
            // Extract function name and parameters from captures
            let function_name = captures.get(1).map_or("", |m| m.as_str());
            let parameters = captures.get(2).map_or("", |m| m.as_str());
            
    
            println!("Function Name: {}", function_name);
            println!("Parameters: {}", parameters);
        }
        Ok(Self::default())
    }
}


impl Fnc {
    pub fn new(doc: String, name: String, param: Vec<(String, ArgType)>, out: String, mod_path: String) -> Self {
        Self {
            doc: doc,
            name: name,
            param: param,
            out: out,
            mod_path: mod_path,
        }
    }

    pub fn gen_parse(&self) -> IOResult<()> {
        todo!()
    }

    pub fn gen_command(&self) -> IOResult<()> {
        todo!()
    }

    
    pub fn expand_methods(&mut self) -> IOResult<()> {
        todo!() // made for changing the selfs to other stuff
    }

    /* Check:
    - params for primitive or FromString implemented
    - output has to implement Display trait for transfer
    - everything marked with `#[crpc]` public
    - docs? else check if gpt docs are wanted
     */
}