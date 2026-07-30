# Feature Specification: Todos Calendar

**Feature Branch**: `001-todos-calendar`

**Created**: 2026-07-29

**Status**: Draft

**Input**: User description: "using ai/features/todos/001_todos_calendar.md create the new feature"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Generate ICS Calendar Alongside Monthly TODOS (Priority: P1)

As a todos-cli user, I run the tool with `--year` and `--month`, and both the monthly TODOS markdown file and an ICS calendar file are generated. The ICS file contains one VTODO component per todo item from the markdown file, so I can import my todos into any calendar app.

**Why this priority**: This is the core value of the feature — generating a usable calendar file. Without it, there is no feature.

**Independent Test**: Can be fully tested by running the CLI with valid `--year 2026 --month 7` and verifying that a `TODOS - 202607.ics` file appears alongside `TODOS - 202607.md` with the correct number of VTODO entries.

**Acceptance Scenarios**:

1. **Given** a valid year and month, **When** the CLI runs, **Then** a `TODOS - YYYYMM.ics` file is generated in the same directory as the markdown file.
2. **Given** `TODOS_DEFAULT_PATH` is set and no `--path` is provided, **When** the CLI runs, **Then** the ICS file is written to the default path.
3. **Given** a month with N total todos across all days, **When** the ICS file is generated, **Then** it contains exactly N `VTODO` components.
4. **Given** a todo with description wrapped in `[[ ]]`, **When** the VTODO is generated, **Then** the brackets are removed and only the inner text is used as the description.

---

### User Story 2 - Due Timestamps and Alarms from CSV Lookup (Priority: P2)

As a todos-cli user, I want each VTODO to have a correct due timestamp and an alarm, computed by matching the weekday and priority of each todo against a CSV configuration file, so that my calendar accurately reflects my planned schedule and reminds me before each task is due.

**Why this priority**: The due timestamp and alarm are what make the ICS file useful for scheduling. Without them, the calendar would only contain task titles with no temporal structure.

**Independent Test**: Can be tested by providing a known monthly TODOS file and CSV, then verifying that the ICS output has the expected `DUE` and `VALARM` values for each VTODO.

**Acceptance Scenarios**:

1. **Given** a todo on weekday W with priority P, **When** the due timestamp is computed, **Then** it equals `md.date + csv.hour` where `csv.weekday = W` and `csv.priority = P`.
2. **Given** a todo with a matching CSV row, **When** the VTODO alarm is generated, **Then** it triggers `csv.minutes` minutes before the due timestamp.
3. **Given** a todo whose weekday and priority have no match in the CSV, **When** the VTODO is generated, **Then** no alarm is attached and the due timestamp defaults to the end of that day (23:59).
4. **Given** all todos in a day, **When** the ICS is generated, **Then** each VTODO has status `NEEDS-ACTION`.

---

### User Story 3 - ICS File Compatibility (Priority: P3)

As a todos-cli user, I want the generated ICS file to be valid according to the iCalendar specification (RFC 5545), so that it can be imported into major calendar applications without errors.

**Why this priority**: The file is only useful if it actually works in calendar apps. Validation ensures compatibility.

**Independent Test**: Can be tested by running the generated ICS file through an RFC 5545 validator or by importing it into a calendar application and confirming all VTODOs appear correctly.

**Acceptance Scenarios**:

1. **Given** a generated ICS file, **When** parsed by an iCalendar validator, **Then** it produces no errors.
2. **Given** a generated ICS file, **When** imported into a calendar application, **Then** all VTODOs appear with correct descriptions, due dates, and alarms.

---

### Edge Cases

- What happens when the CSV file is missing or unparseable? — The CLI SHOULD emit a warning on stderr and generate the ICS file with default due timestamps (end of day) and no alarms.
- What happens when a todo line does not match the expected format (`- [ ] P. text`)? — The CLI SHOULD skip the malformed line, emit a warning, and continue processing remaining todos.
- What happens when the month has no days with todos? — The CLI SHOULD generate a valid ICS file with zero VTODO components.
- What happens when the year/month combination is invalid (e.g., month 13)? — The existing argument validation SHOULD reject it before generation; no ICS-specific handling needed.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The CLI MUST generate an ICS file (`.ics`) whenever it generates the monthly TODOS markdown file.
- **FR-002**: The ICS file MUST be named `TODOS - YYYYMM.ics`, placed in the same output directory as the markdown file.
- **FR-003**: The ICS file MUST contain exactly one `VTODO` component per todo item from the monthly file.
- **FR-004**: Each `VTODO` MUST include a `SUMMARY` derived from the todo description with `[[ ]]` brackets removed.
- **FR-005**: Each `VTODO` MUST include a `DTSTART` set to the todo's date.
- **FR-006**: Each `VTODO` MUST include a `PRIORITY` matching the numeric priority from the monthly file.
- **FR-007**: Each `VTODO` MUST include a `DUE` timestamp computed from `md.date + csv.hour` when a matching CSV row exists.
- **FR-008**: Each `VTODO` MUST include a `VALARM` with `TRIGGER` set to `-PT{csv.minutes}M` when a matching CSV row exists.
- **FR-009**: Each `VTODO` MUST have `STATUS` set to `NEEDS-ACTION`.
- **FR-010**: The system MUST read `templates/todos_due_times.csv` to obtain the weekday/priority → hour/minute mapping.
- **FR-011**: When a CSV row is missing for a given weekday + priority combination, the system MUST default the due timestamp to 23:59 of that day and omit the alarm.
- **FR-012**: The month's weekday name (e.g., "Miércoles") MUST be resolved from the date using the `chrono` crate with Spanish locale.
- **FR-013**: When the CSV file is missing or unparseable, the system MUST log a warning on stderr and proceed with default values (end-of-day due, no alarms).

### Key Entities *(include if feature involves data)*

- **MonthDay**: A single day within the target month, containing a date (YYYYMMDD), a Spanish weekday name, and a list of Todo items. Extracted from the TODOS markdown file.
- **Todo**: A single task item within a MonthDay, with a description, a numeric priority (1-indexed), and optional `[[ ]]` brackets for linking.
- **DueTimeRule**: A row from the CSV file mapping a weekday + priority combination to a due hour:minute and an alarm minutes-before value.
- **ICS Calendar**: The resulting iCalendar object containing VCALENDAR, VTIMEZONE (if needed), and one VTODO per Todo item.
- **VTODO**: An iCalendar component representing a task, with fields for SUMMARY, DTSTART, PRIORITY, DUE, STATUS, and VALARM.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The ICS file is generated in under 1 second for any valid month input (including months with 31 days and high todo density).
- **SC-002**: The generated ICS file passes RFC 5545 validation with zero errors for any valid monthly TODOS file.
- **SC-003**: A user can import the generated ICS file into at least two major calendar applications (e.g., Google Calendar, Apple Calendar) and all VTODOs appear with correct descriptions, due dates, and alarms.
- **SC-004**: The existing monthly TODOS markdown generation remains unchanged — the ICS is an additional output, not a replacement.

## Assumptions

- The CSV file `templates/todos_due_times.csv` exists in the project templates directory and follows the format: `weekday,priority,hour,minutes` with Spanish day names.
- Day names in the markdown file match the Spanish weekday names used in the CSV (Lunes, Martes, Miércoles, Jueves, Viernes, Sábado, Domingo).
- All times are in the system's local timezone. No timezone conversion or UTC normalization is required.
- The `[[ ]]` brackets in descriptions are always properly paired and can be stripped with a simple regex.
- The existing `chrono` crate handles date arithmetic and weekday name resolution. No additional date/time crate is needed.
- The existing output directory and file-naming conventions (TODOS_DEFAULT_PATH, `TODOS - YYYYMM.md`) are reused for the ICS file.
