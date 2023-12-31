//! development utilities
//!
//! This contains shared code for reading test fixtures,
//! testing for differences, and regenerating expected output.

use prettydiff::diff_lines;
use std::path::PathBuf;

pub struct FixtureFile {
    pub file: PathBuf,
    pub title: String,
    pub input: String,
    pub expected: String,
}

/// Read a fixture file into a FixtureFile struct
pub fn read_fixture_file(file: PathBuf) -> FixtureFile {
    let text = std::fs::read_to_string(&file).unwrap();
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
        file,
        title,
        input,
        expected,
    }
}

/// Assert that the actual output matches the expected output,
/// and panic with a diff if it does not.
pub fn assert_no_diff(f: FixtureFile, actual: &str) {
    if actual.trim_end() != f.expected.trim_end() {
        let diff = diff_lines(&f.expected, actual);

        // if environmental variable FORCE_REGEN is set, overwrite the expected output
        if std::env::var("FORCE_REGEN").is_ok() {
            let written = std::fs::write(
                f.file,
                format!(
                    "{}\n......\n\n{}\n\n......\n\n{}\n",
                    f.title.trim_end(),
                    f.input,
                    actual.trim_end()
                ),
            )
            .is_ok();
            if written {
                panic!(
                    "\n{}\nDiff:\n{}\n\nRegenerated expected output",
                    f.title, diff
                );
            }
            panic!(
                "\n{}\nDiff:\n{}\n\nFailed to regenerate expected output",
                f.title, diff
            )
        }
        panic!(
            "\n{}\nDiff:\n{}\nSet FORCE_REGEN=true to update fixture",
            f.title, diff
        );
    }
}
