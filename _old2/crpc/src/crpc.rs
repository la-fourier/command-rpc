use std::io::Result as IOResult;

use crate::*;

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
    pub fn name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_fnc(&mut self, fnc: Fnc) {
        self.fncs.push(fnc);
    }

    pub fn add_crpc(&mut self, crpc: Self) {
        self.crpc.push(crpc);
    }

    pub fn generate_main(&self) -> IOResult<()> {
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