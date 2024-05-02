
mod args;

use args::RustflixArgs;
use clap::Parser;

fn main() {
    for arg in std::env::args() {
        println!("{arg}");
    }
    let args = RustflixArgs::parse();
}