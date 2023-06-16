//! Plugin to collect footnote definitions
//! and move them to be the last child of the root node.
use markdown_it::{
    parser::core::{CoreRule, Root},
    MarkdownIt, Node, NodeValue,
};

use crate::{definitions::FootnoteDefinition, FootnoteMap};

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into parser
    md.add_rule::<FootnoteCollectRule>();
}

#[derive(Debug)]
struct PlaceholderNode;
impl NodeValue for PlaceholderNode {}

#[derive(Debug)]
struct FootnotesContainerNode;
impl NodeValue for FootnotesContainerNode {
    fn render(&self, node: &Node, fmt: &mut dyn markdown_it::Renderer) {
        let mut attrs = node.attrs.clone();
        attrs.push(("class", "footnotes".into()));
        fmt.cr();
        fmt.self_close("hr", &[("class", "footnotes-sep".into())]);
        fmt.cr();
        fmt.open("section", &attrs);
        fmt.cr();
        fmt.open("ol", &[("class", "footnotes-list".into())]);
        fmt.cr();
        fmt.contents(&node.children);
        fmt.cr();
        fmt.close("ol");
        fmt.cr();
        fmt.close("section");
        fmt.cr();
    }
}

#[derive(Debug)]
struct FootnoteRefAnchor {
    ref_ids: Vec<usize>,
}
impl NodeValue for FootnoteRefAnchor {
    fn render(&self, _: &Node, fmt: &mut dyn markdown_it::Renderer) {
        for ref_id in self.ref_ids.iter() {
            fmt.text(" ");
            fmt.open(
                "a",
                &[
                    ("id", format!("#fnref{}", ref_id)),
                    ("class", String::from("footnote-backref")),
                ],
            );
            fmt.text("↩");
            fmt.close("a");
        }
    }
}

// This is an extension for the markdown parser.
struct FootnoteCollectRule;

impl CoreRule for FootnoteCollectRule {
    // This is a custom function that will be invoked once per document.
    //
    // It has `root` node of the AST as an argument and may modify its
    // contents as you like.
    //
    fn run(root: &mut Node, _: &MarkdownIt) {
        // TODO this seems very cumbersome
        // but it is also how the markdown_it::InlineParserRule works
        let data = root.cast_mut::<Root>().unwrap();
        let root_ext = std::mem::take(&mut data.ext);
        let map = match root_ext.get::<FootnoteMap>() {
            Some(map) => map,
            None => return,
        };

        // walk through the AST and extract all footnote definitions
        let mut defs = vec![];
        root.walk_mut(|node, _| {
            // TODO could use drain_filter if it becomes stable: https://github.com/rust-lang/rust/issues/43244
            // defs.extend(
            //     node.children
            //         .drain_filter(|child| !child.is::<FootnoteDefinition>())
            //         .collect(),
            // );

            for child in node.children.iter_mut() {
                if child.is::<FootnoteDefinition>() {
                    let extracted = std::mem::replace(child, Node::new(PlaceholderNode));
                    match extracted.cast::<FootnoteDefinition>() {
                        Some(def_node) => {
                            // skip footnotes that are not referenced
                            match def_node.def_id {
                                Some(def_id) => {
                                    if map.referenced_by(def_id).is_empty() {
                                        continue;
                                    }
                                }
                                None => continue,
                            }
                        }
                        None => continue,
                    }
                    defs.push(extracted);
                }
            }
            node.children.retain(|child| !child.is::<PlaceholderNode>());
        });
        if defs.is_empty() {
            return;
        }

        // wrap the definitions in a container and append them to the root
        let mut wrapper = Node::new(FootnotesContainerNode);
        wrapper.children = defs;
        root.children.push(wrapper);

        let data = root.cast_mut::<Root>().unwrap();
        data.ext = root_ext;
    }
}
