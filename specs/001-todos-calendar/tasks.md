---
description: "Implementation tasks for todos calendar feature"
---

# Tasks: Todos Calendar

**Input**: Design documents from `/specs/001-todos-calendar/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Test tasks are included following TDD per the project constitution.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- All paths relative to repo root

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization — add new modules and test harnesses.

- [x] T001 Add `pub mod calendar;` and `pub mod parser;` declarations to `src/lib.rs`
- [x] T002 [P] Create test directory structure at `tests/unit/` and `tests/integration/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Parsing infrastructure that MUST be complete before ANY user story.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T003 Create `MdParser` in `src/parser.rs` with function to parse a monthly TODOS markdown string into `Vec<DayTodos>` (date, weekday name, Vec of `(priority, description)` tuples)
- [x] T004 [P] Create `CsvParser` in `src/parser.rs` to parse `templates/todos_due_times.csv` into `Vec<DueTimeRule>` (weekday, priority, hour, alarm_minutes)
- [x] T005 Create `TodoItem` struct in `src/parser.rs` with fields: `date: NaiveDate`, `weekday_name: String`, `priority: u8`, `description: String`
- [x] T006 Create `DueTimeRule` struct in `src/parser.rs` with fields: `weekday: String`, `priority: u8`, `hour: NaiveTime`, `alarm_minutes: u16`

**Checkpoint**: Foundation ready — user story implementation can now begin in parallel.

---

## Phase 3: User Story 1 - Generate ICS Calendar Alongside Monthly TODOS (Priority: P1) 🎯 MVP

**Goal**: CLI generates a valid ICS file with one VTODO per todo item, containing SUMMARY, DTSTART, PRIORITY, and STATUS fields.

**Independent Test**: Run `cargo run -- --year 2026 --month 7 --path /tmp/todos-test` and verify `TODOS - 202607.ics` appears with correct VTODO count.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation (TDD per constitution).**

- [x] T007 [P] [US1] Unit test: `calendar` module produces valid ICS structure with `BEGIN:VCALENDAR` / `END:VCALENDAR` and correct VTODO count in `tests/calendar_test.rs`
- [x] T008 [P] [US1] Unit test: `MdParser` correctly extracts date, weekday, priority, description from sample markdown in `tests/parser_test.rs`
- [x] T009 [US1] Integration test: CLI invocation produces both `.md` and `.ics` files in `tests/integration_calendar_test.rs`

### Implementation for User Story 1

- [x] T010 [P] [US1] Implement `MdParser::parse()` in `src/parser.rs` to extract `Vec<TodoItem>` from markdown (match `## YYYYMMDD - WeekdayName` headers and `- [ ] P. Description` items)
- [x] T011 [US1] Create `IcsCalendar` builder in `src/calendar.rs` with `new()`, `add_todo()`, and `to_string()` methods
- [x] T012 [US1] Implement `IcsTodo` struct with fields: `uid`, `dtstamp`, `summary`, `dtstart`, `priority`, `status`, and corresponding ICS serialization in `src/calendar.rs`
- [x] T013 [US1] Wire ICS generation into `src/main.rs` after markdown generation — call `generate_ics()` and write `TODOS - YYYYMM.ics` alongside the `.md` file
- [x] T014 [US1] Implement `[[ ]]` bracket stripping in description in `src/parser.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently.

---

## Phase 4: User Story 2 - Due Timestamps and Alarms from CSV Lookup (Priority: P2)

**Goal**: Each VTODO includes a DUE timestamp (from CSV weekday/priority join) and a VALARM with configurable lead time.

**Independent Test**: Given a known CSV mapping (e.g., Miércoles + priority 1 = 9:00, alarm 30min), a todo on 2026-07-01 (Miércoles) with priority 1 should produce `DUE:20260701T090000` and `TRIGGER:-PT30M`.

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation.**

- [x] T015 [P] [US2] Unit test: `CsvParser` correctly parses `todos_due_times.csv` for all weekdays/priorities in `tests/parser_test.rs`
- [x] T016 [P] [US2] Unit test: Due timestamp computation matches expected formula `md.date + csv.hour` in `tests/calendar_test.rs`
- [x] T017 [P] [US2] Unit test: VALARM trigger is `-PT{csv.minutes}M` when CSV match found in `tests/calendar_test.rs`
- [x] T018 [US2] Unit test: Default due (23:59) and no alarm when CSV has no matching weekday+priority in `tests/calendar_test.rs`

### Implementation for User Story 2

- [x] T019 [P] [US2] Implement `CsvParser::parse()` in `src/parser.rs` to read `templates/todos_due_times.csv`
- [x] T020 [US2] Implement `DueTimeRule::lookup(weekday, priority) -> Option<&DueTimeRule>` in `src/parser.rs`
- [x] T021 [US2] Add `due: NaiveDateTime` and `alarm_minutes: Option<u16>` fields to `IcsTodo` via due lookup in `src/calendar.rs`
- [x] T022 [US2] Implement DUE serialization in ICS output (`DUE:YYYYMMDDTHHMMSS`) in `src/calendar.rs`
- [x] T023 [US2] Implement VALARM serialization (`BEGIN:VALARM`, `TRIGGER:-PT{M}M`, `ACTION:DISPLAY`, `END:VALARM`) in `src/calendar.rs`
- [x] T024 [US2] Implement default fallback: no CSV match → `DUE:YYYYMMDDT235959`, no VALARM in `src/calendar.rs`

**Checkpoint**: User Stories 1 AND 2 should both work independently.

---

## Phase 5: User Story 3 - ICS File Compatibility (Priority: P3)

**Goal**: ICS file is RFC 5545 compliant — line folding, content escaping, unique UIDs, correct DTSTAMP.

**Independent Test**: Generated ICS file passes RFC 5545 validation (correct CRLF, max 75 octet lines, proper escaping).

### Tests for User Story 3

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation.**

- [x] T025 [P] [US3] Unit test: ICS lines do not exceed 75 octets (folding applied) in `tests/calendar_test.rs`
- [x] T026 [P] [US3] Unit test: Special characters in descriptions are properly escaped (`\`, `;`, `,`, newline) in `tests/calendar_test.rs`
- [x] T027 [P] [US3] Unit test: Each VTODO has a unique UID in `tests/calendar_test.rs`
- [x] T028 [US3] Integration test: Empty month produces valid ICS with zero VTODOs in `tests/integration_calendar_test.rs`
- [x] T029 [US3] Integration test: Malformed CSV produces warning on stderr and ICS with defaults in `tests/integration_calendar_test.rs`

### Implementation for User Story 3

- [x] T030 [US3] Implement RFC 5545 content-line escaping (`\` → `\\`, `;` → `\;`, `,` → `\,`, newline → `\\n`) in `src/calendar.rs`
- [x] T031 [US3] Implement line folding (max 75 octets, continuation with leading space) in the ICS serializer in `src/calendar.rs`
- [x] T032 [US3] Implement UID generation (hash-based: `sha256(date + summary + priority)@todos-cli`) in `src/calendar.rs`
- [x] T033 [US3] Add CRLF line endings (`\r\n`) to ICS output in `src/calendar.rs`
- [x] T034 [US3] Add `DTSTAMP` with UTC timestamp of file generation in `src/calendar.rs`
- [x] T035 [US3] Add edge case handling: missing/unparseable CSV logs warning to stderr in `src/main.rs`
- [x] T036 [US3] Add edge case handling: malformed todo line skipped with warning in `src/parser.rs`
- [x] T037 [US3] Add edge case handling: empty month generates valid ICS with zero VTODOs in `src/calendar.rs`

**Checkpoint**: All user stories should now be independently functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, cleanup, and constitution compliance checks.

- [x] T038 [P] Run `cargo clippy` and fix any warnings
- [x] T039 [P] Run `cargo fmt --check` and fix formatting
- [x] T040 Run `cargo test` and ensure all tests pass
- [x] T041 Run quickstart.md validation scenarios and verify output
- [ ] T042 Verify ≥80% line coverage (via `cargo tarpaulin` or equivalent)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Stories (Phase 3–5)**: All depend on Foundational phase completion
  - User stories proceed sequentially in priority order (P1 → P2 → P3)
  - Each story builds on the calendar module and adds new fields
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Depends on Foundational (Phase 2) — depends on MdParser
- **User Story 2 (P2)**: Depends on US1 — adds DUE + VALARM to existing VTODO
- **User Story 3 (P3)**: Depends on US1 + US2 — polishes the serialization layer

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD per constitution)
- Parser functions before calendar generation
- Calendar types before serialization
- Core implementation before edge cases
- Story complete before moving to next priority

### Parallel Opportunities

- T001–T002 (Setup) can run in parallel
- T003–T006 (Foundational) can run in parallel
- Tests within each story marked [P] can run in parallel
- T019 can run in parallel with T020–T021 within US2
- Polish tasks T038–T039 can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
cargo test --test unit calendar_test -- ical_basic_structure
cargo test --test unit parser_test -- md_parser_extracts_fields
cargo test --test integration calendar_test -- cli_generates_ics

# Launch all implementation tasks for US1 together:
# Tasks T010, T011, T012, T014 are in different files (parser.rs, calendar.rs)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (MdParser)
3. Complete Phase 3: User Story 1 (basic ICS generation)
4. **STOP and VALIDATE**: Test US1 independently — verify `.ics` file appears with correct VTODO count
5. Deploy if ready — basic ICS without due times is still useful

### Incremental Delivery

1. Complete Setup + Foundational → Parsers ready
2. Add User Story 1 → Test independently → MVP (basic ICS!)
3. Add User Story 2 → Test independently → Due times + alarms
4. Add User Story 3 → Test independently → RFC compliant polish
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:
1. Complete Setup + Foundational together
2. Once Foundational is done, User Stories are sequential due to build-up nature
   - VTODO fields are progressively enriched across stories
   - Better suited for single-track sequential delivery

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Tests MUST fail before implementing (constitution: TDD/Red-Green-Refactor)
- Follow the data model in `data-model.md` for struct definitions
- Follow `contracts/ics-file-format.md` for exact ICS output format
- Follow `research.md` for the manual string-generation approach (no new crate)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
