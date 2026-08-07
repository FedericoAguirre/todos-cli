---
description: "Implementation tasks for e2e integration tests feature"
---

# Tasks: End-to-End Integration Tests

**Input**: Design documents from `/specs/002-e2e-integration-tests/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: This feature IS the test suite (per the constitution's TDD mandate, integration tests are required). Test-writing tasks are the implementation tasks; the suite's own assertions serve as the Red phase for validating existing CLI behavior.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- All paths relative to repo root

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Scaffold the new integration test file and its imports.

- [x] T001 Create `tests/e2e_generation_test.rs` with the module imports needed (`std::fs`, `std::path::{Path, PathBuf}`, `std::process::Command`, `std::process`, `chrono::{Datelike, NaiveDate, NaiveTime}`) and a doc comment describing the suite

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared test helpers that ALL user story tests depend on.

**⚠️ CRITICAL**: No user story test can be written until this phase is complete.

- [x] T002 Implement `run_cli(year: i32, month: u32, out_dir: &Path) -> std::process::Output` helper in `tests/e2e_generation_test.rs` using `Command::new(env!("CARGO_BIN_EXE_todos-cli"))` with `--year`, `--month`, `--path` args (per research.md and `contracts/cli-test-invocation.md`)
- [x] T003 [P] Implement `temp_dir(prefix: &str) -> PathBuf` helper in `tests/e2e_generation_test.rs` that creates a unique directory under `std::env::temp_dir()` using process id + timestamp, and returns its path (per research.md)
- [x] T004 [P] Implement `count_occurrences(text: &str, pattern: &str) -> usize` helper in `tests/e2e_generation_test.rs` using `.matches(pattern).count()`
- [x] T005 Implement `expected_days(year: i32, month: u32) -> Vec<(String, String)>` helper in `tests/e2e_generation_test.rs` returning `(YYYYMMDD, SpanishWeekdayName)` pairs mirroring the `get_days()` logic (chrono leap-year February = 29 days; index a `["Lunes","Martes","Miércoles","Jueves","Viernes","Sábado","Domingo"]` array by `weekday().number_from_monday() - 1`)

**Checkpoint**: Foundation ready — user story tests can now be written in parallel.

---

## Phase 3: User Story 1 - Full Pipeline Generation in a Temporary Directory (Priority: P1) 🎯 MVP

**Goal**: Verify the CLI runs successfully against a unique temp dir and produces both `TODOS - YYYYMM.md` and `TODOS - YYYYMM.ics` side-by-side.

**Independent Test**: `cargo test --test e2e_generation_test pipeline` passes, proving exit code 0 and both files present.

### Implementation for User Story 1

- [x] T006 [US1] Add test `test_full_pipeline_generates_md_and_ics_side_by_side` in `tests/e2e_generation_test.rs` — invoke `run_cli(2026, 7, &dir)`, assert `output.status.success()`, and assert `TODOS - 202607.md` and `TODOS - 202607.ics` exist in the same temp dir (clean up dir on success)
- [x] T007 [US1] Add test `test_cli_rejects_invalid_month` in `tests/e2e_generation_test.rs` — invoke `run_cli(2026, 13, &dir)` and assert a non-zero exit code (clap validation)

**Checkpoint**: User Story 1 fully functional and testable independently.

---

## Phase 4: User Story 2 - All Seven Day Templates Render with Correct YYYYMMDD (Priority: P1)

**Goal**: Verify the markdown contains one correct `## YYYYMMDD - <Weekday>` heading per day of the month, in order, with all 7 weekday templates exercised.

**Independent Test**: `cargo test --test e2e_generation_test rendering` passes, proving every day heading matches the calendar-derived expectation.

### Implementation for User Story 2

- [x] T008 [US2] Add test `test_all_day_headings_rendered_in_order` in `tests/e2e_generation_test.rs` — generate Feb 2024, extract all `## YYYYMMDD - <Weekday>` lines, assert the count equals `expected_days(2024, 2).len()` (29), each heading's date prefix matches the expected `YYYYMMDD`, and order is ascending and contiguous
- [x] T009 [US2] Add test `test_all_seven_weekday_templates_exercised` in `tests/e2e_generation_test.rs` — generate a 31-day month (e.g., July 2026), assert the set of weekday names appearing across headings equals all 7 canonical Spanish names, and each heading's weekday matches the actual calendar weekday of that date

**Checkpoint**: User Story 2 fully functional and testable independently.

---

## Phase 5: User Story 3 - ICS Structure and VEVENT Synchronization (Priority: P1)

**Goal**: Verify the ICS has a proper `BEGIN:VCALENDAR … END:VCALENDAR` wrapper and one `VEVENT` per `- [ ]` todo item.

**Independent Test**: `cargo test --test e2e_generation_test vevent` passes, proving structure and count synchronization.

### Implementation for User Story 3

- [x] T010 [US3] Add test `test_ics_has_valid_vcalendar_structure` in `tests/e2e_generation_test.rs` — generate a month, read the ICS, assert content starts with `BEGIN:VCALENDAR` and ends with `END:VCALENDAR`
- [x] T011 [US3] Add test `test_vevent_count_matches_todo_count` in `tests/e2e_generation_test.rs` — for July 2026, assert `count_occurrences(ics, "BEGIN:VEVENT")` equals `count_occurrences(md, "- [ ] ")` (per `contracts/generated-ics.md`)

**Checkpoint**: User Stories 1–3 fully functional and testable independently.

---

## Phase 6: User Story 4 - Leap Year February Validation (Priority: P2)

**Goal**: Verify February generates 29 day sections in leap years and 28 in non-leap years, with matching VEVENT counts.

**Independent Test**: `cargo test --test e2e_generation_test february` passes for both 2024 and 2023.

### Implementation for User Story 4

- [x] T012 [US4] Add test `test_february_leap_year_2024_has_29_days` in `tests/e2e_generation_test.rs` — generate Feb 2024, assert 29 `## ` day headings from `20240201` through `20240229`, and `BEGIN:VEVENT` count equals markdown `- [ ] ` count
- [x] T013 [US4] Add test `test_february_non_leap_year_2023_has_28_days` in `tests/e2e_generation_test.rs` — generate Feb 2023, assert 28 day headings from `20230201` through `20230228`, and `BEGIN:VEVENT` count equals markdown `- [ ] ` count

**Checkpoint**: User Stories 1–4 fully functional and testable independently.

---

## Phase 7: User Story 5 - Header Generation Across Month Boundaries (Priority: P2)

**Goal**: Verify the header renders `# TODOS YYYYMM` correctly regardless of starting weekday and across month boundaries.

**Independent Test**: `cargo test --test e2e_generation_test header` passes for sample months starting on different weekdays.

### Implementation for User Story 5

- [x] T014 [US5] Add test `test_header_line_is_correct` in `tests/e2e_generation_test.rs` — generate months and assert the first non-empty line of each `.md` file is exactly `# TODOS <YYYYMM>` (e.g., `# TODOS 202402`)
- [x] T015 [US5] Add test `test_day_headings_contiguous_across_month_start_weekdays` in `tests/e2e_generation_test.rs` — generate a set of months whose first day falls on each of the 7 weekdays (e.g., 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-08), assert day headings are contiguous with no gaps or duplicates (per research.md section 5)

**Checkpoint**: User Stories 1–5 fully functional and testable independently.

---

## Phase 8: User Story 6 - Temporary Directory Cleanup (Priority: P3)

**Goal**: Verify temp dirs are removed after successful runs and preserved on failure (per Clarification 2026-08-07).

**Independent Test**: `cargo test --test e2e_generation_test cleanup` passes for both success and failure paths.

### Implementation for User Story 6

- [x] T016 [US6] Add test `test_temp_dir_removed_after_successful_run` in `tests/e2e_generation_test.rs` — run the CLI successfully, validate both files, assert `dir.exists()` is false after the cleanup call at the end of the test
- [x] T017 [US6] Add test `test_temp_dir_preserved_on_failure` in `tests/e2e_generation_test.rs` — create a temp dir, invoke the CLI with an invalid combination (e.g., `run_cli(2026, 13, &dir)`), assert a non-zero exit and that the temp dir still exists (cleanup skipped on failure)

**Checkpoint**: All user stories independently functional.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Replace the old integration test file, verify constitution gates, and confirm full-suite health.

- [x] T018 [P] Remove `tests/integration_calendar_test.rs` (its 2 cases are absorbed by the new suite — `test_cli_generates_ics` ≈ T006/T010/T011, `test_cli_calendar_for_february` ≈ T012)
- [x] T019 [P] Run `cargo clippy` and fix any warnings in `tests/e2e_generation_test.rs`
- [x] T020 [P] Run `cargo fmt --check` and fix formatting in `tests/e2e_generation_test.rs`
- [x] T021 Run `cargo test` and ensure all tests pass (existing 19 + new e2e cases)
- [x] T022 Run quickstart.md validation scenarios and verify output
- [x] T023 Verify ≥80% line coverage (via `cargo tarpaulin` or equivalent)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Stories (Phase 3–8)**: All depend on Foundational phase completion
  - User stories can proceed in parallel (each adds independent test functions in the same file)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 9)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Depends on Foundational (Phase 2) — uses `run_cli`, `temp_dir`
- **User Story 2 (P1)**: Depends on Foundational (Phase 2) — uses `expected_days` helper; independent of US1
- **User Story 3 (P1)**: Depends on Foundational (Phase 2) — uses `count_occurrences`; independent of US1/US2
- **User Story 4 (P2)**: Depends on Foundational (Phase 2); independent of US1–US3
- **User Story 5 (P2)**: Depends on Foundational (Phase 2); independent of US1–US4
- **User Story 6 (P3)**: Depends on Foundational (Phase 2); independent of US1–US5

### Within Each User Story

- Helpers (Phase 2) MUST be written before any story test
- Each story's tests are written against the current (passing) CLI behavior — they validate, not drive, production code
- Story complete before moving to next priority

### Parallel Opportunities

- T002–T004 (Foundational helpers) can run in parallel (different functions, same file — sequential editing recommended if a single editor)
- All User Story phases (T006–T017) can be written in parallel once Phase 2 is done
- Polish tasks T018–T020 can run in parallel
- User stories can be worked on in parallel by different team members, each appending test functions to `tests/e2e_generation_test.rs`

---

## Parallel Example: User Story 1

```bash
# The whole e2e suite is a single test binary; run individual story tests:
cargo test --test e2e_generation_test pipeline
cargo test --test e2e_generation_test rendering
cargo test --test e2e_generation_test vevent
cargo test --test e2e_generation_test february
cargo test --test e2e_generation_test header
cargo test --test e2e_generation_test cleanup
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001)
2. Complete Phase 2: Foundational (T002–T005)
3. Complete Phase 3: User Story 1 (T006–T007)
4. **STOP and VALIDATE**: `cargo test --test e2e_generation_test pipeline` — proves the core generation flow end-to-end
5. This alone satisfies Acceptance Criteria 1–3 (CLI runs, files created, MD rendered)

### Incremental Delivery

1. Complete Setup + Foundational → Helpers ready
2. Add User Story 1 → Independent validation → MVP (full pipeline generation)
3. Add User Story 2 → Independent validation → All-7-days rendering
4. Add User Story 3 → Independent validation → VEVENT sync
5. Add User Story 4 → Independent validation → Leap-year correctness (AC 5)
6. Add User Story 5 → Independent validation → Header/months
7. Add User Story 6 → Independent validation → Cleanup
8. Polish: remove old integration test, run constitution gates

### Parallel Team Strategy

With multiple developers:
1. Team completes Setup + Foundational together
2. Once Foundational is done, developers each take a User Story phase (T006–T007, T008–T009, T010–T011, T012–T013, T014–T015, T016–T017) in parallel, appending to `tests/e2e_generation_test.rs`
3. Integrate: run full suite, remove old integration test, run clippy/fmt/coverage

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Follow `data-model.md` for the expected-content model (day headings, counts, header line)
- Follow `contracts/generated-markdown.md` and `contracts/generated-ics.md` for exact output formats asserted
- Follow `research.md` for `CARGO_BIN_EXE` invocation and unique temp-dir strategy
- Follow the clarify decision (2026-08-07): preserve temp dir on failure, clean on success
- Production source (`src/`) MUST NOT be modified unless a test uncovers a genuine defect; if so, fix with a TDD Red-Green cycle
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
