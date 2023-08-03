# markdown-it-gfm

[<img alt="crates.io" src="https://img.shields.io/crates/v/markdown-it-gfm.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/markdown-it-gfm)

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin to implement [Github Flavoured Markdown](https://github.github.com/gfm).

## Usage

To load the plugin:

```rust
let parser = &mut markdown_it::MarkdownIt::new();
markdown_it_gfm::add(parser);

let root = parser.parse("https://github.github.com/gfm");
assert_eq!(root.render(), "<p><a href=\"https://github.github.com/gfm\">https://github.github.com/gfm</a></p>\n");
```
