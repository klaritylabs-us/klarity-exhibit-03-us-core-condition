//! US Core Condition checks — problem list and clinical status bindings.
//!
//! Targets ICD-only exports, wrong CodeSystem URLs on clinicalStatus, and missing
//! verificationStatus that still parse as valid Condition JSON.

use serde_json::Value;

use crate::outcome::{ValidationIssue, ValidationReport};
use crate::profile::{
    CONDITION_CATEGORY_SYSTEM, CONDITION_CLINICAL_SYSTEM, CONDITION_VER_STATUS_SYSTEM,
    ICD10_SYSTEM, PROBLEM_LIST_CATEGORY, SNOMED_SYSTEM, US_CORE_CONDITION_PROFILE,
};

pub fn validate_condition_json(json: &str) -> Result<ValidationReport, String> {
    let value: Value = serde_json::from_str(json).map_err(|e| format!("JSON parse: {e}"))?;
    Ok(validate_condition_value(&value))
}

pub fn validate_condition_value(condition: &Value) -> ValidationReport {
    let mut report = ValidationReport::ok();

    if condition.get("resourceType").and_then(Value::as_str) != Some("Condition") {
        report.push(ValidationIssue::error(
            "Condition.resourceType",
            "resourceType must be Condition",
            "condition-resource-type",
        ));
        return report;
    }

    check_meta_profile(condition, &mut report);
    check_clinical_status(condition, &mut report);
    check_verification_status(condition, &mut report);
    check_category_problem_list(condition, &mut report);
    check_code_snomed(condition, &mut report);
    check_subject(condition, &mut report);
    check_resolved_consistency(condition, &mut report);

    report
}

fn check_meta_profile(condition: &Value, report: &mut ValidationReport) {
    let profiles = condition
        .pointer("/meta/profile")
        .and_then(Value::as_array);

    let has_profile = profiles.is_some_and(|arr| {
        arr.iter()
            .filter_map(Value::as_str)
            .any(|p| p == US_CORE_CONDITION_PROFILE)
    });

    if !has_profile {
        report.push(ValidationIssue::error(
            "Condition.meta.profile",
            format!("meta.profile should include {}", US_CORE_CONDITION_PROFILE),
            "us-core-meta-profile",
        ));
    }
}

fn coding_code_in(system: &str, codings: &Value) -> Option<String> {
    codings
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|c| c.get("system").and_then(Value::as_str) == Some(system))
                .and_then(|c| c.get("code").and_then(Value::as_str))
                .map(String::from)
        })
}

fn check_clinical_status(condition: &Value, report: &mut ValidationReport) {
    let clinical = condition.get("clinicalStatus");
    if clinical.is_none() {
        report.push(ValidationIssue::error(
            "Condition.clinicalStatus",
            "clinicalStatus is required for US Core Condition",
            "clinical-status-required",
        ));
        return;
    }

    let codings = clinical.and_then(|c| c.get("coding"));
    let code = codings.and_then(|c| coding_code_in(CONDITION_CLINICAL_SYSTEM, c));

    if code.is_none() {
        report.push(ValidationIssue::error(
            "Condition.clinicalStatus.coding",
            format!(
                "clinicalStatus must use {} (e.g. active, resolved)",
                CONDITION_CLINICAL_SYSTEM
            ),
            "clinical-status-binding",
        ));
    }
}

fn check_verification_status(condition: &Value, report: &mut ValidationReport) {
    let verification = condition.get("verificationStatus");
    if verification.is_none() {
        report.push(ValidationIssue::error(
            "Condition.verificationStatus",
            "verificationStatus is required for US Core Condition",
            "verification-status-required",
        ));
        return;
    }

    let codings = verification.and_then(|c| c.get("coding"));
    let code = codings.and_then(|c| coding_code_in(CONDITION_VER_STATUS_SYSTEM, c));

    if code.is_none() {
        report.push(ValidationIssue::error(
            "Condition.verificationStatus.coding",
            format!(
                "verificationStatus must use {} (e.g. confirmed, provisional)",
                CONDITION_VER_STATUS_SYSTEM
            ),
            "verification-status-binding",
        ));
    }
}

fn check_category_problem_list(condition: &Value, report: &mut ValidationReport) {
    let categories = condition.get("category").and_then(Value::as_array);
    let has_problem_list = categories.is_some_and(|cats| {
        cats.iter().any(|cat| {
            cat.get("coding")
                .and_then(Value::as_array)
                .is_some_and(|codings| {
                    codings.iter().any(|c| {
                        c.get("system").and_then(Value::as_str) == Some(CONDITION_CATEGORY_SYSTEM)
                            && c.get("code").and_then(Value::as_str)
                                == Some(PROBLEM_LIST_CATEGORY)
                    })
                })
        })
    });

    if !has_problem_list {
        report.push(ValidationIssue::warning(
            "Condition.category",
            format!(
                "problem list feeds usually include category {}|{}",
                CONDITION_CATEGORY_SYSTEM,
                PROBLEM_LIST_CATEGORY
            ),
            "problem-list-category",
        ));
    }
}

fn check_code_snomed(condition: &Value, report: &mut ValidationReport) {
    let codings = condition.pointer("/code/coding").and_then(Value::as_array);

    let has_snomed = codings.is_some_and(|arr| {
        arr.iter().any(|c| {
            c.get("system").and_then(Value::as_str) == Some(SNOMED_SYSTEM)
                && c
                    .get("code")
                    .and_then(Value::as_str)
                    .is_some_and(|s| !s.is_empty())
        })
    });

    let icd_only = codings.is_some_and(|arr| {
        let has_icd = arr
            .iter()
            .any(|c| c.get("system").and_then(Value::as_str) == Some(ICD10_SYSTEM));
        let has_snomed = arr
            .iter()
            .any(|c| c.get("system").and_then(Value::as_str) == Some(SNOMED_SYSTEM));
        has_icd && !has_snomed
    });

    if icd_only {
        report.push(ValidationIssue::error(
            "Condition.code",
            "US Core Condition requires SNOMED CT; ICD-10-CM alone is a common problem-list export mistake",
            "code-snomed-required",
        ));
        return;
    }

    if !has_snomed {
        report.push(ValidationIssue::error(
            "Condition.code",
            format!("Condition.code needs SNOMED CT coding ({})", SNOMED_SYSTEM),
            "code-snomed-required",
        ));
    }
}

fn check_subject(condition: &Value, report: &mut ValidationReport) {
    let subject = condition.get("subject");
    let has_ref = subject
        .and_then(|s| s.get("reference"))
        .and_then(Value::as_str)
        .is_some_and(|r| !r.is_empty());

    if !has_ref {
        report.push(ValidationIssue::error(
            "Condition.subject",
            "subject.reference is required",
            "subject-required",
        ));
    }
}

// Resolved clinically but still marked active — shows up in problem-list reconciliation.
fn check_resolved_consistency(condition: &Value, report: &mut ValidationReport) {
    let clinical_code = condition
        .pointer("/clinicalStatus/coding")
        .and_then(|c| coding_code_in(CONDITION_CLINICAL_SYSTEM, c));

    let has_abatement = condition.get("abatementDateTime").is_some()
        || condition.get("abatementAge").is_some()
        || condition.get("abatementPeriod").is_some()
        || condition.get("abatementRange").is_some()
        || condition.get("abatementString").is_some();

    if clinical_code.as_deref() == Some("resolved") && !has_abatement {
        report.push(ValidationIssue::warning(
            "Condition.abatement",
            "clinicalStatus resolved should include abatement when known",
            "resolved-needs-abatement",
        ));
    }

    if clinical_code.as_deref() == Some("active") && has_abatement {
        report.push(ValidationIssue::warning(
            "Condition.clinicalStatus",
            "active clinicalStatus with abatement populated — check problem list hygiene",
            "active-with-abatement",
        ));
    }
}
