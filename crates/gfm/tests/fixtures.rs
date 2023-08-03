use std::path::PathBuf;
use testing::fixture;

#[fixture("tests/fixtures/spec*.md")]
fn test_spec(file: PathBuf) {
    let f = dev::read_fixture_file(file);

    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it_gfm::add(parser);
    let actual = parser.parse(&f.input).xrender();

    dev::assert_no_diff(f, &actual);
}
