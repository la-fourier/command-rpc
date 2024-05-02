#[crpc]
pub mod rustflixArgs {
    #[crpc]
    pub mod userCommand {
        #[crpc]
        pub fn createUser(name: String, emil: String) {
            todo!()
        }

        #[crpc]
        pub fn updateUser(name: String, email: String) {
            todo!()
        }

        #[crpc]
        pub fn deleteUser(name: String, email: String) {
            todo!()
        }

        #[crpc]
        pub fn readUser(name: String, email: String) {
            todo!()
        }
    }

    #[crpc]
    pub fn videoCommand() {
        todo!()
    }

    #[crpc]
    pub fn ViewCommand() {
        todo!()
    }
}

fn main() {
    parse!(std::sys::args());
}

// Callback
fn from_other() {
    callback!(my_cli_backend::...::greet("John"));
}