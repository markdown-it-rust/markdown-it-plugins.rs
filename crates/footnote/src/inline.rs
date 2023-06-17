//! Plugin to parse inline footnotes
//!
//! ```rust
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(parser);
//! markdown_it_footnote::inline::add(parser);
//! let root = parser.parse("Example^[This is a footnote]");
//! let mut names = vec![];
//! root.walk(|node,_| { names.push(node.name()); });
//! assert_eq!(names, vec![
//! "markdown_it::parser::core::root::Root",
//! "markdown_it::plugins::cmark::block::paragraph::Paragraph",
//! "markdown_it::parser::inline::builtin::skip_text::Text",
//! "markdown_it_footnote::inline::InlineFootnote",
//! "markdown_it_footnote::definitions::FootnoteDefinition",
//! "markdown_it::parser::inline::builtin::skip_text::Text",
//! "markdown_it_footnote::references::FootnoteReference"
//! ]);
//! ```
use markdown_it::{
    parser::inline::{InlineRule, InlineState},
    MarkdownIt, Node, NodeValue,
};

use crate::{definitions::FootnoteDefinition, FootnoteMap};

/// Add the inline footnote plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    // insert this rule into inline subparser
    md.inline.add_rule::<InlineFootnoteScanner>();
}

#[derive(Debug)]
pub struct InlineFootnote;
impl NodeValue for InlineFootnote {
    fn render(&self, node: &Node, fmt: &mut dyn markdown_it::Renderer) {
        // simply pass-through to children
        fmt.contents(&node.children);
    }
}

// This is an extension for the inline subparser.
struct InlineFootnoteScanner;

impl InlineRule for InlineFootnoteScanner {
    const MARKER: char = '^';

    fn check(state: &mut InlineState) -> Option<usize> {
        let mut chars = state.src[state.pos..state.pos_max].chars();

        // check line starts with the correct syntax
        let Some('^') = chars.next() else { return None; };
        let Some('[') = chars.next() else { return None; };

        let content_start = state.pos + 2;

        match parse_footnote(state, content_start) {
            Some(content_end) => Some(content_end + 1 - state.pos),
            None => None,
        }
    }

    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        let mut chars = state.src[state.pos..state.pos_max].chars();

        // check line starts with the correct syntax
        let Some('^') = chars.next() else { return None; };
        let Some('[') = chars.next() else { return None; };

        let content_start = state.pos + 2;

        match parse_footnote(state, content_start) {
            Some(content_end) => {
                let foot_map = state.root_ext.get_or_insert_default::<FootnoteMap>();
                let (def_id, ref_id) = foot_map.add_inline_def();

                // create node and set it as current
                let current_node = std::mem::replace(
                    &mut state.node,
                    Node::new(FootnoteDefinition {
                        label: None,
                        def_id: Some(def_id),
                        inline: true,
                    }),
                );

                // perform nested parsing
                let start = state.pos;
                let max = state.pos_max;
                state.pos = content_start;
                state.pos_max = content_end;
                state.md.inline.tokenize(state);
                state.pos = start;
                state.pos_max = max;

                // restore current node
                let def_node = std::mem::replace(&mut state.node, current_node);

                let ref_node = Node::new(crate::references::FootnoteReference {
                    label: None,
                    ref_id,
                    def_id,
                });

                // wrap the footnote definition and reference in an outer node to return
                let mut outer_node = Node::new(InlineFootnote);
                outer_node.children = vec![def_node, ref_node];

                Some((outer_node, content_end + 1 - state.pos))
            }
            None => None,
        }
    }
}

// returns the end position of the footnote
// this function assumes that first character ("[") already matches;
fn parse_footnote(state: &mut InlineState, start: usize) -> Option<usize> {
    let old_pos = state.pos;
    let mut label_end = None;
    state.pos = start + 1;
    let mut found = false;
    while let Some(ch) = state.src[state.pos..state.pos_max].chars().next() {
        if ch == ']' {
            found = true;
            break;
        }
        state.md.inline.skip_token(state);
    }

    if found {
        label_end = Some(state.pos);
    }

    // restore old state
    state.pos = old_pos;

    label_end
}
