//! A [markdown_it] plugin for parsing front matter
//!
//! ```
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it_front_matter::add(parser);
//! let node  = parser.parse("---\nfoo: bar\n---\n");
//! ```

use markdown_it::parser::block::{BlockRule, BlockState};
use markdown_it::parser::core::Root;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
/// AST node for front-matter
pub struct FrontMatter {
    pub content: String,
}

impl NodeValue for FrontMatter {
    fn render(&self, _node: &Node, _fmt: &mut dyn Renderer) {
        // simply bypass the front-matter in HTML output
    }
}

/// Add the front-matter extension to the markdown parser
pub fn add(md: &mut MarkdownIt) {
    // insert this rule into block subparser
    md.block.add_rule::<FrontMatterBlockScanner>().before_all();
}

/// An extension for the block subparser.
struct FrontMatterBlockScanner;

impl BlockRule for FrontMatterBlockScanner {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        // check the parent is the document Root
        if !state.node.is::<Root>() {
            return None;
        }

        // check we are on the first line of the document
        if state.line != 0 {
            return None;
        }

        // check line starts with opening dashes
        let opening = state
            .get_line(state.line)
            .chars()
            .take_while(|c| *c == '-')
            .collect::<String>();
        if !opening.starts_with("---") {
            return None;
        }

        // Search for the end of the block
        let mut next_line = state.line;
        loop {
            next_line += 1;
            if next_line >= state.line_max {
                return None;
            }

            let line = state.get_line(next_line);
            if line.starts_with(&opening) {
                break;
            }
        }

        // get the content of the block
        let (content, _) = state.get_lines(state.line + 1, next_line, 0, true);

        // return new node and number of lines it occupies
        Some((Node::new(FrontMatter { content }), next_line + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let parser = &mut markdown_it::MarkdownIt::new();
        add(parser);
        let node = parser.parse("---\nfoo: bar\n---\nhallo\n");
        // println!("{:#?}", ast.children.first());
        assert!(node.children.first().unwrap().is::<FrontMatter>());

        let text = node.render();
        assert_eq!(text, "hallo\n")
    }
}
