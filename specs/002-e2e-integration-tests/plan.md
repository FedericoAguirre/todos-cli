# Implementation Plan: End-to-End Integration Tests

**Branch**: `002-e2e-integration-tests` | **Date**: 2026-08-07 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/002-e2e-integration-tests/spec.md`

## Summary

Add a comprehensive end-to-end integration test suite (`tests/e2e_generation_test.rs`) that exercises the complete file generation flow: the CLI is invoked against a unique temp directory, and the resulting `TODOS - YYYYMM.md` and `TODOS - YYYYMM.ics` files are validated for (1) side-by-side creation, (2) correct rendering of all 7 weekday templates with zero-padded `YYYYMMDD` headings, (3) `VEVENT` count matching `- [ ]` todo count, (4) leap/non-leap February day counts (29/28), (5) header correctness across month boundaries, and (6) temp directory cleanup on success — preserved on failure for debugging.

## Technical Context

**Language/Version**: Rust (edition 2024)

**Primary Dependencies**: No new dependencies. Dev/test crates permitted without restriction but none required — uses `std::process::Command`, `std::fs`, and `chrono` (already a dependency).

**Storage**: Local filesystem — unique temp directories under the OS temp dir (`std::env::temp_dir()`), e.g. `/tmp/todos-e2e-*`. Cleanup on success; preserved on failure (per Clarification 2026-08-07).

**Testing**: `cargo test` (integration tests in `tests/`), `cargo clippy`, `cargo fmt --check`. Target is the existing `tests/integration_calendar_test.rs` — superseded by the new comprehensive suite.

**Target Platform**: macOS/Linux CLI, no GUI.

**Project Type**: CLI binary with a library crate (`src/lib.rs` exposes `Todos`, `create_todos_file`, parser, calendar).

**Performance Goals**: Each end-to-end test completes in under 2 seconds (CLI invocation + file validation); full e2e suite under ~15 seconds.

**Constraints**: Must not modify production code unless a defect is uncovered. Must not use fixed temp paths (risk of collision and stale state across parallel runs). Must clean up after success and preserve artifacts on failure.

**Scale/Scope**: ~6 test groups covering the 6 user stories; single new test file plus removal of the old `integration_calendar_test.rs`.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Pre-design**: ✅ PASS — all five principles clear. No new dependencies anticipated; the suite is test-only.

**Post-design (Phase 1)**: ✅ **PASS** — Research (research.md) confirmed **no new dependencies** (std + existing `chrono` only). Test-only feature; production source untouched unless a defect is found. All gates hold:

**Principle I (Minimum Dependencies)**: ✅ PASS — no new production or dev dependencies required. `chrono` already exists; the suite uses only std and existing crates.

**Principle II (Rust Best Practices)**: ✅ PASS — tests follow idiomatic Rust (helpers, `Result` handling, unique temp dirs per test).

**Principle III (TDD)**: ✅ PASS — the feature *is* tests; Red-Green is exercised against existing behavior. If a defect is found, tests are written to expose it first.

**Principle IV (Test Coverage)**: ✅ PASS — the e2e suite adds integration coverage across the full generation pipeline; existing unit coverage remains intact.

**Principle V (CLI-First Contract)**: ✅ PASS — tests invoke the actual CLI binary end-to-end, honoring the CLI contract.

**Toolchain**: ✅ PASS — no new crates added.

## Project Structure

### Documentation (this feature)

```text
specs/002-e2e-integration-tests/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── main.rs              # CLI entry point (existing — unchanged)
├── lib.rs               # Todos struct, get_days(), create_todos_file (existing — unchanged unless defect found)
├── calendar.rs          # ICS generation (existing — unchanged unless defect found)
└── parser.rs            # MD & CSV parsing (existing — unchanged unless defect found)

templates/
├── header.md            # Header template (existing)
├── 1.md … 7.md          # Day templates (existing)
└── todos_due_times.csv  # Due-time rules (existing)

tests/
├── e2e_generation_test.rs    # NEW — comprehensive end-to-end suite (this feature)
├── integration_calendar_test.rs  # REMOVED — superseded by e2e_generation_test.rs
├── calendar_test.rs          # Existing unit tests (unchanged)
└── parser_test.rs            # Existing unit tests (unchanged)
```

**Structure Decision**: Single Rust binary crate. A new integration test file `tests/e2e_generation_test.rs` hosts the comprehensive suite. The old `tests/integration_calendar_test.rs` (2 tests, fixed temp paths `/tmp/todos-test-ics` and `/tmp/todos-test-feb`) is removed and its coverage is absorbed into the new suite, which uses unique temp dirs and explicit cleanup/preservation semantics. Production source files are left untouched unless a test uncovers a defect.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No constitution violations. No complexity justification required.
