use std::{path::PathBuf, string::ParseError, vec};

pub fn builder(missing_docs: bool) -> std::io::Result<()> {
    let mut curdir = std::env::current_dir()?;
    curdir.push("src/crpc.rs");
    let mut crpc = Crpc::default();
    for fnc in fn_iterator(curdir)? {
        // mk Vec<Fnc>
        // expand_methods
        // check stuff
    }
    Ok(())

    // Erase `#[crpc]` and name crpc obj
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
    fn name(mut self: Self, name: String) {
        self.name = name;
    }

    fn add_fnc(mut self: Self, fnc: Fnc) {
        self.fncs.push(fnc);
    }

    fn add_crpc(mut self: Self, crpc: Self) {
        self.crpc.push(crpc);
    }

    fn generate_main(self: Self) {
        for fnc in self.fncs {
            fnc.gen_command();
            fnc.gen_parse();
        }
        for crpc in self.crpc {
            crpc.generate_main();
        }
    }
    
}

#[derive(Default)]
pub struct Fnc {
    doc: String,
    name: String,
    param: Vec<String>,
    out: String,
    mod_path: String,
}

impl std::str::FromStr for Fnc {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fnc = Self::default();
        let lines = s.split("\n");
        lines.for_each(move |mut line| {
            line = line.replace(" ", "")
                       .replace("   ", "").as_str();
            if line.starts_with("///") || line.starts_with("//!") {
                fnc.doc.push_str(line);
            }
            if line.contains("fn") {
                let bracket_it = line.split("(").into_iter();
                let st = bracket_it.next().unwrap_or_default();
                fnc.name = st.split(" ").into_iter().collect::<Iterator<std::io::Split<&str>>>().last();
                let nd = bracket_it.next().unwrap_or_default();
            }
        });
        Ok(fnc)
    }
}

impl Fnc {
    pub fn new(doc: String, name: String, param: Vec<String>, out: String, mod_path: String) -> Self {
        Self {
            doc: doc,
            name: name,
            param: param,
            out: out,
            mod_path: mod_path,
        }
    }

    pub fn gen_parse(self: Self) -> std::io::Result<String> {
        todo!()
    }

    pub fn gen_command(self: Self) -> std::io::Result<String> {
        todo!()
    }

    
    pub fn expand_methods(self: Self) -> std::io::Result<()> {
        todo!() // made for changing the selfs to other stuff
    }

    /* Check:
    - params for primitive or FromString implemented
    - output has to implement Display trait for transfer
    - everything marked with `#[crpc]` public
    - docs? else check if gpt docs are wanted
     */
}

pub fn fn_iterator(path: PathBuf) -> std::io::Result<Vec<String>> {
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