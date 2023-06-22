// add test
use serde::{Deserialize, Serialize};
use std::{env, fs::read_to_string, path::PathBuf};

#[derive(Serialize, Deserialize)]
struct Case {
    name: String,
    input: String,
    expected: String,
}

#[test]
fn test_cases() {
    // read test cases from tests/cases.json
    // TODO maybe could use https://github.com/commure/datatest instead
    let cases_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "#[fixture] requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ))
    .join("tests/fixtures.json");
    let cases_text = read_to_string(&cases_path).unwrap();
    let cases: Vec<Case> = serde_json::from_str::<Vec<Case>>(&cases_text).unwrap();

    // setup slugger
    let mut slugger = github_slugger::Slugger::default();
    // apply
    for (num, case) in cases.iter().enumerate() {
        // note test 'Unassigned' failed, but when checked in github, the expected was found to be wrong
        let actual = slugger.slug(&case.input);
        assert_eq!(actual, case.expected, "case #{}: {}", num, case.name);
    }
}
