# markdown-it-footnote.rs

[<img alt="crates.io" src="https://img.shields.io/crates/v/markdown-it-footnote.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/markdown-it-footnote)

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin to process footnotes.

It is based on the
`pandoc definition <http://johnmacfarlane.net/pandoc/README.html#footnotes>`__:

```md
Normal footnote:

Here is a footnote reference,[^1] and another.[^longnote]

Here is an inline note.^[my note is here!]

[^1]: Here is the footnote.

[^longnote]: Here's one with multiple blocks.

    Subsequent paragraphs are indented to show that they
belong to the previous footnote.
```

## Usage

To load the full plugin:

```rust
let parser = &mut markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(parser);

markdown_it_footnote::add(parser);

let ast  = parser.parse("Example^[my note]");
let html = ast.render();
```

Alternatively, you can load the separate components:

```rust
let parser = &mut markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(parser);

markdown_it_footnote::definitions::add(md);
markdown_it_footnote::references::add(md);
markdown_it_footnote::inline::add(md);
markdown_it_footnote::collect::add(md);
markdown_it_footnote::back_refs::add(md);
```

Which have the following roles:

- `definitions`: parse footnote definitions, e.g. `[^1]: foo`
- `references`: parse footnote references, e.g. `[^1]`
- `inline`: parse inline footnotes, e.g. `^[foo]`
- `collect`: collect footnote definitions (removing duplicate/unreferenced ones) and move them to be the last child of the root node.
- `back_refs`: add anchor(s) to footnote definitions, with links back to the reference(s)
