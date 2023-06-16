//! development utilities

use prettydiff::diff_lines;
use std::path::PathBuf;

pub struct FixtureFile {
    pub title: String,
    pub input: String,
    pub expected: String,
}

/// Read a fixture file into a FixtureFile struct
pub fn read_fixture_file(file: PathBuf) -> FixtureFile {
    let text = std::fs::read_to_string(file).unwrap();
    let mut lines = text.lines();
    let mut title = String::new();
    let mut input = String::new();
    let mut expected = String::new();
    loop {
        match lines.next() {
            None => panic!("no '....' line found to signal start of input"),
            Some(line) if line.starts_with("....") => break,
            Some(line) => {
                title.push_str(line);
                title.push('\n');
            }
        }
    }
    loop {
        match lines.next() {
            None => panic!("no '....' line found to signal start of expected output"),
            Some(line) if line.starts_with("....") => break,
            Some(line) => {
                input.push_str(line);
                input.push('\n');
            }
        }
    }
    loop {
        match lines.next() {
            None => break,
            Some(line) => {
                expected.push_str(line);
                expected.push('\n');
            }
        }
    }
    // strip preceding empty line in input
    while input.starts_with('\n') {
        input = input[1..].to_string();
    }
    // strip trailing empty lines from input
    while input.ends_with('\n') {
        input.pop();
    }
    // strip preceding empty line in expected
    while expected.starts_with('\n') {
        expected = expected[1..].to_string();
    }

    FixtureFile {
        title,
        input,
        expected,
    }
}

pub fn assert_fixture(f: FixtureFile, actual: &str) {
    if actual != f.expected {
        let diff = diff_lines(&f.expected, actual);
        panic!(
            "\n{}\nDiff:\n{}\n\nActual:\n{actual}\n",
            f.title,
            diff
        )
    }
}
