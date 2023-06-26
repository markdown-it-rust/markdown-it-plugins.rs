mod ctype;
mod email;
mod url;
pub(crate) mod utils;
mod www;

/// Match an autolink from the start of the string.
/// Return the link and the number of chars to skip.
pub fn match_start(contents: &str) -> Option<(String, usize)> {
    let bytes_contents = contents.as_bytes();

    if let Some((url, link_end)) = url::match_url(bytes_contents) {
        return Some((url, link_end));
    }
    if let Some((url, link_end)) = www::match_www(bytes_contents) {
        return Some((url, link_end));
    }
    if let Some((email, link_end)) = email::match_email(bytes_contents) {
        return Some((email, link_end));
    }
    None
}

/// Match an autolink from an index in the string (invalid index returns None).
/// Return the link and the number of chars to skip (from index).
/// Note, this enforces the rule that autolinks can only come at the beginning of a line, after whitespace, or any of the delimiting characters `*`, `_`, `~`, and `(`.
pub fn match_index(contents: &str, index: usize) -> Option<(String, usize)> {
    if index > 0 {
        let prev_char = contents.chars().nth(index - 1)?;

        // All such recognized autolinks can only come at the beginning of a line, after whitespace, or any of the delimiting characters *, _, ~, and (.
        if !matches!(prev_char, ' ' | '\t' | '\r' | '\n' | '*' | '_' | '~' | '(') {
            return None;
        }
    }

    let start_contents = contents.get(index..)?;
    let (link, skip_len) = match_start(start_contents)?;

    Some((link, skip_len))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    // non-matches
    #[case("", None)]
    #[case(" ", None)]
    #[case("foo", None)]
    #[case("example.com", None)]
    #[case("www.", None)]
    #[case("@example.com", None)]
    // url matches
    #[case("http://localhost:3000", Some(("http://localhost:3000", 21)))]
    #[case("https://localhost:3000", Some(("https://localhost:3000", 22)))]
    #[case("ftp://localhost:3000", Some(("ftp://localhost:3000", 20)))]
    #[case("http://Á.com", Some(("http://Á.com", 12)))]
    #[case("https://www.wolframalpha.com/input/?i=x^2+(y-(x^2)^(1/3))^2=1", Some(("https://www.wolframalpha.com/input/?i=x^2+(y-(x^2)^(1/3))^2=1", 61)))]
    // www matches
    #[case("www.example.com", Some(("http://www.example.com", 15)))]
    #[case("www.Á.com", Some(("http://www.Á.com", 9)))]
    // email matches
    #[case("john@example.com", Some(("mailto:john@example.com", 16)))]
    #[case("mailto:@example.com", Some(("mailto:@example.com", 19)))]
    #[case("xmpp:john@example.com", Some(("xmpp:john@example.com", 21)))]
    fn test_match_start(#[case] input: &str, #[case] expected: Option<(&str, i32)>) {
        assert_eq!(
            match_start(input),
            expected.and_then(|a| Some((a.0.to_string(), a.1 as usize)))
        );
    }

    #[rstest]
    // 622
    #[case("www.commonmark.org", Some(("http://www.commonmark.org", 18)))]
    // 623
    #[case("www.commonmark.org/help for more information.", Some(("http://www.commonmark.org/help", 23)))]
    // 624
    #[case("www.commonmark.org.", Some(("http://www.commonmark.org", 18)))]
    #[case("www.commonmark.org/a.b.", Some(("http://www.commonmark.org/a.b", 22)))]
    // 625
    #[case("www.google.com/search?q=Markup+(business)", Some(("http://www.google.com/search?q=Markup+(business)", 41)))]
    #[case("www.google.com/search?q=Markup+(business)))", Some(("http://www.google.com/search?q=Markup+(business)", 41)))]
    // 626
    #[case("www.google.com/search?q=(business))+ok", Some(("http://www.google.com/search?q=(business))+ok", 38)))]
    // 627
    #[case("www.google.com/search?q=commonmark&hl=en", Some(("http://www.google.com/search?q=commonmark&hl=en", 40)))]
    #[case("www.google.com/search?q=commonmark&hl;", Some(("http://www.google.com/search?q=commonmark", 34)))]
    // 628
    #[case("www.commonmark.org/he<lp", Some(("http://www.commonmark.org/he", 21)))]
    // 629
    #[case("http://commonmark.org", Some(("http://commonmark.org", 21)))]
    #[case("https://encrypted.google.com/search?q=Markup+(business))", Some(("https://encrypted.google.com/search?q=Markup+(business)", 55)))]
    // 630
    #[case("foo@bar.baz", Some(("mailto:foo@bar.baz", 11)))]
    // 631
    #[case("hello@mail+xyz.example", None)]
    #[case("hello+xyz@mail.example", Some(("mailto:hello+xyz@mail.example", 22)))]
    // 632
    #[case("a.b-c_d@a.b", Some(("mailto:a.b-c_d@a.b", 11)))]
    #[case("a.b-c_d@a.b.", Some(("mailto:a.b-c_d@a.b", 11)))]
    #[case("a.b-c_d@a.b-", None)]
    #[case("a.b-c_d@a.b_", None)]
    // 633
    #[case("mailto:foo@bar.baz", Some(("mailto:foo@bar.baz", 18)))]
    #[case("mailto:a.b-c_d@a.b", Some(("mailto:a.b-c_d@a.b", 18)))]
    #[case("mailto:a.b-c_d@a.b.", Some(("mailto:a.b-c_d@a.b", 18)))]
    #[case("mailto:a.b-c_d@a.b/", Some(("mailto:a.b-c_d@a.b", 18)))]
    #[case("mailto:a.b-c_d@a.b-", None)]
    #[case("mailto:a.b-c_d@a.b_", None)]
    #[case("xmpp:foo@bar.baz", Some(("xmpp:foo@bar.baz", 16)))]
    #[case("xmpp:foo@bar.baz.", Some(("xmpp:foo@bar.baz", 16)))]
    // 634
    #[case("xmpp:foo@bar.baz/txt", Some(("xmpp:foo@bar.baz/txt", 20)))]
    #[case("xmpp:foo@bar.baz/txt@bin", Some(("xmpp:foo@bar.baz/txt@bin", 24)))]
    #[case("xmpp:foo@bar.baz/txt@bin.com", Some(("xmpp:foo@bar.baz/txt@bin.com", 28)))]
    // 635
    #[case("xmpp:foo@bar.baz/txt/bin", Some(("xmpp:foo@bar.baz/txt", 20)))]
    fn test_spec(#[case] input: &str, #[case] expected: Option<(&str, i32)>) {
        assert_eq!(
            match_start(input),
            expected.and_then(|a| Some((a.0.to_string(), a.1 as usize)))
        );
    }

    #[rstest]
    #[case("www.commonmark.org", 0, Some(("http://www.commonmark.org", 18)))]
    #[case(" www.commonmark.org", 0, None)]
    #[case("www.commonmark.org", 100, None)]
    #[case(" www.commonmark.org", 1, Some(("http://www.commonmark.org", 18)))]
    #[case("[www.commonmark.org", 1, None)]
    fn test_match_index(
        #[case] input: &str,
        #[case] index: usize,
        #[case] expected: Option<(&str, i32)>,
    ) {
        assert_eq!(
            match_index(input, index),
            expected.and_then(|a| Some((a.0.to_string(), a.1 as usize)))
        );
    }
}
