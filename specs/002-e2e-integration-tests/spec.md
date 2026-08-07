# Feature Specification: End-to-End Integration Tests

**Feature Branch**: `002-e2e-integration-tests`

**Created**: 2026-08-07

**Status**: Draft

**Input**: User description: "As a QA developer I want an end-to-end integration test that validates complete file generation flow including: MD and ICS files created side-by-side in temp directory; Template rendering for all 7 days (Monday-Sunday) correctly using YYYYMMDD placeholders from get_days(); VEVENT count matching total todo items (- [ ]) between .md and .ics outputs; Correct date handling including leap year February validation; Proper header generation across month boundaries; Cleanup of temp test directories after successful runs."

## Clarifications

### Session 2026-08-07

- Q: When an integration test run fails partway through, should the temporary output directory be preserved for debugging, or removed regardless of the outcome? → A: Preserve temp dir on failure for debugging; clean up only after successful validation.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Full Pipeline Generation in a Temporary Directory (Priority: P1)

As a QA developer, I run the CLI with a valid year, month, and a `--path` pointing to a fresh temporary directory, and I verify the tool exits without error and produces both the monthly markdown file (`TODOS - YYYYMM.md`) and the calendar file (`TODOS - YYYYMM.ics`) side-by-side in that directory.

**Why this priority**: This is the core end-to-end flow. If the CLI cannot generate both outputs successfully, no other validation can proceed.

**Independent Test**: Run the CLI against a newly created temp directory with valid inputs and assert exit code 0 plus the presence of both expected output files with the correct names.

**Acceptance Scenarios**:

1. **Given** a valid year and month, **When** the CLI runs with `--path` pointing to a temporary directory, **Then** it exits with status code 0.
2. **Given** the CLI runs successfully, **Then** a file named `TODOS - YYYYMM.md` exists in the temporary directory.
3. **Given** the CLI runs successfully, **Then** a file named `TODOS - YYYYMM.ics` exists in the same temporary directory.

---

### User Story 2 - All Seven Day Templates Render with Correct YYYYMMDD (Priority: P1)

As a QA developer, I want the generated markdown to contain one day section for every date in the target month, each rendered through the correct weekday template (Monday through Sunday) with the correct zero-padded `YYYYMMDD` value substituted, so that I know template rendering is correct end-to-end.

**Why this priority**: Rendering is the primary feature of the tool; verifying every weekday template and every placeholder substitution is essential.

**Independent Test**: Generate a month with a known starting weekday (e.g., February 2024 starts on a Thursday) and assert that each expected `## YYYYMMDD - <Weekday>` heading appears exactly once, in the correct order, with the correct Spanish weekday name.

**Acceptance Scenarios**:

1. **Given** a valid target month, **When** the markdown file is generated, **Then** it contains exactly one day section per date in that month (28, 29, 30, or 31 depending on the month and year).
2. **Given** a generated day section, **When** the template placeholder is substituted, **Then** the heading uses the zero-padded `YYYYMMDD` format (e.g., `## 20240201 - Jueves`).
3. **Given** a full month, **When** all day templates are exercised, **Then** each day heading uses the correct Spanish weekday name matching the template number (1.md = Lunes, ..., 7.md = Domingo) rotated continuously across the month.

---

### User Story 3 - ICS Structure and VEVENT Synchronization (Priority: P1)

As a QA developer, I want the generated ICS file to have a proper `BEGIN:VCALENDAR ... END:VCALENDAR` structure and to contain exactly one `VEVENT` per unchecked todo item (`- [ ]`) found in the markdown file, so that the two outputs stay synchronized.

**Why this priority**: The calendar is only useful if it reflects exactly what the markdown contains. Count synchronization is the key correctness check.

**Independent Test**: Generate a month, count the `- [ ]` occurrences in the markdown and the `BEGIN:VEVENT` blocks in the ICS, and assert the two counts are equal.

**Acceptance Scenarios**:

1. **Given** a generated ICS file, **When** its content is inspected, **Then** it begins with `BEGIN:VCALENDAR` and ends with `END:VCALENDAR`.
2. **Given** a markdown file containing N unchecked todo items, **When** the ICS is generated, **Then** it contains exactly N `BEGIN:VEVENT` blocks.
3. **Given** a month with no todo items, **When** the ICS is generated, **Then** it is still a valid calendar with zero `VEVENT` components.

---

### User Story 4 - Leap Year February Validation (Priority: P2)

As a QA developer, I want the tool to generate the correct number of day sections for February in both leap and non-leap years, so that calendar arithmetic is validated (29 days in leap years, 28 in non-leap years).

**Why this priority**: Date handling is a core correctness concern and February is the only month whose length varies by year.

**Independent Test**: Generate February 2024 (leap year) and February 2023 (non-leap year), asserting the markdown contains 29 and 28 day headings respectively, and that the corresponding VEVENT counts stay synchronized with the todo counts.

**Acceptance Scenarios**:

1. **Given** February of a leap year (e.g., 2024), **When** the files are generated, **Then** the markdown contains 29 day headings (from `20240201` through `20240229`).
2. **Given** February of a non-leap year (e.g., 2023), **When** the files are generated, **Then** the markdown contains 28 day headings (from `20230201` through `20230228`).
3. **Given** the generated files for any February, **When** counts are compared, **Then** the ICS `VEVENT` count equals the markdown todo count.

---

### User Story 5 - Header Generation Across Month Boundaries (Priority: P2)

As a QA developer, I want the header template to render correctly regardless of which weekday a month starts on and across month-year transitions, so that header generation is validated for all boundary conditions.

**Why this priority**: Month starts vary across the 7 weekdays, and a wrong header would corrupt the whole file.

**Independent Test**: Generate months that start on each of the seven weekdays (and a December-to-January transition), asserting the header line is always correct and no rendering errors occur.

**Acceptance Scenarios**:

1. **Given** any valid month, **When** the markdown is generated, **Then** the first line is the header rendered from the header template with the correct `YYYYMM` value.
2. **Given** months starting on each of the seven weekdays, **When** they are generated, **Then** all day sections are contiguous and correctly ordered with no missing or duplicated headings.

---

### User Story 6 - Temporary Directory Cleanup (Priority: P3)

As a QA developer, I want the temporary output directories used by the integration tests to be removed after successful runs — but preserved on failure for debugging — so that repeated test runs do not accumulate stale artifacts while still leaving evidence when something goes wrong.

**Why this priority**: Hygiene matters for repeatable, reliable test suites, but does not affect functional correctness.

**Independent Test**: Run the test, then assert the temporary directory no longer exists on disk after the run completes successfully; on a failing run, assert the directory is retained.

**Acceptance Scenarios**:

1. **Given** a test run that completes successfully, **When** the test finishes, **Then** the temporary output directory no longer exists.
2. **Given** a test run that fails partway through, **When** the test finishes, **Then** the temporary output directory is preserved for debugging.
3. **Given** generated files that pass all validation checks, **When** cleanup executes, **Then** cleanup only occurs after file validation has succeeded.

---

### Edge Cases

- What happens when the year/month combination is invalid (e.g., month 13)? — The CLI argument validation rejects it before generation; the test asserts a non-zero exit code.
- What happens in February of a leap century year (e.g., 1900 = non-leap, 2000 = leap)? — The day count follows the calendar rule (28 and 29 respectively), consistent with `get_days()`.
- What happens when a month has no todo items? — The ICS remains valid with zero `VEVENT` components and the markdown still contains all day sections.
- What happens across a month-year boundary (e.g., December 2026 to January 2027)? — The header uses the correct `YYYYMM` value for the target month, independent of the previous month.
- What happens when the CLI fails mid-generation? — The temporary directory is preserved as-is so QA can inspect partial output; the test fails and does not clean up.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The integration test MUST execute the CLI binary with valid `--year`, `--month`, and `--path` arguments pointing to a temporary directory.
- **FR-002**: The test MUST assert that the CLI exits with status code 0 for valid year/month inputs.
- **FR-003**: The test MUST assert that both `TODOS - YYYYMM.md` and `TODOS - YYYYMM.ics` are created in the same temporary output directory.
- **FR-004**: The test MUST assert that the markdown file contains one day section for every date in the target month, matching the days returned by the tool's date calculation.
- **FR-005**: The test MUST assert that each day heading is rendered with the correct zero-padded `YYYYMMDD` value.
- **FR-006**: The test MUST assert that each day heading uses the correct Spanish weekday name consistent with the day template (1.md = Lunes through 7.md = Domingo) and the actual date.
- **FR-007**: The test MUST assert that the ICS file starts with `BEGIN:VCALENDAR` and ends with `END:VCALENDAR`.
- **FR-008**: The test MUST assert that the number of `BEGIN:VEVENT` blocks in the ICS equals the number of `- [ ]` todo items in the markdown file.
- **FR-009**: The test MUST verify that February of a leap year generates 29 day sections and February of a non-leap year generates 28 day sections.
- **FR-010**: The test MUST verify correct header generation for months starting on each of the seven weekdays, including a month boundary transition.
- **FR-011**: The test MUST remove the temporary output directory after all generated files have been successfully validated, and MUST preserve it when validation or generation fails.
- **FR-012**: The test MUST run within the standard test harness without requiring manual setup or external services.

### Key Entities *(include if feature involves data)*

- **MonthDay**: A single date within the target month (YYYYMMDD) with its Spanish weekday name; the unit rendered by each day template.
- **TodoItem**: A task parsed from the markdown (`- [ ] <priority>. <description>`), one per `VEVENT` in the ICS.
- **GeneratedMarkdown**: The `TODOS - YYYYMM.md` file — header template output followed by one section per day.
- **GeneratedICS**: The `TODOS - YYYYMM.ics` file — a `VCALENDAR` wrapper containing one `VEVENT` per todo item.
- **VEVENT**: An iCalendar event component; its count must always equal the markdown todo item count.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The CLI exits with status code 0 for every valid (year, month) pair exercised by the integration tests.
- **SC-002**: The generated markdown renders exactly the number of day sections returned by the tool's date calculation (28/29/30/31) with zero missing or duplicated headings.
- **SC-003**: The ICS is structurally valid (`BEGIN:VCALENDAR ... END:VCALENDAR`) and its `VEVENT` count equals the markdown `- [ ]` count for every month exercised.
- **SC-004**: February runs produce 29 day sections in leap years and 28 in non-leap years, with matching `VEVENT` counts.
- **SC-005**: Temporary test directories are cleaned up after every successful run — no leftover artifacts remain on disk.

## Assumptions

- Acceptance criterion 5 in the user description reads "29 items in non-leap, 28 in leap years". This is treated as a typographical error; the correct calendar behavior — consistent with the existing `get_days()` tests — is 29 days in a leap year and 28 in a non-leap year.
- The ICS format uses `VEVENT` components (not `VTODO`), consistent with the current implementation and README documentation.
- The integration test uses the standard test harness (integration tests run via `cargo test`) with temporary directories created per run; no external services, network access, or fixtures are required.
- Cleanup executes after file validation succeeds; on test failure, the temporary output directory is preserved for debugging (see Clarifications 2026-08-07).
- The tests run against the current day templates (5 todo items per day), so total todo counts are deterministic per month.
