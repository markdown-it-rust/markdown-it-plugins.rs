# markdown-it-plugins.rs

Monorepo of plugins for [markdown-it.rs](https://crates.io/crates/markdown-it):

- [markdown-it-front-matter](crates/front_matter/README.md)
- [markdown-it-footnote](crates/footnote/README.md)
- [markdown-it-tasklist](crates/tasklist/README.md)

More to come... (hopefully, many from [mdit-py-plugins](https://github.com/executablebooks/mdit-py-plugins))

## Development

Feedback on the code is always welcome!

Use [pre-commit](https://pre-commit.com/) to run checkers and formatters before committing:

```bash
git add -A
pre-commit run --all
```

## Release

Use [cargo-release](https://github.com/crate-ci/cargo-release) to release.
