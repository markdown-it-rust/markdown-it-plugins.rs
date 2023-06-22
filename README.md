# markdown-it-plugins.rs

Monorepo of plugins for [markdown-it.rs](https://crates.io/crates/markdown-it):

- [markdown-it-front-matter](crates/front_matter/README.md)
- [markdown-it-footnote](crates/footnote/README.md)
- [markdown-it-tasklist](crates/tasklist/README.md)
- [markdown-it-heading-anchors](crates/heading_anchors/README.md)

More to come... (hopefully, many from [mdit-py-plugins](https://github.com/executablebooks/mdit-py-plugins))

Also utility crates:

- [github-slugger](crates/github_slugger/README.md)

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
