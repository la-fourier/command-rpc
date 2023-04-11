# command-rpc
A rpc for command line written in Rust for Python and Rust

## This works as following:

    - mark one module in `lib.rs` with `#[crpc]`
    - you can insert all the stuff you need in that and mark all functions, mods and implementations
    - documentate arguments or tell if chat-gpt should do that
    - include the build-command in your `build.rs`

## That will lead to:

    - a cli that takes with unnamed, required and ordered params
    - optional params with `--var val` syntax
    - a lightweight, helpful documentation
    - all that made with `clap`