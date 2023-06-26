//! Add id attribute (slug) to headings.
//!
//! ```rust
//! let md = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(md);
//! markdown_it_autolink::add(md);
//!
//! assert_eq!(
//!     md.parse("www.example.com").render(),
//!     "<p><a href=\"http://www.example.com\">www.example.com</a></p>\n",
//! );
//! ```

use gfm_autolinks::{
    check_prev,
    email::{match_mailto, match_xmpp},
    url::match_http,
    www::match_www,
};
use markdown_it::{
    parser::inline::{InlineRule, InlineState, TextSpecial},
    plugins::cmark::inline::autolink::Autolink,
    MarkdownIt, Node,
};

/// Add the GFM autolink extension plugin to MarkdownIt.
pub fn add(md: &mut MarkdownIt) {
    // TODO should these rules be before/after something?
    md.inline.add_rule::<WwwScanner>();
    // TODO whether to only check for `:` and then back-scan,
    // back-scan could be more efficient (less characters to check against) but would require more logic,
    // and its then possible that another rule could have already matched the protocol
    md.inline.add_rule::<HtmlScanner>();
    md.inline.add_rule::<MailtoScanner>();
    md.inline.add_rule::<XmppScanner>();
    // TODO this does not currently work with `_` in the user part
    // since the `_` is parsed as emphasis, before it can be back-scanned
    md.inline.add_rule::<BareEmailScanner>();
}


pub struct WwwScanner;
impl InlineRule for WwwScanner {
    const MARKER: char = 'w';
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        if !state.src[state.pos..state.pos_max].starts_with("www.") {
            return None;
        }
        check_preceding(state)?;
        let (url, length) = match_www(state.src[state.pos..state.pos_max].as_bytes())?;
        Some((gen_node(state, url, length), length))
    }
}

pub struct HtmlScanner;
impl InlineRule for HtmlScanner {
    const MARKER: char = 'h';
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        if !state.src[state.pos..state.pos_max].starts_with("http://")
            && !state.src[state.pos..state.pos_max].starts_with("https://")
        {
            return None;
        }
        check_preceding(state)?;
        let (url, length) = match_http(state.src[state.pos..state.pos_max].as_bytes())?;
        Some((gen_node(state, url, length), length))
    }
}

pub struct MailtoScanner;
impl InlineRule for MailtoScanner {
    const MARKER: char = 'm';
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        if !state.src[state.pos..state.pos_max].starts_with("mailto:") {
            return None;
        }
        check_preceding(state)?;
        let (url, length) = match_mailto(state.src[state.pos..state.pos_max].as_bytes())?;
        Some((gen_node(state, url, length), length))
    }
}

pub struct XmppScanner;
impl InlineRule for XmppScanner {
    const MARKER: char = 'x';
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        if !state.src[state.pos..state.pos_max].starts_with("xmpp:") {
            return None;
        }
        check_preceding(state)?;
        let (url, length) = match_xmpp(state.src[state.pos..state.pos_max].as_bytes())?;
        Some((gen_node(state, url, length), length))
    }
}

pub struct BareEmailScanner;
impl InlineRule for BareEmailScanner {
    const MARKER: char = '@';
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        if state.link_level > 0 {
            return None;
        }

        let mut chars = state.src[state.pos..state.pos_max].chars();
        if chars.next() != Some('@') {
            return None;
        }
        chars.next()?;

        let trailing = state.trailing_text_get();

        let mut user_rev = vec![];
        for char in trailing.chars().rev() {
            if char.is_ascii_alphanumeric()
                || char == '.'
                || char == '-'
                || char == '_'
                || char == '+'
            {
                user_rev.push(char);
            } else {
                break;
            }
        }

        if user_rev.is_empty() {
            return None;
        }
        let before_length = user_rev.len();

        let user = String::from_iter(user_rev.iter().rev());

        if state.pos - before_length > 0
            && !check_prev(state.src.chars().nth(state.pos - before_length - 1)?)
        {
            return None;
        }

        let mut after_length = 0;
        let mut num_period = 0;
        let remaining = &state.src[state.pos + 1..state.pos_max];
        for (i, c) in remaining.char_indices() {
            if c.is_ascii_alphanumeric() {
            } else if c == '@' {
                return None;
            } else if c == '.'
                && remaining
                    .chars()
                    .nth(i + 1)
                    .is_some_and(|c| c.is_ascii_alphanumeric())
            {
                num_period += 1;
            } else if c != '-' && c != '_' {
                break;
            }

            after_length += 1;
        }

        if after_length == 0 || num_period == 0 {
            return None;
        }
        let last_char = remaining.chars().nth(after_length - 1)?;
        if !last_char.is_ascii_alphanumeric() && last_char != '.' {
            return None;
        }

        state.trailing_text_pop(before_length);

        let mut url = format!(
            "{}{}",
            user,
            &state.src[state.pos..state.pos + 1 + after_length]
        );

        let mut inner_node = Node::new(TextSpecial {
            content: url.to_string(),
            markup: url.to_string(),
            info: "autolink",
        });
        inner_node.srcmap = state.get_map(
            state.pos - before_length,
            state.pos + 1 + after_length - before_length,
        );

        url.insert_str(0, "mailto:");
        let mut node = Node::new(Autolink { url });
        node.children.push(inner_node);

        state.pos -= before_length;

        Some((node, before_length + 1 + after_length))
    }
}

/// Check if the preceding syntax allows an autolink.
fn check_preceding(state: &InlineState) -> Option<()> {
    if state.link_level > 0 {
        return None;
    }
    if state.pos > 0 && !check_prev(state.src[state.pos - 1..state.pos].chars().next()?) {
        return None;
    }
    Some(())
}

/// Generate a node for an autolink.
fn gen_node(state: &InlineState, url: String, length: usize) -> Node {
    let text = &state.src[state.pos..state.pos + length];

    let mut inner_node = Node::new(TextSpecial {
        content: text.to_string(),
        markup: text.to_string(),
        info: "autolink",
    });
    inner_node.srcmap = state.get_map(state.pos, state.pos + length);

    let mut node = Node::new(Autolink { url });
    node.children.push(inner_node);

    node
}
