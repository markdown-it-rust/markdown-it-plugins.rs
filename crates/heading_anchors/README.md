# markdown-it-heading-anchors.rs

[<img alt="crates.io" src="https://img.shields.io/crates/v/markdown-it-heading-anchors.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/markdown-it-heading-anchors)

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin that adds an id attribute to headings and optionally permalinks.

The default behaviour is designed to imitate GitHub's heading anchors as closely as possible, but it can be configured to suit your needs.

## Usage

```rust
let parser = &mut markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(md);
markdown_it_heading_anchors::add(parser);
parser.parse("# Heading").render();
// <h1><a aria-hidden="true" class="anchor" id="heading" href="#heading">
// <svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="m7.775 3.275 1.25-1.25a3.5 3.5 0 1 1 4.95 4.95l-2.5 2.5a3.5 3.5 0 0 1-4.95 0 .751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018 1.998 1.998 0 0 0 2.83 0l2.5-2.5a2.002 2.002 0 0 0-2.83-2.83l-1.25 1.25a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042Zm-4.69 9.64a1.998 1.998 0 0 0 2.83 0l1.25-1.25a.751.751 0 0 1 1.042.018.751.751 0 0 1 .018 1.042l-1.25 1.25a3.5 3.5 0 1 1-4.95-4.95l2.5-2.5a3.5 3.5 0 0 1 4.95 0 .751.751 0 0 1-.018 1.042.751.751 0 0 1-1.042.018 1.998 1.998 0 0 0-2.83 0l-2.5 2.5a1.998 1.998 0 0 0 0 2.83Z"></path></svg>
// </a>Head</h1>
```

## Options

To change the default options, use the `add_with_options` function:

```rust
use markdown_it_heading_anchors::{
    add_with_options, HeadingAnchorOptions, AnchorPosition
};

let parser = &mut markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(md);
let mut options = HeadingAnchorOptions::default();
options.position = AnchorPosition::After;
options.inner_html = String::from("¶");
add_with_options(parser, options);
parser.parse("# Heading").render();
// <h1>Heading<a aria-hidden="true" class="anchor" id="heading" href="#heading">¶</a></h1>
```

Available options:

| Name | Type | Default | Description |
| ---- | ---- | ------- | :---------- |
| `min_level` | `u8` | 1 | Minimum heading level to add anchors to. |
| `max_level` | `u8` | 6 | Maximum heading level to add anchors to. |
| `id_on_heading` | `bool` | `false` | Whether to add the id attribute to the heading. |
| `position` | `AnchorPosition` | `::Start` | Where to place the anchor in the heading children |
| `classes` | `Vec<String>` | `["anchor"]` | Classes to add to the anchor. |
| `inner_html` | `String` | see example | HTML to add inside the anchor (i.e. the icon). |

## TODO

- Ignore alt text in images (also custom "textify"?).
- Allow for customizing the slug generation function.
- Allow for prefixing the `id` attribute.

## Acknowledgements

Adapted from <https://github.com/Flet/markdown-it-github-headings> and <https://github.com/executablebooks/mdit-py-plugins>
(see also <https://github.com/valeriangalliat/markdown-it-anchor>).
