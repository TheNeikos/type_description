# Generating Markdown

Depending on your case you can interact with `TypeDescription`s in multiple ways.

If you are given a JSON formatted type description file, then you should use the CLI for further processing.
(This may be the case if you are using someone else's project).

If you have some Rust types and want to generate markdown directly from their
`TypeDescription` then you might be best served with the library.

## Using the CLI

The preferred way of interacting with the CLI is to use the `type_description` binary.

It can be invoked in the following ways:

- With the nix package manager (flakes enabled): `nix run github:TheNeikos/type_description`
- With cargo: `cargo install type_description`
- Directly from source: `git clone https://github.com/TheNeikos/type_description && cd type_description && cargo run -F bin`

Either way you have installed it. You can now generate markdown from it by sending it to standard input.

E.g.:

- Using nix: `cat desc.rtd | nix run github:TheNeikos/type_description`
- After installing it: `curl example.com/some_type_description | type_description`
- Building from source: `curl example.com/some_type_description | cargo run -F bin`


## Using the library

Rendering a `TypeDescription` to markdown can be done with the
[`type_description::render::render_to_markdown`](https://docs.rs/type_description/latest/type_description/render/fn.render_to_markdown.html)
method.

```rust
# extern crate type_description;
# fn main() {
use type_description::AsTypeDescription;

let ty_desc = std::collections::HashMap::<String, f64>::as_type_description();

let markdown: String = type_description::render::render_to_markdown(&ty_desc).expect("Could not render to markdown");

println!("{}", markdown);
# }
```

You can then use a markdown to HTML processor to generate HTML, or maybe print
it to the terminal with the [`termimad`](https://docs.rs/termimad) crate.
