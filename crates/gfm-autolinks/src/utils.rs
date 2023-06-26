use once_cell::sync::Lazy;
use std::str;
use unicode_categories::UnicodeCategories;

use crate::ctype::isalpha;

pub fn check_domain(data: &[u8], allow_short: bool) -> Option<usize> {
    let mut np = 0;
    let mut uscore1 = 0;
    let mut uscore2 = 0;

    // don't allow empty domain names
    if data.is_empty() {
        return None;
    }

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

pub fn autolink_delim(data: &[u8], mut link_end: usize) -> usize {
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
