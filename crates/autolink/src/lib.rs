//! Implement the Github Flavoured Markdown [autolink extension](https://github.github.com/gfm/#autolinks-extension-).
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
    email::{match_any_email, Protocol},
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
    md.inline.add_rule::<ProtocolScanner>();
    // TODO this does not currently work with `_` in the user part
    // since the `_` is parsed as emphasis, before it can be back-scanned
    md.inline.add_rule::<BareEmailScanner>();
}

/// Scanner for bare `www.` URLs.
pub struct WwwScanner;
impl InlineRule for WwwScanner {
    const MARKER: char = 'w';
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        if state.link_level > 0 {
            return None;
        }
        if !state.src[state.pos..state.pos_max].starts_with("www.") {
            return None;
        }
        check_preceding(state, 0)?;
        let (url, length) = match_www(state.src[state.pos..state.pos_max].as_bytes())?;

        let text = state.src[state.pos..state.pos + length].to_string();

        create_autolink(state, 0, length, url, Some(text))
    }
}

/// Scanner for URL protocols that are supported by the GFM autolink extension;
/// `http`, `https`, `mailto`, and `xmpp`.
/// The rule searches for `:`, back-scans to match the protocol name,
/// then forward-spans to match the rest of the URL.
/// Note, this is a balance between performance (only activating on `:` for all protocols)
/// and correctness (risking that the preceding protocol has already been parsed by another rule).
pub struct ProtocolScanner;
impl InlineRule for ProtocolScanner {
    const MARKER: char = ':';
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        // auto-links cannot be inside other links
        if state.link_level > 0 {
            return None;
        }

        // remaining text must start with `:` and have at least 3 more chars
        let remaining = &state.src[state.pos..state.pos_max];
        let mut chars = remaining.chars();
        if chars.next() != Some(':') {
            return None;
        }
        for _ in 0..3 {
            chars.next()?;
        }

        // get preceding "unparsed" text
        let trailing = state.trailing_text_get();

        for (pname, ptype) in vec![("mailto", Protocol::Mailto), ("xmpp", Protocol::Xmpp)] {
            if !trailing.ends_with(pname) {
                continue;
            }
            let bscan_len = pname.len();
            check_preceding(state, bscan_len)?;
            let (full_url, total_len) = match_any_email(
                &[pname.as_bytes(), remaining.as_bytes()].concat(),
                bscan_len + 1,
                ptype,
            )?;
            return create_autolink(state, bscan_len, total_len, full_url, None);
        }

        for pname in &["http", "https"] {
            if !trailing.ends_with(pname) {
                continue;
            }
            let bscan_len = pname.len();
            check_preceding(state, bscan_len)?;
            let (full_url, total_len) =
                match_http(&[pname.as_bytes(), remaining.as_bytes()].concat())?;
            return create_autolink(state, bscan_len, total_len, full_url, None);
        }

        None
    }
}

pub struct BareEmailScanner;
impl InlineRule for BareEmailScanner {
    const MARKER: char = '@';
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        // auto-links cannot be inside other links
        if state.link_level > 0 {
            return None;
        }

        // remaining text must start with `@` and have at least 1 more chars
        let mut chars = state.src[state.pos..state.pos_max].chars();
        if chars.next() != Some('@') {
            return None;
        }
        chars.next()?;

        let trailing = state.trailing_text_get();

        // back-scan to find the local part of the email
        let mut local_rev = vec![];
        for char in trailing.chars().rev() {
            if char.is_ascii_alphanumeric()
                || char == '.'
                || char == '-'
                || char == '_'
                || char == '+'
            {
                local_rev.push(char);
            } else {
                break;
            }
        }
        if local_rev.is_empty() {
            return None;
        }
        let local_len = local_rev.len();
        check_preceding(state, local_len)?;

        // forward-scan to find the domain part of the email
        let remaining = &state.src[state.pos + 1..state.pos_max];
        let mut domain_len = 0;
        let mut num_period = 0;
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
            domain_len += 1;
        }
        // There must be at least one period in the domain
        if domain_len == 0 || num_period == 0 {
            return None;
        }
        // The last character must not be one of `-` or `_`
        let last_char = remaining.chars().nth(domain_len - 1)?;
        if !last_char.is_ascii_alphanumeric() && last_char != '.' {
            return None;
        }
        let text = format!(
            "{}{}",
            String::from_iter(local_rev.iter().rev()),
            &state.src[state.pos..state.pos + 1 + domain_len]
        );
        create_autolink(
            state,
            local_len,
            local_len + 1 + domain_len,
            format!("mailto:{}", text),
            Some(text),
        )
    }
}

/// Check if the preceding syntax allows an autolink.
fn check_preceding(state: &InlineState, bscan_len: usize) -> Option<()> {
    if state.pos - bscan_len > 0 && !check_prev(state.src.chars().nth(state.pos - bscan_len - 1)?) {
        return None;
    }
    Some(())
}

/// Create the autolink node, removing any parsed trailing text.
fn create_autolink(
    state: &mut InlineState,
    bscan_len: usize,
    total_len: usize,
    url: String,
    text: Option<String>,
) -> Option<(Node, usize)> {
    state.trailing_text_pop(bscan_len);

    let content = text.unwrap_or_else(|| url.clone());

    let mut inner_node = Node::new(TextSpecial {
        content: content.to_string(),
        markup: content,
        info: "autolink",
    });
    inner_node.srcmap = state.get_map(state.pos - bscan_len, state.pos - bscan_len + total_len);

    let mut node = Node::new(Autolink { url });
    node.children.push(inner_node);

    state.pos -= bscan_len;

    Some((node, total_len))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("text http://example.com ", &["Root:0:24", "Paragraph:0:24", "Text:0:5", "Autolink:5:23", "TextSpecial:5:23"])]
    #[case("text mailto:bob@test.com ", &["Root:0:25", "Paragraph:0:25", "Text:0:5", "Autolink:5:24", "TextSpecial:5:24"])]
    #[case("text www.example.com ", &["Root:0:21", "Paragraph:0:21", "Text:0:5", "Autolink:5:20", "TextSpecial:5:20"])]
    #[case("text bob@test.com ", &["Root:0:18", "Paragraph:0:18", "Text:0:5", "Autolink:5:17", "TextSpecial:5:17"])]
    fn test_sourcepos(#[case] intput: &str, #[case] expected: &[&str]) {
        let md = &mut markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(md);
        add(md);
        let node = md.parse(intput);
        let mut pos = vec![];
        node.walk(|n, _| {
            n.srcmap.and_then(|s| {
                let (i, j) = s.get_byte_offsets();
                let name = n.name().split("::").last().unwrap_or_default();
                pos.push(format!("{}:{}:{}", name, i, j));
                Some(())
            });
        });
        assert_eq!(pos, expected);
    }
}
