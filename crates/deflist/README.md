# markdown-it-deflist.rs

[<img alt="crates.io" src="https://img.shields.io/crates/v/markdown-it-deflist.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/markdown-it-deflist)

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin to process definition lists.

It is based on the [pandoc definition](http://johnmacfarlane.net/pandoc/README.html#definition-lists):

```md
Term 1

:   Definition 1

Term 2 with *inline markup*

:   Definition 2

        { some code, part of Definition 2 }

    Third paragraph of definition 2.
```

See the [tests](tests/fixtures) for more examples.

## Usage

```rust
let parser = &mut markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(md);
markdown_it_deflist::add(parser);
parser.parse("term\n: definition").render();
// <dl>\n<dt>term</dt>\n<dd>definition</dd>\n</dl>\n
```
