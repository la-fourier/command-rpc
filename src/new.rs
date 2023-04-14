use std::{path::PathBuf, vec, convert::Infallible};
use regex::{Regex};
use std::str::FromStr;
use std::io::Result as IOResult;
use std::hash::Hash;

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
    let mut crpc = Crpc::default();
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

pub struct Crpc {
    name: String,
    fncs: Vec<Fnc>,
    crpc: Vec<Crpc>,
}

impl core::default::Default for Crpc {
    fn default() -> Self {
        Self { name: String::from("Default"), fncs: vec![], crpc: vec![] }
    }
}

impl Crpc {
    fn name(&mut self, name: String) {
        self.name = name;
    }

    fn add_fnc(&mut self, fnc: Fnc) {
        self.fncs.push(fnc);
    }

    fn add_crpc(&mut self, crpc: Self) {
        self.crpc.push(crpc);
    }

    fn generate_main(&self) -> IOResult<()> {
        for fnc in &self.fncs {
            fnc.gen_command();
            fnc.gen_parse();
        }
        for crpc in &self.crpc {
            crpc.generate_main();
        }
        Ok(())
    }
    
}

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

pub fn fn_iterator(path: PathBuf) -> IOResult<Vec<String>> {
    todo!()
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