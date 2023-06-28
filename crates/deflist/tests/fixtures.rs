use std::path::PathBuf;
use testing::fixture;

#[fixture("tests/fixtures/example*.md")]
fn test_fixtures_examples(file: PathBuf) {
    let f = dev::read_fixture_file(file);

    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it_deflist::add(parser);
    markdown_it::plugins::sourcepos::add(parser);
    let actual = parser.parse(&f.input).render();

    dev::assert_no_diff(f, &actual);
}

#[fixture("tests/fixtures/extra*.md")]
fn test_fixtures_extras(file: PathBuf) {
    let f = dev::read_fixture_file(file);

    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::extra::tables::add(parser);
    markdown_it_deflist::add(parser);
    markdown_it::plugins::sourcepos::add(parser);
    let actual = parser.parse(&f.input).render();

    dev::assert_no_diff(f, &actual);
}
