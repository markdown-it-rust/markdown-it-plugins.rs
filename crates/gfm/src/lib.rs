//! A [markdown_it] plugin for parsing Github Flavoured Markdown
//!
//! ```rust
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it_gfm::add(parser);
//! let root = parser.parse("https://github.github.com/gfm");
//! assert_eq!(root.render(), "<p><a href=\"https://github.github.com/gfm\">https://github.github.com/gfm</a></p>\n");
//! ```
use markdown_it::MarkdownIt;

/// Add the GFM plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    markdown_it::plugins::cmark::add(md);
    markdown_it::plugins::extra::tables::add(md);
    markdown_it::plugins::extra::strikethrough::add(md);
    markdown_it::plugins::html::add(md);
    // // TODO html sanitation
    markdown_it_tasklist::add_disabled(md);
    markdown_it_autolink::add(md);
}

/// Add the GFM plugin to the parser, plus heading anchors
pub fn add_with_anchors(md: &mut MarkdownIt) {
    add(md);
    markdown_it_heading_anchors::add(md);
}
