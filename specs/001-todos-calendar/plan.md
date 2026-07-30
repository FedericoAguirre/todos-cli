# Implementation Plan: Todos Calendar

**Branch**: `001-todos-calendar` | **Date**: 2026-07-29 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/001-todos-calendar/spec.md`

## Summary

Generate an ICS calendar file (`TODOS - YYYYMM.ics`) alongside the existing monthly TODOS markdown file. Each VTODO contains description, date, priority, due timestamp (from CSV lookup by weekday+priority), and a VALARM with configurable lead time.

## Technical Context

**Language/Version**: Rust (edition 2021)

**Primary Dependencies**: Tera (templating), Clap (arg parsing), Chrono (dates). Candidate for new dependency: `icalendar` crate for RFC 5545 ICS generation, OR manual string-based generation.

**Storage**: Local filesystem — write `.ics` file to the same output directory as the markdown file.

**Testing**: `cargo test` (unit + integration), `cargo clippy`, `cargo fmt --check`

**Target Platform**: macOS/Linux CLI, no GUI.

**Project Type**: CLI binary.

**Performance Goals**: ICS generation completes in < 100ms for a 31-day month with 200+ todos.

**Constraints**: Must not break existing markdown generation. New dependency requires constitutional justification.

**Scale/Scope**: Single-user CLI tool. Maximum ~200 todos/month.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Pre-design**: Adding `icalendar` crate would violate Principle I (Minimum Dependencies) and Toolchain restrictions. Required justification.

**Post-design (Phase 1)**: ✅ **PASS** — Research (research.md) concluded on **manual ICS string generation**, avoiding any new production dependency. The existing approved deps (Tera, Clap, Chrono) are sufficient.

**Principle III (TDD)**: ✅ Follow Red-Green-Refactor. Unit tests for MD/CSV parsing and ICS generation must be written before implementation.

**Principle IV (Test Coverage)**: ✅ ≥80% line coverage required. Integration tests for CLI end-to-end + unit tests for parser and calendar modules.

**Toolchain**: ✅ No new crates added. All existing approved dependencies reused.

## Project Structure

### Documentation (this feature)

```text
specs/001-todos-calendar/
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
├── main.rs              # CLI entry point (existing)
├── calendar.rs          # ICS generation module (new)
├── parser.rs            # MD & CSV parsing (new)
└── templates/           # Tera templates (existing)

tests/
├── integration/
│   └── calendar_test.rs # End-to-end CLI + ICS validation
└── unit/
    ├── parser_test.rs   # MD/CSV parsing tests
    └── calendar_test.rs # ICS generation unit tests
```

**Structure Decision**: Single Rust binary crate. New modules `calendar.rs` and `parser.rs` added to `src/`. Test files mirror module structure under `tests/`.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| New dep (`icalendar`) if chosen | RFC 5545 compliance for VTODO + VALARM with proper escaping, line folding, and property encoding | Manual string generation avoids the dep but risks subtle RFC violations (escaping, line length, property parameters) and adds ~300+ LoC for something a well-tested crate handles |
