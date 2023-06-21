//! A [markdown_it] plugin for parsing tasklists
//!
//! ```rust
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(parser);
//! markdown_it_tasklist::add(parser);
//! let root = parser.parse("- [x] task");
//! let mut names = vec![];
//! root.walk(|node,_| { names.push(node.name()); });
//! assert_eq!(names, vec![
//! "markdown_it::parser::core::root::Root",
//! "markdown_it::plugins::cmark::block::list::BulletList",
//! "markdown_it::plugins::cmark::block::list::ListItem",
//! "markdown_it_tasklist::TodoCheckbox",
//! "markdown_it::parser::inline::builtin::skip_text::Text",
//! ]);
//! ```

use markdown_it::{
    parser::{core::CoreRule, inline::Text},
    plugins::cmark::block::{
        list::{BulletList, ListItem, OrderedList},
        paragraph::Paragraph,
    },
    MarkdownIt, Node, NodeValue, Renderer,
};
use once_cell::sync::Lazy;
use regex::Regex;

/// Add the tasklist plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<TasklistRule<false>>();
}

/// Add the tasklist plugin to the parser
pub fn add_disabled(md: &mut MarkdownIt) {
    md.add_rule::<TasklistRule<true>>();
}

#[derive(Debug)]
pub struct TodoCheckbox {
    pub checked: bool,
    pub disabled: bool,
}

impl NodeValue for TodoCheckbox {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();
        attrs.push(("class", "task-list-item-checkbox".into()));
        attrs.push(("type", "checkbox".into()));
        if self.disabled {
            attrs.push(("disabled", "".into()));
        }
        if self.checked {
            attrs.push(("checked", "".into()));
        }
        fmt.self_close("input", &attrs);
    }
}

struct TasklistRule<const DISABLED: bool>;

static CHECKBOX_CHECKED_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\[[xX]\][\s\t\n\v\f\r]").unwrap());
static CHECKBOX_UNCHECKED_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\[\s\][\s\t\n\v\f\r]").unwrap());

impl<const DISABLED: bool> CoreRule for TasklistRule<DISABLED> {
    fn run(root: &mut Node, _: &MarkdownIt) {
        fn walk_recursive(node: &mut Node, disabled: bool) {
            if node.is::<Paragraph>() {
                // Paragraphs cannot contain lists, so we can stop here,
                // without walking children
                return;
            }
            if node.is::<BulletList>() || node.is::<OrderedList>() {
                let mut contains_task = false;
                for item in node.children.iter_mut() {
                    if !item.is::<ListItem>() {
                        continue;
                    }
                    if let Some(child) = item.children.first_mut() {
                        // can be a paragraph->text or text, depending on if the list is tight
                        let mut text_value = None;
                        if child.cast_mut::<Paragraph>().is_some() {
                            if let Some(child) = child.children.first_mut() {
                                if let Some(value) = child.cast_mut::<Text>() {
                                    text_value = Some(value);
                                }
                            }
                        } else if let Some(value) = child.cast_mut::<Text>() {
                            text_value = Some(value);
                        }
                        if let Some(text) = text_value {
                            // TODO fix source mappings
                            if CHECKBOX_UNCHECKED_RE.is_match(&text.content) {
                                contains_task = true;
                                text.content.replace_range(0..3, "");
                                item.attrs.push(("class", "task-list-item".into()));
                                item.children.insert(
                                    0,
                                    Node::new(TodoCheckbox {
                                        checked: false,
                                        disabled,
                                    }),
                                );
                            } else if CHECKBOX_CHECKED_RE.is_match(&text.content) {
                                contains_task = true;
                                text.content.replace_range(0..3, "");
                                item.attrs.push(("class", "task-list-item".into()));
                                item.children.insert(
                                    0,
                                    Node::new(TodoCheckbox {
                                        checked: true,
                                        disabled,
                                    }),
                                );
                            }
                        }
                    }
                }
                if contains_task {
                    node.attrs.push(("class", "contains-task-list".into()));
                }
            }
            for n in node.children.iter_mut() {
                stacker::maybe_grow(64 * 1024, 1024 * 1024, || {
                    walk_recursive(n, disabled);
                });
            }
        }

        walk_recursive(root, DISABLED);
    }
}
