pub fn testt() {
    println("sdflkj");
}

use quote::quote; // using works for child mods too!

pub mod crpc {
    pub fn test() {
        todo!()
    }

    pub mod inner_sc {
        pub fn ttt() {
            let code = quote ! {
                let x = "pop";
            };
        }
    }
}