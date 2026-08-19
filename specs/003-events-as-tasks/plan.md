# Implementation Plan: Events as Tasks in Calendar

**Branch**: `003-events-as-tasks` | **Date**: 2026-08-18 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/003-events-as-tasks/spec.md`

## Summary

The generated ICS file currently emits one `VEVENT` per todo (an event that occupies a 1-hour time block sampled from `todos_due_times.csv`). This feature converts the calendar representation from events to **tasks** (`VTODO` components): each todo becomes a checkable task pinned to its day via a date-only `DTSTART;VALUE=DATE`, with a time-based `DUE`, an RFC 5545 `PRIORITY`, a constant `STATUS:NEEDS-ACTION`, and an optional `VALARM` reminder. Checked-off markdown todos (`- [x]`) already produce no task (the parser only reads `- [ ]`), satisfying FR-014 with no parser change.

Research confirmed the format approach: date-only `DTSTART` plus floating local-time `DUE` avoids the spurious timezone-shifted reminders, and `TRIGGER;RELATED=END:-PT{n}M` anchors alarms to the due moment per RFC 5545 §3.6.2/§3.8.6.3. No new production dependency is required — manual ICS string generation continues (Constitution Principle I).

## Technical Context

**Language/Version**: Rust (edition 2024; rustc 1.85+, matching existing toolchain)

**Primary Dependencies**: chrono `0.4.x` (dates, `Utc::now()` for DTSTAMP), clap `4.x` (CLI, unchanged), Tera `1.x` (templates, unchanged)

**Storage**: Filesystem — generated `TODOS - YYYYMM.md` and `TODOS - YYYYMM.ics` in the same output directory

**Testing**: `cargo test` (unit + integration), `cargo clippy` zero warnings, `cargo fmt --check`

**Target Platform**: macOS and Android calendar applications + RFC 5545-compliant ICS processors

**Project Type**: Rust CLI tool

**Performance Goals**: N/A (single-shot file generation; existing <1s budget unchanged)

**Constraints**: Constitution — only tera/clap/chrono in production deps; ≥80% line coverage; TDD (red-green-refactor); RFC 5545 line folding ≤75 octets + CRLF + escaping

**Scale/Scope**: ~155 todos/month (5/day × 31 days max) — trivial volume, no perf concerns

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Minimum Dependencies | PASS | No new production dependency: manual ICS generation continues |
| II. Rust Best Practices | PASS | `Result`/`Option` retained; clippy/fmt clean as part of gates |
| III. TDD (NON-NEGOTIABLE) | PASS | Red tests written first (VTODO assertions), then implementation |
| IV. Test Coverage ≥ 80% | PASS | Existing suite + updated VTODO assertions keep coverage high |
| V. CLI-First Contract | PASS | No CLI changes; `--path` / `TODOS_DEFAULT_PATH` behavior untouched |
| Toolchain whitelist | PASS | Only permitted crates in production tree |

No gate violations. No Complexity Tracking required.

## Project Structure

### Documentation (this feature)

```text
specs/003-events-as-tasks/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output — timezone/format decisions
├── data-model.md        # Phase 1 output — entities (IcsTodo replaces IcsEvent)
├── quickstart.md        # Phase 1 output — validation scenarios
├── contracts/           # Phase 1 output — ICS VTODO file-format contract
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── main.rs              # CLI entry; unchanged
├── lib.rs               # lib facade; unchanged
├── calendar.rs          # CHANGED: IcsEvent → IcsTodo, VEVENT output → VTODO output
└── parser.rs            # UNCHANGED (already skips - [x]); add test for checked-off exclusion

tests/
├── calendar_test.rs     # CHANGED: VEVENT/DTEND/UTC assertions → VTODO/DUE/date-only assertions
├── e2e_generation_test.rs  # CHANGED: BEGIN:VEVENT counts → BEGIN:VTODO counts
└── parser_test.rs       # CHANGED: add test: - [x] lines produce no TodoItem
```

**Structure Decision**: Single-project layout (existing). All structural change is confined to `src/calendar.rs` plus the three test files; `main.rs`, `lib.rs`, and `parser.rs` logic are untouched. Downstream docs that describe event-style output (README "Todos Calendar" section, `specs/001-todos-calendar/contracts/ics-file-format.md`, `specs/002-e2e-integration-tests/contracts/generated-ics.md`) are updated in the implementation phase to describe task output.

## Complexity Tracking

> No constitution violations to justify. Section intentionally empty.