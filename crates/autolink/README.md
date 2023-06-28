# markdown-it-autolink.rs

[<img alt="crates.io" src="https://img.shields.io/crates/v/markdown-it-autolink.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/markdown-it-autolink)

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin that implements the Github Flavoured Markdown [autolink extension](https://github.github.com/gfm/#autolinks-extension-).

## Usage

```rust
let md = &mut markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(md);
markdown_it_autolink::add(md);
md.parse("www.example.com").render();
// <p><a href="http://www.example.com">www.example.com</a></p>
```
