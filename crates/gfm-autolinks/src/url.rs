use std::str;

use crate::{
    ctype::isspace,
    utils::{autolink_delim, check_domain},
};

/// Match a URL link starting with protocol `http`/`https`,
/// from the start of the string.
/// Return the link and the number of chars to skip.
pub fn match_http(contents: &[u8]) -> Option<(String, usize)> {
    let prefix_len: usize;
    if contents.starts_with(b"http://") {
        prefix_len = 7;
    } else if contents.starts_with(b"https://") {
        prefix_len = 8;
    } else {
        return None;
    }

    let mut link_end = match check_domain(&contents[prefix_len..], true) {
        None => return None,
        Some(link_end) => link_end,
    };

    while link_end < contents.len() && !isspace(contents[link_end]) {
        link_end += 1;
    }

    link_end = autolink_delim(contents, link_end);

    let url = str::from_utf8(&contents[..link_end]).unwrap().to_string();
    let skip_len = url.chars().count();
    Some((url, skip_len))
}
