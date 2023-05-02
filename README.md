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

Of course, that could be a disadvantage, you should not use `crpc` for big and well-standardized
transfer protocolls.



## How to use `crpc`

A tutorial will be coming and linked here then.

1. Add `command-rpc` as dependency.
2. Write a `crpc` module that has the `#[crpc_mod]` attribute. The functions (that need to be public!)
 in it you annotate with `#[crpc_fn]` is going to be nested as command, and (public) modules with
 `#[crpc_mod]` included as subcommand, its inner (public) functions will be included too.
3. To give the subcommands generated with `#[crpc_mod]` functionality, implement a (public) function in
  the module named after the module.
4. Mark ALL Structs you need for parameters of the functions you want to give to the commands
  with `#[crpc_param]`.
4. Give the `main.rs` file acess to this module. Now 



## What are you interested in? -New feature suggestion

You like

# old
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