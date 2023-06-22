use std::path::PathBuf;
use testing::fixture;

use markdown_it_heading_anchors::{add_with_options, AnchorPosition, HeadingAnchorOptions};

#[fixture("tests/fixtures/*.md")]
fn test_fixtures(file: PathBuf) {
    let f = dev::read_fixture_file(file);

    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    let mut options = HeadingAnchorOptions::default();
    options.inner_html = String::from("¶");
    options.position = AnchorPosition::Start;
    add_with_options(parser, options);
    let actual = parser.parse(&f.input).render();

    dev::assert_no_diff(f, &actual);
}
