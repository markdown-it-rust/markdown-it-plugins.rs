use std::path::PathBuf;
use testing::fixture;

#[fixture("tests/fixtures/[!_]*.md")]
fn test_html(file: PathBuf) {
    let f = dev::read_fixture_file(file);

    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it_footnote::add(parser);
    let actual = parser.parse(&f.input).render();

    dev::assert_no_diff(f, &actual);
}
