use indoc::indoc;
use rstest::rstest;

use markdown_it_front_matter::add;

#[rstest]
// TODO read cases from fixtures.md
#[case(
    // must have closing
    indoc! {"
        ---
        # Head
    "},
    "<hr />\n<h1>Head</h1>\n"
)]
#[case(
    // should parse empty front matter
    indoc! {"
        ---
        ---
        # Head
    "},
    "<h1>Head</h1>\n"
)]
#[case(
    // ends on last line
    indoc! {"
        ---
        ---
    "},
    ""
)]
#[case(
    // should parse basic front matter
    indoc! {"
        ---
        x: 1
        ---
        # Head
    "},
    "<h1>Head</h1>\n"
)]
#[case(
    // should parse front matter with indentation
    indoc! {"
        ---
        title: Associative arrays
        people:
            name: John Smith
            age: 33
        morePeople: { name: Grace Jones, age: 21 }
        ---
        # Head
    "},
    "<h1>Head</h1>\n"
)]
#[case(
    // should ignore spaces after front matter delimiters
    indoc! {"
        ---
        x: 1
        ---
        # Head
    "},
    "<h1>Head</h1>\n"
)]
#[case(
    // should ignore front matter with less than 3 opening dashes
    indoc! {"
        --
        x: 1
        --
        # Head
    "},
    "<h2>--\nx: 1</h2>\n<h1>Head</h1>\n"
)]
#[case(
    // should require front matter have matching number of opening and closing dashes
    indoc! {"
        ----
        x: 1
        ---
        # Head
    "},
    "<hr />\n<h2>x: 1</h2>\n<h1>Head</h1>\n"
)]
fn fixtures_test(#[case] input: &str, #[case] expected: &str) {
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    add(parser);
    let text = parser.parse(input).xrender();
    assert_eq!(text, expected)
}
