# Tasks: Events as Tasks in Calendar

**Input**: Design documents from `/specs/003-events-as-tasks/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/ics-file-format.md, quickstart.md

**Tests**: Constitution Principle III mandates Test-Driven Development (NON-NEGOTIABLE) — unit + integration tests MUST be written first (Red) and fail before implementation. All test tasks below are therefore REQUIRED, not optional.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish a clean baseline before any change; confirm the current VEVENT behavior is the starting point.

- [X] T001 [P] Verify baseline: run `cargo build && cargo test && cargo clippy` on branch `003-events-as-tasks` and confirm all existing tests pass (they currently assert VEVENT output)
- [X] T002 Review current VEVENT generation in `src/calendar.rs` (IcsEvent struct, format_ics) and every VEVENT/DTEND/STATUS assertion in `src/calendar.rs` tests, `tests/calendar_test.rs`, and `tests/e2e_generation_test.rs` to enumerate what the Red tests must flip to VTODO

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Lock in the parse-time invariant that underpins US1 (only pending todos become tasks).

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T003 [P] Add parser unit test in `tests/parser_test.rs` proving `- [x] 1. Done task` lines produce NO `TodoItem` (document FR-014 invariant; run `cargo test --test parser_test` and confirm it passes — the parser already skips checked lines)

**Checkpoint**: Foundation ready — the `- [ ]`-only parsing contract is locked by test.

---

## Phase 3: User Story 1 - Todos Appear as Tasks in Calendar Apps (Priority: P1) 🎯 MVP

**Goal**: Each pending todo is emitted as a `VTODO` task component (with `UID`, `DTSTAMP`, date-only `DTSTART;VALUE=DATE`, `SUMMARY`, `STATUS:NEEDS-ACTION`) instead of a `VEVENT` time-block event.

**Independent Test**: Generate a calendar file and assert it contains `BEGIN:VTODO` components (and zero `VEVENT`), with `DTSTART;VALUE=DATE` and `STATUS:NEEDS-ACTION` on every task.

### Tests for User Story 1 (RED — write first, ensure they FAIL) ⚠️

- [X] T004 [P] [US1] Update unit test `test_ics_uses_vevent_instead_of_vtodo` in `src/calendar.rs` → assert `VTODO` is present and `VEVENT` is absent
- [X] T005 [P] [US1] Update unit test `test_ics_uses_dtend_instead_of_due` in `src/calendar.rs` → assert `DUE:` present and `DTEND:` absent
- [X] T006 [P] [US1] Update unit test `test_ics_no_status_field` in `src/calendar.rs` → assert `STATUS:NEEDS-ACTION` is present
- [X] T007 [P] [US1] Update `test_ics_basic_structure` in `tests/calendar_test.rs` → assert `BEGIN:VTODO`/`END:VTODO`, `DTSTART;VALUE=DATE`, and no `DTEND` (drop the `Z\r\n` assertion for DTSTART)
- [X] T008 [P] [US1] Update `test_ics_vevent_count` in `tests/calendar_test.rs` → assert `BEGIN:VTODO` count equals todo count (rename test to `test_ics_vtodo_count`)

### Implementation for User Story 1

- [X] T009 [US1] Rename `IcsEvent` → `IcsTodo` (fields: uid, dtstamp, summary, date, priority, due, status, alarm_minutes) and `IcsCalendar.events` → `todos` in `src/calendar.rs`
- [X] T010 [US1] Rewrite `format_ics` in `src/calendar.rs` to emit `BEGIN:VTODO`/`END:VTODO` with `UID`, `DTSTAMP` (UTC `Z`), `DTSTART;VALUE=DATE:<YYYYMMDD>`, `SUMMARY:[P<n>] <description>` (escaped), `STATUS:NEEDS-ACTION` — remove `DTEND` output entirely

**Checkpoint**: At this point, User Story 1 is fully functional and testable independently — ICS output contains only VTODO tasks.

---

## Phase 4: User Story 2 - Due Date and Time from Existing Rules (Priority: P1)

**Goal**: Each task carries a floating-local `DUE` moment from the weekday/priority rule (or 23:59 default) and a `VALARM` anchored to the due moment.

**Independent Test**: For a known weekday+priority rule, the generated task shows `DUE:<date>T<rule.hour>` and `TRIGGER;RELATED=END:-PT<m>M`; a task with no matching rule shows `DUE:<date>T235959` and no VALARM.

### Tests for User Story 2 (RED — write first, ensure they FAIL) ⚠️

- [X] T011 [P] [US2] Update `test_event_timestamp_from_csv_match` in `tests/calendar_test.rs` → assert `DUE:20260701T090000\r\n` (floating, no `Z`) and `TRIGGER;RELATED=END:-PT30M\r\n`; remove DTEND assertions
- [X] T012 [P] [US2] Update `test_event_timestamp_no_csv_match` in `tests/calendar_test.rs` → assert `DUE:20260701T235959\r\n` and no `BEGIN:VALARM`
- [X] T013 [P] [US2] Update `test_ics_valarm_present_for_rules` in `src/calendar.rs` → assert `TRIGGER;RELATED=END:-PT30M\r\n` (update from bare `TRIGGER:-PT30M`)

### Implementation for User Story 2

- [X] T014 [US2] Compute `due: NaiveDateTime` in `generate_ics` in `src/calendar.rs`: `item.date.and_time(rule.hour)` on rule match, else `item.date.and_time(NaiveTime::from_hms_opt(23, 59, 59))`; store in IcsTodo
- [X] T015 [US2] Emit `DUE` in `format_ics` in `src/calendar.rs` formatted as `%Y%m%dT%H%M%S` (floating local, no timezone suffix)
- [X] T016 [US2] Emit `VALARM` in `format_ics` in `src/calendar.rs` with `TRIGGER;RELATED=END:-PT{m}M` when `alarm_minutes` is `Some`; omit entirely when `None`

**Checkpoint**: At this point, User Stories 1 AND 2 both work independently — tasks are checkable AND carry correct due moments/reminders.

---

## Phase 5: User Story 3 - Compatibility with Android, Mac, and Other ICS Processors (Priority: P2)

**Goal**: The generated file remains RFC 5545-compliant (escaping, folding ≤75 octets, CRLF, valid VCALENDAR wrapper) and imports into task-capable processors; e2e suite stays synchronized to VTODO.

**Independent Test**: Generate a month, count `BEGIN:VTODO` vs `- [ ] ` in the markdown (equal), and verify the file parses with zero structural errors and only VTODO components.

### Tests for User Story 3 (RED — write first, ensure they FAIL) ⚠️

- [X] T017 [P] [US3] Update `test_vevent_count_matches_todo_count` in `tests/e2e_generation_test.rs` → count `BEGIN:VTODO` against `- [ ] ` (rename test to `test_vtodo_count_matches_todo_count`)
- [X] T018 [P] [US3] Update `test_february_leap_year_2024_has_29_days` in `tests/e2e_generation_test.rs` → assert `BEGIN:VTODO` count matches md todo count
- [X] T019 [P] [US3] Update `test_february_non_leap_year_2023_has_28_days` in `tests/e2e_generation_test.rs` → assert `BEGIN:VTODO` count matches md todo count
- [X] T020 [P] [US3] Add test in `tests/calendar_test.rs` asserting the output contains NO `VEVENT` and exactly `N` `VTODO` components for `N` todos (component-type exclusivity)

### Implementation for User Story 3

- [X] T021 [US3] Confirm `DTSTAMP` remains UTC (`%Y%m%dT%H%M%SZ`) while `DUE`/`DTSTART` carry no timezone in `format_ics` in `src/calendar.rs`
- [X] T022 [US3] Confirm `escape_ics` and `fold_lines` are applied unchanged to the VTODO output in `src/calendar.rs` (escaping `\ ; , \n`; folding ≤75 octets with CRLF)

**Checkpoint**: The calendar file is structurally valid, importable into task-capable processors, and synchronized with the markdown.

---

## Phase 6: User Story 4 - Regeneration and Re-import (Priority: P3)

**Goal**: Tasks keep stable, unique identities across regenerations; identical-looking todos remain distinct.

**Independent Test**: Generate the same month twice → identical UIDs per task; two todos with the same date+description+priority → distinct UIDs.

### Tests for User Story 4 (RED — write first, ensure they FAIL) ⚠️

- [X] T023 [P] [US4] Add unit test in `src/calendar.rs` asserting two identical todos (same date, description, priority) produce DIFFERENT UIDs
- [X] T024 [P] [US4] Add unit test in `src/calendar.rs` asserting `generate_ics` called twice with identical input yields identical UID sets (regeneration stability)

### Implementation for User Story 4

- [X] T025 [US4] Extend `generate_uid` in `src/calendar.rs` to incorporate a per-item ordinal (index within the items list) so identical-looking todos get distinct deterministic UIDs without breaking stability for unchanged inputs

**Checkpoint**: All user stories are now independently functional.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Update consumers of the event-style output and run final quality gates.

- [X] T026 [P] Update the "Todos Calendar" section of `README.md` to document `VTODO` task output (fields, DUE logic, RELATED=END alarm) replacing the VEVENT/DTEND explanation
- [X] T027 [P] Update `specs/002-e2e-integration-tests/contracts/generated-ics.md` → replace VEVENT/DTEND with VTODO/DUE, update count expectations note
- [X] T028 Verify `specs/001-todos-calendar/contracts/ics-file-format.md` aligns with the new VTODO format (it already documents VTODO — confirm no drift)
- [X] T029 Run `cargo fmt --check` and `cargo clippy` — fix all warnings so both are clean
- [X] T030 Run the full `cargo test` suite and confirm all Red tests from T003–T024 now pass (Green)
- [X] T031 Run quickstart.md validation scenarios in `specs/003-events-as-tasks/quickstart.md` (cargo run + grep checks + RFC 5545 structural validation)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational — then sequential in priority order (US1 → US2 → US3 → US4)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: None beyond Foundational — the MVP core change (all VTODO structure work is here)
- **User Story 2 (P1)**: Depends on US1 (IcsTodo must exist before DUE/VALARM fields are added); independently testable afterward
- **User Story 3 (P2)**: Depends on US1; overlaps with US2 only in shared `src/calendar.rs` (format_ics) — schedule after US2
- **User Story 4 (P3)**: Depends on US1 (UID emit path); touches `generate_uid`/`generate_ics` which US2 also touches — schedule after US2

### Within Each User Story

- Tests (RED) MUST be written and FAIL before implementation
- Data model before formatting; formatting before integration assertions

### Parallel Opportunities

- T001 and T002 (Phase 1) run in parallel
- T003 (Phase 2) is standalone
- Within US1: T004–T008 (tests) run in parallel; T009 then T010 sequential
- Within US2: T011–T013 (tests) run in parallel; T014 → T015 → T016 sequential
- Within US3: T017–T020 (tests) run in parallel; T021 and T022 sequential after tests
- Within US4: T023 and T024 (tests) run in parallel; T025 after
- Polish: T026, T027, T028 run in parallel; T029, T030, T031 sequential after

---

## Parallel Example: User Story 1

```bash
# Launch all Red tests for User Story 1 together:
Task: "Update test_ics_uses_vevent_instead_of_vtodo in src/calendar.rs"
Task: "Update test_ics_uses_dtend_instead_of_due in src/calendar.rs"
Task: "Update test_ics_no_status_field in src/calendar.rs"
Task: "Update test_ics_basic_structure in tests/calendar_test.rs"
Task: "Update test_ics_vevent_count in tests/calendar_test.rs"

# Then launch implementation sequentially:
Task: "Rename IcsEvent → IcsTodo in src/calendar.rs"
Task: "Rewrite format_ics to emit VTODO in src/calendar.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: `cargo test` — VTODO structure tests green; ICS contains no VEVENT
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → foundation locked
2. Add User Story 1 → test independently → demo (MVP: tasks appear as checkable VTODO)
3. Add User Story 2 → test independently → demo (due moments + reminders work)
4. Add User Story 3 → test independently → demo (e2e + RFC 5545 compliance)
5. Add User Story 4 → test independently → demo (stable identities)

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: US1 + US2 (same file `src/calendar.rs`)
   - Developer B: US3 e2e/contract tests (`tests/`)
   - Developer C: US4 UID tests + docs (`src/calendar.rs` after US2, `README.md`)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies (CAUTION: T004–T006, T011–T013, T023–T024 all edit `src/calendar.rs` — only run the ones touching different test functions in parallel; do NOT run Red tests for US2/US4 before US1 implementation lands or they may not compile)
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Verify tests fail (Red) before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same-function conflicts, cross-story dependencies that break independence