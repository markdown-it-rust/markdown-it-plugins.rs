use crate::ctype::{isalnum, isalpha, isspace};
// use crate::nodes::{AstNode, NodeLink, NodeValue};
// use crate::parser::inlines::make_inline;
// use typed_arena::Arena;
use once_cell::sync::Lazy;
use std::str;
use unicode_categories::UnicodeCategories;

mod ctype;

#[derive(Debug)]
pub struct Link {
    pub url: String,
    pub text: String,
}

pub fn process(
    contents_str: &str,
    // relaxed: bool,
) -> Option<(Link, usize)> {
    let contents = contents_str.as_bytes();
    let len = contents.len();
    let mut i = 0;

    while i < len {
        let mut post_org = None;
        // let mut bracket_opening = 0;

        while i < len {
            
            // cmark-gfm ignores links inside brackets, such as `[[http://example.com]`
            // if !relaxed {
            //     match contents[i] {
            //         b'[' => {
            //             bracket_opening += 1;
            //         }
            //         b']' => {
            //             bracket_opening -= 1;
            //         }
            //         _ => (),
            //     }

            //     if bracket_opening > 0 {
            //         i += 1;
            //         continue;
            //     }
            // }

            match contents[i] {
                b':' => {
                    post_org = url_match(contents, i);
                    if post_org.is_some() {
                        break;
                    }
                }
                b'w' => {
                    post_org = www_match(contents, i);
                    if post_org.is_some() {
                        break;
                    }
                }
                b'@' => {
                    post_org = email_match(contents, i);
                    if post_org.is_some() {
                        break;
                    }
                }
                _ => (),
            }
            i += 1;
        }

        if let Some((link, reverse, _skip)) = post_org {
            // i -= reverse;
            // node.insert_after(link);
            // if i + skip < len {
            //     let remain = str::from_utf8(&contents[i + skip..]).unwrap();
            //     assert!(!remain.is_empty());
            //     link.insert_after(make_inline(
            //         arena,
            //         NodeValue::Text(remain.to_string()),
            //         (0, 1, 0, 1).into(),
            //     ));
            // }
            // contents_str.truncate(i);
            return Some((link, i - reverse));
        }
    }
    None
}

fn www_match(
    contents: &[u8],
    i: usize,
) -> Option<(Link, usize, usize)> {
    static WWW_DELIMS: Lazy<[bool; 256]> = Lazy::new(|| {
        let mut sc = [false; 256];
        for c in &[b'*', b'_', b'~', b'(', b'['] {
            sc[*c as usize] = true;
        }
        sc
    });

    if i > 0 && !isspace(contents[i - 1]) && !WWW_DELIMS[contents[i - 1] as usize] {
        return None;
    }

    if !contents[i..].starts_with(b"www.") {
        return None;
    }

    let mut link_end = match check_domain(&contents[i..], false) {
        None => return None,
        Some(link_end) => link_end,
    };

    while i + link_end < contents.len() && !isspace(contents[i + link_end]) {
        link_end += 1;
    }

    link_end = autolink_delim(&contents[i..], link_end);

    let mut url = "http://".to_string();
    url.push_str(str::from_utf8(&contents[i..link_end + i]).unwrap());

    let link = Link {
        url,
        text: str::from_utf8(&contents[i..link_end + i])
            .unwrap()
            .to_string(),
    };
    Some((link, 0, link_end))
}

fn check_domain(data: &[u8], allow_short: bool) -> Option<usize> {
    let mut np = 0;
    let mut uscore1 = 0;
    let mut uscore2 = 0;

    for (i, c) in unsafe { str::from_utf8_unchecked(data) }.char_indices() {
        if c == '_' {
            uscore2 += 1;
        } else if c == '.' {
            uscore1 = uscore2;
            uscore2 = 0;
            np += 1;
        } else if !is_valid_hostchar(c) && c != '-' {
            if uscore1 == 0 && uscore2 == 0 && (allow_short || np > 0) {
                return Some(i);
            }
            return None;
        }
    }

    if (uscore1 > 0 || uscore2 > 0) && np <= 10 {
        None
    } else if allow_short || np > 0 {
        Some(data.len())
    } else {
        None
    }
}

fn is_valid_hostchar(ch: char) -> bool {
    !ch.is_whitespace() && !ch.is_punctuation()
}

fn autolink_delim(data: &[u8], mut link_end: usize) -> usize {
    static LINK_END_ASSORTMENT: Lazy<[bool; 256]> = Lazy::new(|| {
        let mut sc = [false; 256];
        for c in &[
            b'?', b'!', b'.', b',', b':', b'*', b'_', b'~', b'\'', b'"', b'[', b']',
        ] {
            sc[*c as usize] = true;
        }
        sc
    });

    for (i, &b) in data.iter().enumerate().take(link_end) {
        if b == b'<' {
            link_end = i;
            break;
        }
    }

    while link_end > 0 {
        let cclose = data[link_end - 1];

        let copen = if cclose == b')' { Some(b'(') } else { None };

        if LINK_END_ASSORTMENT[cclose as usize] {
            link_end -= 1;
        } else if cclose == b';' {
            let mut new_end = link_end - 2;

            while new_end > 0 && isalpha(data[new_end]) {
                new_end -= 1;
            }

            if new_end < link_end - 2 && data[new_end] == b'&' {
                link_end = new_end;
            } else {
                link_end -= 1;
            }
        } else if let Some(copen) = copen {
            let mut opening = 0;
            let mut closing = 0;
            for &b in data.iter().take(link_end) {
                if b == copen {
                    opening += 1;
                } else if b == cclose {
                    closing += 1;
                }
            }

            if closing <= opening {
                break;
            }

            link_end -= 1;
        } else {
            break;
        }
    }

    link_end
}

fn url_match(
    contents: &[u8],
    i: usize,
) -> Option<(Link, usize, usize)> {
    const SCHEMES: [&[u8]; 3] = [b"http", b"https", b"ftp"];

    let size = contents.len();

    if size - i < 4 || contents[i + 1] != b'/' || contents[i + 2] != b'/' {
        return None;
    }

    let mut rewind = 0;
    while rewind < i && isalpha(contents[i - rewind - 1]) {
        rewind += 1;
    }

    let cond = |s: &&[u8]| size - i + rewind >= s.len() && &&contents[i - rewind..i] == s;
    if !SCHEMES.iter().any(cond) {
        return None;
    }

    let mut link_end = match check_domain(&contents[i + 3..], true) {
        None => return None,
        Some(link_end) => link_end,
    };

    while link_end < size - i && !isspace(contents[i + link_end]) {
        link_end += 1;
    }

    link_end = autolink_delim(&contents[i..], link_end);

    let url = str::from_utf8(&contents[i - rewind..i + link_end])
        .unwrap()
        .to_string();

    let link = Link {
        url: url.clone(),
        text: url,
    };
    Some((link, rewind, rewind + link_end))
}

fn email_match(
    // arena: &'a Arena<AstNode<'a>>,
    contents: &[u8],
    i: usize,
) -> Option<(Link, usize, usize)> {
    static EMAIL_OK_SET: Lazy<[bool; 256]> = Lazy::new(|| {
        let mut sc = [false; 256];
        for c in &[b'.', b'+', b'-', b'_'] {
            sc[*c as usize] = true;
        }
        sc
    });

    let size = contents.len();

    let mut auto_mailto = true;
    let mut is_xmpp = false;
    let mut rewind = 0;

    while rewind < i {
        let c = contents[i - rewind - 1];

        if isalnum(c) || EMAIL_OK_SET[c as usize] {
            rewind += 1;
            continue;
        }

        if c == b':' {
            if validate_protocol("mailto", contents, i - rewind - 1) {
                auto_mailto = false;
                rewind += 1;
                continue;
            }

            if validate_protocol("xmpp", contents, i - rewind - 1) {
                is_xmpp = true;
                auto_mailto = false;
                rewind += 1;
                continue;
            }
        }

        break;
    }

    if rewind == 0 {
        return None;
    }

    let mut link_end = 1;
    let mut np = 0;

    while link_end < size - i {
        let c = contents[i + link_end];

        if isalnum(c) {
            // empty
        } else if c == b'@' {
            return None;
        } else if c == b'.' && link_end < size - i - 1 && isalnum(contents[i + link_end + 1]) {
            np += 1;
        } else if c == b'/' && is_xmpp {
            // xmpp allows a `/` in the url
        } else if c != b'-' && c != b'_' {
            break;
        }

        link_end += 1;
    }

    if link_end < 2
        || np == 0
        || (!isalpha(contents[i + link_end - 1]) && contents[i + link_end - 1] != b'.')
    {
        return None;
    }

    link_end = autolink_delim(&contents[i..], link_end);
    if link_end == 0 {
        return None;
    }

    let mut url = if auto_mailto {
        "mailto:".to_string()
    } else {
        "".to_string()
    };
    let text = str::from_utf8(&contents[i - rewind..link_end + i]).unwrap();
    url.push_str(text);

    let link = Link {
        url,
        text: text.to_string(),
    };
    Some((link, rewind, rewind + link_end))
}

fn validate_protocol(protocol: &str, contents: &[u8], cursor: usize) -> bool {
    let size = contents.len();
    let mut rewind = 0;

    while rewind < cursor && isalpha(contents[cursor - rewind - 1]) {
        rewind += 1;
    }

    size - cursor + rewind >= protocol.len()
        && &contents[cursor - rewind..cursor] == protocol.as_bytes()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let p = process("a foo@bar.baz");
        println!("{:?}", p);
        panic!("hallo")
    }
}