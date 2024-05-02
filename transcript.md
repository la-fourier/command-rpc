Trailer (~3min) DT
Hi Leute!
Bist du Rust-Entwickler und sitzst gerade an einem CLI-Tool, das du natürlich mit `clap` schreibst? clap hat zwar einige praktische Funktionen wie die Kommentare für die --help-Flag, aber die Menge an unübersichtlichem Code ist - gewaltig. Und dann muss man doch, fast wie wenn man `clap` nicht zum Parsen verwendet hätte, der Hierarchie folgen.

Warum nutzt man nicht etwas, das im Hintergrund `clap` nutzt, aber nicht `clap`ist?
Bei solchen Fragen bist du hier genau richtg! comm

---

Trailer (~3min) ENG
Hey guys!
Are you a rust-developer stuggling with `clap` and its code-intense way of using structs, enums and derive macros? Pretty annoying.
Same problem for those backend developers looking out for a nice and neaty, scalable and easy-to-use tool for remote procedure calls?

I´m glad to be able to offer help to both of you - why don´t just take the best of clap and leave the rest behind, so use it under the hood of a procedural macro?
This is, what `command_rpc` exactly does - you are still able to write comments right next to your code and they will be used for a `--help` flag. At the same time all the work of parsing the commands is done by ready-to-export functions `command_rpc` provides for you.
So instead of writing all of that CLI structure by hand, it is automatically generated, and here is how to do:

In any module accessible by the binary crate, so the `main.rs` file, create a module you mark with `#[command_rpc_main]`. Inside it you just can write any code you want, but those functions you want to export as subcommands and modules that contain more of them and are hence subcommands themselves you just mark with `#[command_rpc]`. Of course you can write any other stuff inside that, but for reasons of clean code and security I´d recommend not to do so - it will be far more readable.
The only thing left to do is to parse the cli in the main function and then execute the cli call, after some preprocessing maybe.. ..for a more detailed tutorial hit the info card, check out `crates.io` or my github page.

This is not the end, there will be more features, such as the opportunity marking functions as deprecated for a special version of your program without violating downwards compatibility - so version management will be improved.

Last but definitely not least I want to express my big gratitude to backdropbuild for motivating me to a first release and especially Ami Pullack, for suggesting this to me. For those of you who don´t know backdropbuild yet, it´s a free of charge programme for developers of any level - just even for me, a 11th grade high school student working on that in his free time. Within this you work on a project for 4 weeks aiming a release of your project or product; at that point sponsors decide about grants for up to 50 of all projects. These are supposed to motivate the participants on further development of their projects.

This was obviously a DevOps project and it´s left to me to recommend you backdropbuild. It was just such a worthy experience really diving into the project, work hard for it and of course I hope for a grant. Therefore I´d love to get feedback and other feature suggestions from you - maybe a collaboration!

Bye!

---

GPT-Korrektur

Hey everyone!

Are you a Rust developer struggling with clap due to its code-intensive usage of structs, enums, and derive macros? This can be frustrating. The same goes for backend developers seeking a clean, scalable, and easy-to-use tool for remote procedure calls.

I'm delighted to offer assistance to both groups. Why not extract the best features of clap and utilize them within a procedural macro? That's exactly what command_rpc does. You can still write comments next to your code, which will be utilized for a --help flag. Meanwhile, all the parsing work is handled by ready-to-export functions provided by command_rpc.

Instead of manually constructing the CLI structure, it's automatically generated. Here's how:

In any module accessible by the binary crate, such as the main.rs file, create a module marked with #[command_rpc_main]. Inside, you can write any code you want. Mark the functions you want to export as subcommands, as well as modules containing more functions that serve as subcommands themselves, with #[command_rpc]. While you can include other content, I recommend against it for the sake of clean code and security.

All that's left is parsing the CLI in the main function and executing the CLI call, perhaps after some preprocessing. For a more detailed tutorial, check out the info card, crates.io, or my GitHub page.

This is just the beginning; more features are on the way. For instance, you'll soon have the option to mark functions as deprecated for a specific version of your program without compromising backwards compatibility.

Finally, I want to express my sincere gratitude to backdropbuild for motivating me to make the first release, and especially to Ami Pullack for suggesting the idea. For those unfamiliar with backdropbuild, it's a free program available to developers of all skill levels, even someone like me—a high school student in the 11th grade—working on it in my spare time.

Working on this project through backdropbuild was an incredibly rewarding experience. I hope to receive a grant, and I'm eager to receive feedback and feature suggestions from you, and perhaps even collaborate!