use std::fs;
use std::path::PathBuf;

use us_core_condition::validate_condition_json;

fn fixture(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples");
    path.push(name);
    fs::read_to_string(path).expect("fixture")
}

#[test]
fn valid_problem_list() {
    let report = validate_condition_json(&fixture("valid-problem-list.json")).unwrap();
    assert!(report.is_valid(), "{:?}", report.issues);
}

#[test]
fn valid_resolved() {
    let report = validate_condition_json(&fixture("valid-resolved.json")).unwrap();
    assert!(report.is_valid(), "{:?}", report.issues);
}

#[test]
fn icd_only_fails() {
    let report = validate_condition_json(&fixture("invalid-icd-only-code.json")).unwrap();
    assert!(!report.is_valid());
}

#[test]
fn clinical_status_binding_fails() {
    let report =
        validate_condition_json(&fixture("invalid-clinical-status-free-text.json")).unwrap();
    assert!(!report.is_valid());
}

#[test]
fn missing_verification_fails() {
    let report = validate_condition_json(&fixture("invalid-missing-verification.json")).unwrap();
    assert!(!report.is_valid());
}
