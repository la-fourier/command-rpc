# command-rpc
A rpc for command line written in Rust for Python and Rust

## This works as following:

    - mark up your core command with `#[crpc(core)]`
    - all others with `#[crpc(sub)]`
    - give them a structure by modulizing
    - a built-in with subcommands is named `core()` in its module
    - The main module has to be named `crpc`
    - documentate arguments or tell if chat-gpt should do that
    - include the build-command in your `build.rs`

## That will lead to:

    - a cli that takes with unnamed, required and ordered params
    - optional params with `--var val` syntax
    - a lightweight, helpful documentation
    - all that made with `clap`