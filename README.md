# US Core Condition / problem list (FHIR R4) — Rust

Small library and CLI that sanity-check **US Core Condition** JSON before it leaves your integration layer: **clinicalStatus** and **verificationStatus** bindings, **SNOMED** on `code`, and **problem-list-item** category.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 03** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [US Core Condition](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-condition.html)

## What it checks

- `meta.profile` includes [US Core Condition STU 6.1](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-condition.html)
- `clinicalStatus` and `verificationStatus` use HL7 condition code systems (not free text only)
- `code` includes SNOMED CT — not ICD-10 alone on the problem list
- `category` includes problem-list-item where applicable
- Hygiene warnings for resolved/active vs `abatement`

| Problem | What breaks |
|--------|-------------|
| **ICD-10 only on `code`** | Billing exports mapped to FHIR without SNOMED |
| **clinicalStatus text only** | No `condition-clinical` coding |
| **Missing verificationStatus** | Legacy problem list segments |
| **Resolved vs active** | `resolved` without abatement, or active with abatement set |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-condition -- examples/valid-problem-list.json
cargo run --bin validate-condition -- examples/invalid-icd-only-code.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-condition -- examples/valid-problem-list.json
```

**Pass** (`examples/valid-problem-list.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-clinical-status-free-text.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "Condition.clinicalStatus.coding",
    "detail": "clinicalStatus must use http://terminology.hl7.org/CodeSystem/condition-clinical (e.g. active, resolved)",
    "rule_id": "clinical-status-binding"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken payloads for tests and demos
- [docs/architecture.md](./docs/architecture.md) — short pipeline note

## Inferno vs this repo

Inferno exercises your **API** against CMS-style scenarios. This repo is about **payload shape** — problem lists that fail binding at the receiver after a green conformance run.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
