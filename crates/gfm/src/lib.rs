//! A [markdown_it] plugin for parsing Github Flavoured Markdown
//!
//! ```rust
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it_gfm::add(parser);
//! let root = parser.parse("https://github.github.com/gfm");
//! assert_eq!(root.render(), "<p><a href=\"https://github.github.com/gfm\">https://github.github.com/gfm</a></p>\n");
//! ```
use markdown_it::parser::inline::builtin::InlineParserRule;
use markdown_it::plugins::html::html_block::HtmlBlock;
use markdown_it::plugins::html::html_inline::HtmlInline;
use markdown_it::{parser::core::CoreRule, MarkdownIt, Node};
use regex::Regex;

/// Add the GFM plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    markdown_it::plugins::cmark::add(md);
    markdown_it::plugins::extra::tables::add(md);
    markdown_it::plugins::extra::strikethrough::add(md);
    markdown_it::plugins::html::add(md);
    md.add_rule::<TagFilter>().after::<InlineParserRule>();
    markdown_it_tasklist::add_disabled(md);
    markdown_it_autolink::add(md);
}

/// Add the GFM plugin to the parser, plus heading anchors
pub fn add_with_anchors(md: &mut MarkdownIt) {
    add(md);
    markdown_it_heading_anchors::add(md);
}

/// Implement the Disallowed Raw HTML (tagfilter) rule
struct TagFilter;
impl CoreRule for TagFilter {
    fn run(root: &mut Node, _md: &MarkdownIt) {
        let regex = Regex::new(
            r#"<(?i)(iframe|noembed|noframes|plaintext|script|style|title|textarea|xmp)"#,
        )
        .unwrap();
        root.walk_mut(|node, _| {
            if let Some(value) = node.cast_mut::<HtmlBlock>() {
                value.content = regex.replace_all(&value.content, "&lt;$1").to_string();
            }
            if let Some(value) = node.cast_mut::<HtmlInline>() {
                value.content = regex.replace_all(&value.content, "&lt;$1").to_string();
            }
        });
    }
}
