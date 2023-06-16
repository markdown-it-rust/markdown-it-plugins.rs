//! A [markdown_it] plugin for parsing footnotes
//!
//! ```
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it_footnote::add(parser);
//! let node = parser.parse("[^note]\n\n[^note]: A footnote\n");
//! ```
use definitions::FootnoteDefinition;
use markdown_it::{parser::extset::RootExt, MarkdownIt};
use std::collections::HashMap;

pub mod back_refs;
pub mod collect;
pub mod definitions;
pub mod references;

/// Add the full footnote plugin to the markdown-it parser
pub fn add(md: &mut MarkdownIt) {
    definitions::add(md);
    references::add(md);
    collect::add(md);
    back_refs::add(md);
}

#[derive(Debug, Default)]
// IDs for a footnote label
struct LabelIDs {
    defs: Vec<usize>,
    refs: Vec<usize>,
}

#[derive(Debug, Default)]
/// The set of parsed footnote definition labels,
/// stored in the root node.
pub struct FootnoteMap {
    def_counter: usize,
    ref_counter: usize,
    map: HashMap<String, LabelIDs>,
}
impl RootExt for FootnoteMap {}
impl FootnoteMap {
    fn referenced_by(&self, def: &FootnoteDefinition) -> Option<Vec<usize>> {
        match self.map.get(&def.label) {
            Some(ids) => {
                if ids.defs.first() != Some(&def.def_id) {
                    return None;
                }
                if ids.refs.is_empty() {
                    return None;
                }
                Some(ids.refs.clone())
            }
            None => None,
        }
    }
    /// Returns the ID of the definition
    fn add_def(&mut self, label: &str) -> usize {
        self.def_counter += 1;
        match self.map.get_mut(label) {
            Some(ids) => {
                ids.defs.push(self.def_counter);
            }
            None => {
                self.map.insert(
                    label.to_string(),
                    LabelIDs {
                        defs: vec![self.def_counter],
                        refs: vec![],
                    },
                );
            }
        }
        self.def_counter
    }
    /// If a definition exists,
    /// returns the ID for the definition and reference
    fn add_ref(&mut self, label: &str) -> Option<(usize, usize)> {
        match self.map.get_mut(label) {
            Some(ids) => {
                let def_id = match ids.defs.first() {
                    Some(id) => *id,
                    None => return None,
                };
                self.ref_counter += 1;
                ids.refs.push(self.ref_counter);
                return Some((def_id, self.ref_counter));
            }
            None => return None,
        };
    }
}
