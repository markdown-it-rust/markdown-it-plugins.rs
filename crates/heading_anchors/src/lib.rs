//! Add id attribute (slug) to headings.
//!
//! ```rust
//! use markdown_it_heading_anchors::{
//!     add_with_options, HeadingAnchorOptions, AnchorPosition
//! };
//!
//! let md = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(md);
//! let mut options = HeadingAnchorOptions::default();
//! options.position = AnchorPosition::Start;
//! options.inner_html = String::from("¶");
//! add_with_options(md, options);
//!
//! assert_eq!(
//!     md.parse("# heading\n\n# heading").render(),
//!     "<h1>\
//!     <a aria-hidden=\"true\" class=\"anchor\" id=\"heading\" href=\"#heading\">¶</a>\
//!     heading</h1>\n\
//!     <h1>\
//!     <a aria-hidden=\"true\" class=\"anchor\" id=\"heading-1\" href=\"#heading-1\">¶</a>\
//!     heading</h1>\n",
//! );
//! ```

use github_slugger::Slugger;
use markdown_it::{
    parser::{core::CoreRule, extset::MarkdownItExt},
    plugins::cmark::block::{heading::ATXHeading, lheading::SetextHeader},
    MarkdownIt, Node, NodeValue,
};

pub fn add(md: &mut MarkdownIt) {
    md.ext.get_or_insert_default::<HeadingAnchorOptions>();
    md.add_rule::<AddHeadingAnchors>();
}

pub fn add_with_options(md: &mut MarkdownIt, options: HeadingAnchorOptions) {
    md.ext.insert(options);
    md.add_rule::<AddHeadingAnchors>();
}

#[derive(Debug)]
pub enum AnchorPosition {
    Start,
    End,
    None,
}

#[derive(Debug)]
pub struct HeadingAnchorOptions {
    /// Minimum heading level to add anchors to.
    pub min_level: u8,
    /// Maximum heading level to add anchors to.
    pub max_level: u8,
    /// Whether to add the id attribute to the heading itself.
    pub id_on_heading: bool,
    /// Where to add the anchor.
    pub position: AnchorPosition,
    /// Classes to add to the anchor.
    pub classes: Vec<String>,
    /// Inner HTML of the anchor.
    pub inner_html: String,
    // TODO allow custom slugger
    // (must make sure reset is called, or create new slugger for each use)
    // TODO id prefix (different to href,
    // see <https://github.com/Flet/markdown-it-github-headings/tree/master#why-should-i-prefix-heading-ids>)
}
impl Default for HeadingAnchorOptions {
    fn default() -> Self {
        Self {
            min_level: 1,
            max_level: 6,
            id_on_heading: false,
            position: AnchorPosition::Start,
            classes: vec![String::from("anchor")],
            inner_html: String::from(
                r#"<svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="m7.775 3.275 1.25-1.25a3.5 3.5 0 1 1 4.95 4.95l-2.5 2.5a3.5 3.5 0 0 1-4.95 0 .751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018 1.998 1.998 0 0 0 2.83 0l2.5-2.5a2.002 2.002 0 0 0-2.83-2.83l-1.25 1.25a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042Zm-4.69 9.64a1.998 1.998 0 0 0 2.83 0l1.25-1.25a.751.751 0 0 1 1.042.018.751.751 0 0 1 .018 1.042l-1.25 1.25a3.5 3.5 0 1 1-4.95-4.95l2.5-2.5a3.5 3.5 0 0 1 4.95 0 .751.751 0 0 1-.018 1.042.751.751 0 0 1-1.042.018 1.998 1.998 0 0 0-2.83 0l-2.5 2.5a1.998 1.998 0 0 0 0 2.83Z"></path></svg>"#,
            ),
        }
    }
}
impl MarkdownItExt for HeadingAnchorOptions {}

#[derive(Debug)]
pub struct Permalink {
    pub href: String,
    pub id: Option<String>,
    pub inner_html: String,
}
impl NodeValue for Permalink {
    fn render(&self, node: &Node, fmt: &mut dyn markdown_it::Renderer) {
        let mut attrs = node.attrs.clone();
        if let Some(id) = &self.id {
            attrs.push(("id", id.clone()));
        }
        attrs.push(("href", format!("#{}", self.href)));
        fmt.open("a", &attrs);
        fmt.text_raw(&self.inner_html);
        fmt.close("a");
    }
}

struct AddHeadingAnchors;
impl CoreRule for AddHeadingAnchors {
    fn run(root: &mut Node, md: &MarkdownIt) {
        let options = md.ext.get::<HeadingAnchorOptions>().unwrap();
        let mut slugger = Slugger::default();
        root.walk_mut(|node, _| {
            if let Some(value) = node.cast::<ATXHeading>() {
                if value.level < options.min_level || value.level > options.max_level {
                    return;
                }
            }
            if let Some(value) = node.cast::<SetextHeader>() {
                if value.level < options.min_level || value.level > options.max_level {
                    return;
                }
            }
            if node.is::<ATXHeading>() || node.is::<SetextHeader>() {
                // TODO strip image (alt) text
                let id = slugger.slug(&node.collect_text());
                if options.id_on_heading {
                    node.attrs.push(("id", id.clone()));
                }
                let permalink = Permalink {
                    href: id.clone(),
                    id: {
                        if options.id_on_heading {
                            None
                        } else {
                            Some(id)
                        }
                    },
                    inner_html: options.inner_html.clone(),
                };
                let mut link_node = Node::new(permalink);
                link_node.attrs.push(("aria-hidden", String::from("true")));
                for class in &options.classes {
                    link_node.attrs.push(("class", class.clone()));
                }
                match options.position {
                    AnchorPosition::Start => {
                        node.children.insert(0, link_node);
                    }
                    AnchorPosition::End => {
                        node.children.push(link_node);
                    }
                    AnchorPosition::None => {}
                }
            }
        });
    }
}
