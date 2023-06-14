# markdown-it-front-matter.rs

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin to process front matter containers.

## Usage

```rust
let parser = &mut markdown_it::MarkdownIt::new();
markdown_it_front_matter::add(parser);
let ast  = parser.parse("---\nfoo: bar\n---\n");

print!("{:#?}", ast.children);
// [
//     Node {
//         children: [],
//         srcmap: Some(
//             (
//                 0,
//                 16,
//             ),
//         ),
//         ext: NodeExtSet(
//             {},
//         ),
//         attrs: [],
//         node_type: markdown_it_front_matter::FrontMatter,
//         node_value: FrontMatter {
//             content: "foo: bar\n",
//         },
//     },
// ]
```

## Valid Front Matter

Essentially, valid front matter is a fenced block:

* Indicated by **three** or **more** dashes: `---`
* Opening and closing fences must be the same number of *dash* characters
* Opening fence must begin on the first line of the markdown string/file
* Opening fence must not be indented

```yaml
---
valid-front-matter: true
---
```
