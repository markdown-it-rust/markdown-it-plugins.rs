//! Plugin to parse definition lists
//!
//! ```rust
//! let md = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(md);
//! markdown_it_deflist::add(md);
//! assert_eq!(
//!     md.parse("term\n: definition").render(),
//!     "<dl>\n<dt>term</dt>\n<dd>definition</dd>\n</dl>\n"
//! );
//! ```

use markdown_it::{
    parser::{
        block::{BlockRule, BlockState},
        inline::InlineRoot,
    },
    plugins::cmark::block::paragraph::{Paragraph, ParagraphScanner},
    MarkdownIt, Node, NodeValue, Renderer,
};

/// Add the definition list plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    // insert this rule into block subparser
    md.block
        .add_rule::<DefinitionListScanner>()
        .before::<ParagraphScanner>();
}

#[derive(Debug)]
struct DefinitionList;
impl NodeValue for DefinitionList {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.cr();
        fmt.open("dl", &node.attrs);
        fmt.cr();
        fmt.contents(&node.children);
        fmt.cr();
        fmt.close("dl");
        fmt.cr();
    }
}

#[derive(Debug)]
struct DefinitionTerm;
impl NodeValue for DefinitionTerm {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.cr();
        fmt.open("dt", &node.attrs);
        fmt.contents(&node.children);
        fmt.close("dt");
        fmt.cr();
    }
}

#[derive(Debug)]
struct DefinitionDescription;
impl NodeValue for DefinitionDescription {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.cr();
        fmt.open("dd", &node.attrs);
        fmt.contents(&node.children);
        fmt.close("dd");
        fmt.cr();
    }
}

/// An extension for the block subparser.
struct DefinitionListScanner;

impl BlockRule for DefinitionListScanner {
    fn check(state: &mut BlockState) -> Option<()> {
        // if it's indented more than 3 spaces, it should be a code block
        if state.line_indent(state.line) >= 4 {
            return None;
        }

        // validation mode validates a dd block only, not a whole deflist
        if !state.node.is::<DefinitionDescription>() {
            return None;
        }
        check_for_description(state, state.line)?;

        Some(())
    }

    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        // if it's indented more than 3 spaces, it should be a code block
        if state.line_indent(state.line) >= 4 {
            return None;
        }

        let start_line = state.line;
        let mut next_line = state.line + 1;
        if next_line >= state.line_max {
            return None;
        }

        if state.is_empty(next_line) {
            next_line += 1;
            if next_line >= state.line_max {
                return None;
            }
        }

        if state.line_offsets[next_line].indent_nonspace < state.blk_indent as i32 {
            return None;
        }

        let mut dd_first_nonspace = check_for_description(state, next_line)?;

        // start definition list
        let mut node_dl = Node::new(DefinitionList);
        let mut dl_tight = true;

        // iterate over definition list items
        // One definition list can contain multiple terms (dt),
        // and one term can be followed by multiple descriptions (dd).
        // Thus, there is two nested loops here

        let mut dt_line = state.line;
        let mut dd_line = next_line;

        'terms: loop {
            let mut prev_empty_end = false;

            // create the term, which is only one line
            let mut node_dt = Node::new(DefinitionTerm);
            node_dt.srcmap = state.get_map(dt_line, dt_line);
            let (content, mapping) = state.get_lines(dt_line, dt_line + 1, state.blk_indent, false);
            node_dt
                .children
                .push(Node::new(InlineRoot::new(content, mapping)));
            node_dl.children.push(node_dt);

            'descriptions: loop {
                // compute the offsets of the description start
                let mut dd_indent_nonspace = dd_first_nonspace as i32
                    - state.line_offsets[dd_line].first_nonspace as i32
                    + state.line_offsets[dd_line].indent_nonspace;
                while dd_first_nonspace < state.line_offsets[dd_line].line_end {
                    let c = state.src[dd_first_nonspace..].chars().next()?;
                    if c == ' ' {
                        dd_indent_nonspace += 1;
                    } else if c == '\t' {
                        dd_indent_nonspace += 4 - dd_indent_nonspace % 4;
                    } else {
                        break;
                    }
                    dd_first_nonspace += 1;
                }

                // cache, then override, the current state
                let cached_state = CachedState {
                    tight: state.tight,
                    blk_indent: state.blk_indent,
                    dd_indent_nonspace: state.line_offsets[dd_line].indent_nonspace,
                    dd_first_nonspace: state.line_offsets[dd_line].first_nonspace,
                };
                state.tight = true;
                state.blk_indent = state.line_offsets[dd_line].indent_nonspace as usize + 2;
                state.line_offsets[dd_line].indent_nonspace = dd_indent_nonspace;
                state.line_offsets[dd_line].first_nonspace = dd_first_nonspace;
                state.line = dd_line;

                // run a nested parse, adding to the description node
                let cached_node =
                    std::mem::replace(&mut state.node, Node::new(DefinitionDescription));
                state.md.block.tokenize(state);
                let mut node_dd = std::mem::replace(&mut state.node, cached_node);
                node_dd.srcmap = state.get_map(next_line, state.line - 1);
                node_dl.children.push(node_dd);

                // If any of the definition list items are tight, mark it as tight
                if !state.tight || prev_empty_end {
                    dl_tight = false;
                }
                // Items become loose if they finish with empty line (except the last one)
                prev_empty_end = (state.line - dd_line) > 1 && state.is_empty(state.line - 1);

                // restore the state
                state.tight = cached_state.tight;
                state.blk_indent = cached_state.blk_indent;
                state.line_offsets[dd_line].indent_nonspace = cached_state.dd_indent_nonspace;
                state.line_offsets[dd_line].first_nonspace = cached_state.dd_first_nonspace;

                next_line = state.line;

                if next_line >= state.line_max
                    || state.line_offsets[next_line].indent_nonspace < state.blk_indent as i32
                {
                    break 'terms;
                }
                match check_for_description(state, next_line) {
                    Some(pos) => dd_first_nonspace = pos,
                    None => break 'descriptions,
                }

                dd_line = next_line;
            }

            dt_line = next_line;
            dd_line = dt_line + 1;

            if next_line >= state.line_max
                || state.is_empty(dt_line)
                || state.line_offsets[dt_line].indent_nonspace < state.blk_indent as i32
                || dd_line >= state.line_max
            {
                break 'terms;
            }
            if state.is_empty(dd_line) {
                dd_line += 1;
            }
            if dd_line >= state.line_max
                || state.line_offsets[dd_line].indent_nonspace < state.blk_indent as i32
            {
                break 'terms;
            }
            match check_for_description(state, dd_line) {
                Some(pos) => dd_first_nonspace = pos,
                None => break 'terms,
            }
        }

        // mark paragraphs tight if needed
        if dl_tight {
            for child in node_dl.children.iter_mut() {
                mark_tight_paragraphs(&mut child.children);
            }
        }

        state.line = start_line;
        Some((node_dl, next_line - start_line))
    }
}

struct CachedState {
    tight: bool,
    blk_indent: usize,
    dd_indent_nonspace: i32,
    dd_first_nonspace: usize,
}

/// Check the line is a description, i.e. `[:~][\t\s]+[^\t\s].*`,
/// return next pos after marker on success.
fn check_for_description(state: &mut BlockState, line: usize) -> Option<usize> {
    let mut chars = state.get_line(line).chars();

    // requires a marker
    let first_char = chars.next()?;
    if first_char != ':' && first_char != '~' {
        return None;
    }

    // requires at least one space after the marker
    let second_char = chars.next()?;
    if second_char != ' ' && second_char != '\t' {
        return None;
    }

    // skip remaining spaces after marker
    // and check if there is any content
    loop {
        match chars.next() {
            Some(' ' | '\t') => {}
            Some(_) => break,
            None => return None,
        }
    }

    Some(state.line_offsets[line].first_nonspace + 1)
}

fn mark_tight_paragraphs(nodes: &mut Vec<Node>) {
    let mut idx = 0;
    while idx < nodes.len() {
        if nodes[idx].is::<Paragraph>() {
            let children = std::mem::take(&mut nodes[idx].children);
            let len = children.len();
            nodes.splice(idx..idx + 1, children);
            idx += len;
        } else {
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use markdown_it::plugins::cmark;

    use super::*;

    #[test]
    fn test_definition_list() {
        let mut md = MarkdownIt::new();
        cmark::add(&mut md);
        add(&mut md);

        println!("test\n  : foo\n      : bar\n");
        let ast = md.parse("test\n  : foo\n     : bar\n");
        // println!("{:?}", ast);
        println!("{}", ast.render());
        // panic!("TODO")
    }
}
