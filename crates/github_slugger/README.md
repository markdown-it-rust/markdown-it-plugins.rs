# github-slugger

[<img alt="crates.io" src="https://img.shields.io/crates/v/github-slugger.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/github-slugger)

Generate a slug just like GitHub does for markdown headings. It also ensures slugs are unique in the same way GitHub does it.
The overall goal of this package is to emulate the way GitHub handles generating markdown heading anchors as close as possible.
It is based on the [github-slugger](https://github.com/Flet/github-slugger) JavaScript package.

This project is not a markdown or HTML parser: passing `alpha *bravo* charlie`
or `alpha <em>bravo</em> charlie` doesn’t work.
Instead pass the plain text value of the heading: `alpha bravo charlie`.

## Usage

```rust
let mut slugger = github_slugger::Slugger::default();

slugger.slug("foo")
// returns 'foo'

slugger.slug("foo")
// returns 'foo-1'

slugger.slug("bar")
// returns 'bar'

slugger.slug("foo")
// returns 'foo-2'

slugger.slug("Привет non-latin 你好")
// returns 'привет-non-latin-你好'

slugger.slug("😄 emoji")
// returns '-emoji'

slugger.reset()

slugger.slug("foo")
// returns 'foo'
```

Check [`tests/fixtures.json`](tests/fixtures.json) for more examples.

If you need, you can also use the underlying implementation which does not keep
track of the previously slugged strings:

```rust
github_slugger::slug("foo bar baz")
// returns 'foo-bar-baz'

github_slugger::slug("foo bar baz")
// returns the same slug 'foo-bar-baz' because it does not keep track
```
