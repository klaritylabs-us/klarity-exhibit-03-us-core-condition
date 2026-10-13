# Architecture

```text
Condition.json
  → parse
  → meta.profile (us-core-condition)
  → clinicalStatus + verificationStatus (HL7 condition CodeSystems)
  → category (problem-list-item warning if missing)
  → code (SNOMED required; flag ICD-only)
  → subject.reference
  → resolved / abatement consistency warnings
  → ValidationIssue JSON from CLI
```
