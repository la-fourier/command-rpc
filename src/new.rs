pub fn fn_iterator(path: PathBuf) -> Result<impl Iterator<Item = PathBuf>, std::io::Error> {
    todo!()
}

pub fn expand_methods(iterator: impl Iterator<Item = PathBuf>) -> Result<impl Iterator<Item = PathBuf>, std::io::Error> {
    todo!() // made for changing the selfs to other stuff
}

pub fn erase_crpc_marks(iterator: impl Iterator<Item = PathBuf>) {
    todo!()
}

pub mod check_for_correctness {
    todo!();
    /* Check:
    - params for primitive or FromString implemented
    - output has to implement Display trait for transfer
    - everything marked with `#[crpc]` public
    - docs? else check if gpt docs are wanted
     */
}

pub enum CRPC {
    FNC(Fnc),
    CRPC(CRPC),
}

pub struct Fnc {
    doc: str,
    name: str,
    param: Vec<str>,
    out: str,
    path: str
}

impl Fnc {
    pub fn new(doc: str, name: str, param: Vec<str>, out: str, path: str) -> Self {
        todo!()
    }

    pub fn gen_gather(self: Self) -> std::io::Result<()> {
        todo!()
    }
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