# command-rpc

A command-line rpc written in Rust.



## Contents:

- Installation, Status and Community
    - What `crpc` is made for
    - How to use `crpc`
    - What are you interested in? -New feature suggestion
    - Contribution
- Changelog



## Installation, Status and Community

Run `cargo add command-rpc` shell command or insert `command-rpc = "*"` in your Cargo.toml.
Just now, in `v0.1.9` this tools stands at the beginning of its development. The a first
working solution will hopefully be published in three weeks.

A bit later, a discord server could be set up if wanted.
Furthermore, a tutorial is planned.



## What `crpc`is made for

    + lightweight
    + efficient
    + type-checking
    + easy-to write and beginner friendly
    + export to any language
    + calls from expirienced user possible

Of course, that could be a disadvantage, you should not use `crpc` for big and well-standardized transfer protocolls.



## How to use `crpc`

1. Add `command-rpc` as dependency.
2. 



## What are you interested in? -New feature suggestion

You like
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