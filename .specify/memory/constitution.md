<!-- Sync Impact Report
  Version change: (template) → 1.0.0
  Modified principles: N/A (all new)
  Added sections: Core Principles (5), Toolchain & Technology Stack, Development Workflow & Quality Gates, Governance
  Removed sections: N/A
  Follow-up TODOs: None
-->

# todos-cli Constitution

## Core Principles

### I. Minimum Dependencies
Every crate in `Cargo.toml` MUST be justified by a direct feature requirement.
Transient dependency proliferation MUST be avoided — prefer `default-features = false`
and selective feature flags. Rationale: A small dependency surface reduces build
times, audit burden, and the risk of supply-chain vulnerabilities.

### II. Rust Best Practices
Code MUST follow idiomatic Rust: use `Result`/`Option` for fallible operations,
leverage the type system to make invalid states unrepresentable, and prefer
iterator combinators and pattern matching over manual loops where appropriate.
All code MUST pass `cargo clippy` with no warnings and MUST be formatted with
`cargo fmt`. Rationale: Consistent, idiomatic code is easier to review, maintain,
and onboard contributors to.

### III. Test-Driven Development (NON-NEGOTIABLE)
All feature work MUST follow the Red-Green-Refactor cycle:
1. Write a failing test (Red) — approved before implementation begins.
2. Write the minimum implementation to pass (Green).
3. Refactor while keeping tests green (Refactor).
Both **unit tests** (for individual functions/modules) and **integration tests**
(for end-to-end CLI invocation and file generation) MUST be present. Rationale:
TDD ensures every line of code is tested and the API is designed for testability
from the start.

### IV. Test Coverage
Test coverage MUST be at least **80%** measured at the line level. CI MUST
reject any change that drops coverage below this threshold. Coverage gaps in
error-handling paths or edge cases MUST be documented with a `// coverage:ignore`
comment and rationale. Rationale: High coverage gives confidence during
refactoring and prevents regressions in the generated output.

### V. CLI-First Contract
The application entry point MUST be a CLI binary using `clap` for argument
parsing. All user-facing output MUST go to stdout (machine-readable) or stderr
(logs/diagnostics). The `--path` argument MUST fall back to the
`TODOS_DEFAULT_PATH` environment variable when omitted. Rationale: A strict CLI
contract keeps the tool scriptable, composable, and predictable.

## Toolchain & Technology Stack

Only the following crates are permitted in the production dependency tree
(additional dev-dependencies for testing are allowed without restriction):

- **Tera** (`1.x`) — Templating engine for generating the monthly TODOS file.
- **Clap** (`4.x`, `derive` feature) — Argument parsing.
- **Chrono** (`0.4.x`, `std` feature) — Date and calendar arithmetic.

Any new production dependency MUST be proposed with a written justification
in a PR before addition. Rationale: Constraining the stack keeps the tool
lightweight, fast to compile, and easy to audit.

## Development Workflow & Quality Gates

Every PR MUST pass the following checks before merge:

1. **Build**: `cargo build` succeeds with no errors or warnings.
2. **Tests**: `cargo test` passes all unit and integration tests.
3. **Lint**: `cargo clippy` produces zero warnings.
4. **Format**: `cargo fmt --check` confirms consistent formatting.
5. **Coverage**: Line coverage ≥ 80% (measured via `cargo tarpaulin` or
   equivalent tool).

Release builds MUST use the `release` profile defined in `Cargo.toml`
(currently: optimise for size, strip debuginfo, LTO, abort on panic).
Rationale: Automated gates enforce consistency without manual review overhead.

## Governance

This Constitution is the governing document for the todos-cli project. It
supersedes all informal practices, conventions, and prior agreements.

- **Amendments**: Changes to this Constitution MUST be proposed as a PR that
  modifies `.specify/memory/constitution.md`, includes a Sync Impact Report
  comment, and bumps the version per semantic versioning rules.
- **Versioning Policy**:
  - MAJOR: Backward-incompatible governance changes (principle removals or
    redefinitions).
  - MINOR: New principle/section added or materially expanded guidance.
  - PATCH: Clarifications, wording, typo fixes, non-semantic refinements.
- **Compliance Review**: Every PR MUST verify compliance with all principles.
  Reviewers are responsible for flagging violations. The `AGENTS.md` file
  serves as runtime guidance for agent-assisted development.

**Version**: 1.0.0 | **Ratified**: 2026-07-29 | **Last Amended**: 2026-07-29
