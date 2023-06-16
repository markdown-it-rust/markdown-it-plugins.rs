# markdown-it-footnote.rs

[<img alt="crates.io" src="https://img.shields.io/crates/v/markdown-it-footnote.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/markdown-it-footnote)

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin to process footnotes.

It is based on the
`pandoc definition <http://johnmacfarlane.net/pandoc/README.html#footnotes>`__:

```md
Normal footnote:

Here is a footnote reference,[^1] and another.[^longnote]

[^1]: Here is the footnote.

[^longnote]: Here's one with multiple blocks.

    Subsequent paragraphs are indented to show that they
belong to the previous footnote.
```

## Usage

```rust
let parser = &mut markdown_it::MarkdownIt::new();
markdown_it_footnote::add(parser);
// TODO
```
