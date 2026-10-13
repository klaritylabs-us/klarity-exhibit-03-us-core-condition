//! US Core Condition STU 6.1 constants.

pub const US_CORE_CONDITION_PROFILE: &str =
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-condition";

pub const SNOMED_SYSTEM: &str = "http://snomed.info/sct";

pub const CONDITION_CATEGORY_SYSTEM: &str =
    "http://terminology.hl7.org/CodeSystem/condition-category";

pub const PROBLEM_LIST_CATEGORY: &str = "problem-list-item";

pub const CONDITION_CLINICAL_SYSTEM: &str =
    "http://terminology.hl7.org/CodeSystem/condition-clinical";

pub const CONDITION_VER_STATUS_SYSTEM: &str =
    "http://terminology.hl7.org/CodeSystem/condition-ver-status";

/// Example SNOMED for Type 2 diabetes mellitus (fixtures).
pub const SNOMED_TYPE2_DIABETES: &str = "44054006";

/// ICD-10-CM often appears alongside SNOMED in real pipelines — but not as a substitute.
pub const ICD10_SYSTEM: &str = "http://hl7.org/fhir/sid/icd-10-cm";
