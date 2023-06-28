# markdown-it-plugins.rs

<p align="center">
  <img alt="markdown-it-plugins icon" src="https://raw.githubusercontent.com/chrisjsewell/markdown-it-plugins.rs/main/docs/_static/icon.svg">
</p>

Monorepo of plugins for [markdown-it.rs](https://crates.io/crates/markdown-it):

- [markdown-it-front-matter](crates/front_matter/README.md)
- [markdown-it-footnote](crates/footnote/README.md)
- [markdown-it-tasklist](crates/tasklist/README.md)
- [markdown-it-heading-anchors](crates/heading_anchors/README.md)
- [markdown-it-autolink](crates/autolink/README.md)
- [markdown-it-deflist](crates/deflist/README.md)

More to come... (hopefully, many from [mdit-py-plugins](https://github.com/executablebooks/mdit-py-plugins))

Also utility crates:

- [github-slugger](crates/github_slugger/README.md)
- [gfm-autolinks](crates/gfm_autolinks/README.md)

## Development

Feedback on the code is always welcome!

Use [pre-commit](https://pre-commit.com/) to run checkers and formatters before committing:

```bash
git add -A
pre-commit run --all
```

## Release

Use [cargo-release](https://github.com/crate-ci/cargo-release) to release.
(maybe move to cargo-smart-release in the future)

## Notes

For translating markdown-it plugins to rust, here are some useful notes:

- `state.bMarks[startLine] + state.tShift[startLine]` is equivalent to `state.line_offsets[line].first_nonspace`
- `state.eMarks[startLine]` is equivalent to `state.line_offsets[line].line_end`
- `state.sCount[line]` is equivalent to `state.line_offsets[line].indent_nonspace`
- `state.sCount[line] - state.blkIndent` is equivalent to `state.line_indent(state.line)`
