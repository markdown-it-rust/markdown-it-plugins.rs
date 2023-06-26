# gfm-autolinks

[<img alt="crates.io" src="https://img.shields.io/crates/v/gfm-autolinks.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/gfm-autolinks)

A GitHub-flavored Markdown autolink matcher: <https://github.github.com/gfm/#autolinks-extension->.

## Usage

The `match_start` function matches from the start of the string,
and returns `None` or the generated autolink, and the number of characters matched.

```rust
use gfm_autolinks::match_start;

match_start("foo")
// returns None

match_start("http://example.com more")
// returns Some(("http://example.com", 18))

match_start("www.example.com more")
// returns Some(("http://www.example.com", 15))

match_start("me@hotmail.com more")
// returns Some(("mailto:me@hotmail.com", 14))
```

The `match_index` function matches from a given index,
and also returns `None` or the generated autolink, and the number of characters matched.
If the index is not 0, it will also apply the rule,
that the autolink must be preceded by a whitespace character or one of `* _ ~ (`.
Invalid index will return `None`.

```rust
use gfm_autolinks::match_index;

match_index("foo", 10)
// returns None

match_index(" www.example.com", 1)
// returns Some(("http://www.example.com", 18))

match_index("]www.example.com", 1)
// returns None
```

Note, no HTML escaping is performed, e.g.

```rust
use gfm_autolinks::match_start;

match_start("http://example.com?foo=bar&baz=qux")
// returns Some(("http://example.com?foo=bar&baz=qux", 34))
```

## Acknowledgements

Originally adapted from [comrak](https://github.com/kivikakk/comrak/blob/main/src/parser/autolink.rs).
