use crate::ctype::isspace;
use crate::utils::{autolink_delim, check_domain};
use std::str;

/// Match a `www` link from the start of the string.
/// Return the link and the number of chars to skip.
pub fn match_www(contents: &[u8]) -> Option<(String, usize)> {
    if !contents.starts_with(b"www.") {
        return None;
    }

    let mut link_end = match check_domain(&contents[4..], false) {
        None => return None,
        Some(link_end) => link_end,
    };

    while link_end < contents.len() && !isspace(contents[link_end]) {
        link_end += 1;
    }

    link_end = autolink_delim(contents, link_end);

    let text = str::from_utf8(&contents[..link_end]).unwrap();
    let skip_len = text.chars().count();
    let url = format!("http://{}", text);

    Some((url, skip_len))
}
