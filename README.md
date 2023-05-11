# command-rpc

A command-line rpc written in Rust.



## Contents:

- Installation, Status and Community
- What `crpc` is made for
- How to use `crpc`
- What are you interested in? -New feature suggestion
- Contribution
- Version overview

---

## Installation, Status and Community

Run `cargo add command-rpc --features standard` shell command or insert `command-rpc = { version = "*", features = ["standard"]` in your Cargo.toml.
Just now, in `v0.1.9` this tools stands at the beginning of its development. The a first
working solution will hopefully be published in three weeks.

A bit later, a discord server could be set up if wanted.
Furthermore, a tutorial is planned.

---

## What `crpc` is made for

+ lightweight
+ efficient
+ type-checking
+ easy-to write and beginner friendly
+ export to any language
+ calls from expirienced user possible

Of course, that could be a disadvantage, you should not use `crpc` for big and well-standardized
transfer protocolls.

---

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
4. Give the `main.rs` file acess to this module. Now you can expand, build or compile your program.

As a extra tip, give at first parameter `--help` so you can see the help text of your cli endpoint.

---

## What are you interested in? -New feature suggestion

You like it and would even love it with some new feature? I´d be happy if you suggest it in the issues or contribute a pr directly.

#### Question: In the endpoind you´d use whitespaces between the arguments, are you interested in an option to change that?

#### Question: Are you interested in being able to export macros?

---

## Contribution

To do so, you may write for longer collaboration a message to me (Mail: loos-johannes@gmx.de, Discord: Lá Foûrier),
or make directly a pr. I´d be glad!

---

## Version overview

```
v0.1: Little tests with `proc_macro = true` and import issues.

v0.2: Full documented preview structure, no full implementation.

Future preview:

v0.3: About the begin of June ´23 there will be a beta version.

v1.0: After one month of troubleshooting there will be a first full version release (and also Github Release).

v1.1: Special proc macros - they shall manage communication that is frontend-backend-like.
```