# Feature Specification: Events as Tasks in Calendar

**Feature Branch**: `003-events-as-tasks`

**Created**: 2026-08-18

**Status**: Draft

**Input**: User description: "I want to change the events in the events file to be tasks, they must be represented as tasks in the android and mac calendars or other ics processors"

## Clarifications

### Session 2026-08-18

- Q: Should todos that are already checked off (`- [x]`) in the monthly file appear as completed tasks in the calendar, or should only still-pending todos (`- [ ]`) become tasks? → A: Only pending todos become tasks; checked-off items are excluded and emit no task (matches current behavior).
- Q: Should each task carry a scheduled date as well as its due moment (i.e., appear pinned to its todo-day in the calendar), or only a due moment? → A: Each task carries both a date-only date field pinned to the todo's day and a time-based due moment (rule-based due time preserved for reminders).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Todos Appear as Tasks in Calendar Apps (Priority: P1)

As a todos-cli user, I import the generated calendar file into my calendar app and each todo from my monthly file appears as a **task** that I can check off — not as a time-blocked event — in Android, Mac, and other applications that read ICS files.

**Why this priority**: This is the core value of the change. The current calendar file shows todos as events that occupy time slots; the user wants them to behave as actual to-do items (tasks) in every calendar processor.

**Independent Test**: Import the generated calendar file into a calendar application that distinguishes tasks from events (e.g., Apple Calendar's Reminders area, or a TODO-compliant reader) and confirm every todo is listed as a task rather than an event on the agenda.

**Acceptance Scenarios**:

1. **Given** a calendar file with N todos, **When** it is inspected or imported into an application that distinguishes task vs. event entries, **Then** all N todos are represented as tasks and none as events.
2. **Given** a calendar file generated for a valid month, **When** a task from the file is opened, **Then** it displays the scheduled due date and time, allowing it to be marked complete.
3. **Given** a calendar file, **When** scanned for event-type time-block entries, **Then** no todo occupies a fixed start-to-end time block (todos carry a due moment instead).
4. **Given** a task in a calendar app, **When** the user checks it off, **Then** that task's completion state is preserved independently of the other tasks.

---

### User Story 2 - Due Date and Time from Existing Rules (Priority: P1)

As a todos-cli user, I want each task to carry the correct due date and time computed from my existing weekday/priority due-time rules, so that my calendar reflects my planned schedule.

**Why this priority**: A task without a due date is not actionable in a calendar. This preserves the scheduling value introduced by the due-time rules.

**Independent Test**: Generate the calendar file, then check that each task shows a due date matching the todo's date from the markdown, and a due time that either matches the configured rule for that weekday+priority or defaults to the end of that day.

**Acceptance Scenarios**:

1. **Given** a todo scheduled on weekday W with priority P, **When** the task is generated, **Then** its due date equals the todo's date from the monthly file.
2. **Given** a todo whose weekday+priority matches a rule, **When** the task is generated, **Then** its due time equals the configured hour for that rule.
3. **Given** a todo with no matching rule, **When** the task is generated, **Then** its due time defaults to the end of that same day (23:59).
4. **Given** a task in a calendar app, **When** an alarm was configured, **Then** the alarm fires the configured number of minutes before the due time.

---

### User Story 3 - Compatibility with Android, Mac, and Other ICS Processors (Priority: P2)

As a todos-cli user, I want the calendar file to import cleanly into Android, Mac (Apple), and other ICS-consuming applications, so my todos are usable across devices.

**Why this priority**: The change is only valuable if calendar processors honor the task representation during import without errors or dropped items.

**Independent Test**: Import the same generated file into at least two major calendar applications (one Android-based and one Apple-based) and confirm all tasks appear with correct due dates, priorities, and alarms.

**Acceptance Scenarios**:

1. **Given** a generated calendar file, **When** imported into an Android calendar app, **Then** all todos appear as tasks with correct due dates.
2. **Given** a generated calendar file, **When** imported into the Mac (Apple) calendar, **Then** all todos appear as tasks in the Reminders section with correct due dates and priorities.
3. **Given** a generated calendar file, **When** opened by an RFC 5545-compliant processor, **Then** it parses without structural errors.

---

### User Story 4 - Regeneration and Re-import (Priority: P3)

As a todos-cli user, I want tasks to keep a stable identity across file regeneration and re-import, so that completion state and metadata I set in my calendar app do not get confused between versions of the file.

**Why this priority**: Tasks are stateful (completion), so stable identity matters once users start tracking progress in their calendar app. Less critical than the first three stories but important for long-term use.

**Independent Test**: Generate the file for the same month twice and verify each task maps to the same stable identity both times, while the priority label and due date remain consistent.

**Acceptance Scenarios**:

1. **Given** the same monthly input, **When** the calendar file is generated twice, **Then** each todo maps to the same stable identity in both versions.
2. **Given** two todos with identical descriptions on the same date, **When** their tasks are inspected, **Then** they have distinct identities.
3. **Given** a todo with brackets in its description, **When** the task is generated, **Then** the brackets are removed and only the inner text is shown.

---

### Edge Cases

- What happens when the due-time CSV file is missing or unparseable? — The tool SHOULD emit a warning on stderr and still generate the calendar file with each task due at end-of-day (23:59) and no alarms.
- What happens when a month has no todos at all? — The tool SHOULD generate a valid calendar file containing zero tasks.
- What happens when a todo's weekday+priority has no matching rule? — That task defaults to end-of-day due time with no alarm; other tasks are unaffected.
- What happens when two todos look identical (same description, date, priority)? — They MUST remain distinct tasks so the calendar app can track them separately.
- What happens to checked-off todos (`- [x]`) in the monthly file? — They are excluded from the calendar file entirely; only pending todos (`- [ ]`) are represented as tasks.
- What happens when the calendar file is edited or the app marks a task complete and the file is regenerated? — Regeneration produces a new snapshot; existing app-side completion state is not modified, but re-importing the new file may create new tasks. This is standard ICS behavior.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The calendar file MUST represent each todo item as a task rather than as a time-blocked event.
- **FR-002**: Each task MUST include a summary derived from the todo description with `[[ ]]` brackets removed.
- **FR-003**: Each task MUST include a date-only date field equal to the todo's date from the monthly markdown file, pinning the task to its day.
- **FR-004**: Each task MUST include a due time computed from the weekday/priority due-time rules when a match exists.
- **FR-005**: Each task MUST default its due time to the end of that day (23:59) when no rule matches its weekday and priority.
- **FR-006**: Each task MUST retain the todo's numeric priority (1 highest through 6).
- **FR-007**: Each task with a matching due-time rule MUST include a reminder that fires the configured number of minutes before the due time.
- **FR-008**: Tasks with no matching rule MUST NOT include a reminder.
- **FR-009**: The calendar file MUST contain exactly one task component per pending todo item (`- [ ]`) from the monthly file.
- **FR-010**: Each task MUST have a stable, unique identity derived deterministically from its data, so regeneration is consistent and identical-looking todos stay distinct.
- **FR-011**: The calendar file MUST remain RFC 5545-compliant and import into Android, Apple, and other ICS processors.
- **FR-012**: When the due-time CSV is missing or unparseable, the tool MUST warn on stderr and generate the file with end-of-day due times and no reminders.
- **FR-013**: The calendar file MUST continue to be generated alongside the monthly markdown file under the same `TODOS - YYYYMM` naming convention.
- **FR-014**: The calendar file MUST NOT represent checked-off todos (`- [x]`); only pending todos (`- [ ]`) produce a task component.

### Key Entities *(include if feature involves data)*

- **Todo/Task**: A single item from the monthly file (description, priority, date, weekday) that the calendar file exposes as a checkable task.
- **Task Component**: The calendar representation of a Todo/Task carrying its summary, date-only day field, due moment, priority, unique identity, and optional reminder.
- **Due-Time Rule**: A weekday+priority → (hour, reminder minutes) mapping used to compute a task's due time and reminder.
- **Calendar File**: The ICS output (`TODOS - YYYYMM.ics`) imported by calendar applications; a container of Task components plus calendar metadata.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of todos in a generated calendar file are represented as tasks (zero represented as time-blocked events), verified by inspection of the file.
- **SC-002**: The generated calendar file imports without errors into at least one Android-based and one Apple-based calendar application, with all todos visible as tasks.
- **SC-003**: Every task's due date matches its todo's date for 100% of todos; every rule-matching task shows the configured due time and reminder.
- **SC-004**: The file passes RFC 5545 structural validation with zero errors.
- **SC-005**: Regenerating the file for the same month yields identical task identities (no spurious duplicate or drifting tasks across identical inputs).

## Assumptions

- The due-time rules (`todos_due_times.csv`) and the markdown parsing behavior stay unchanged; only the calendar representation of the output changes.
- Tasks are sent with both a date-only day field (the todo's date) and an absolute due moment (date + local time), matching how the existing due-time rules are interpreted.
- Calendar applications are expected to render a task as their native to-do/reminder object; exact rendering (e.g., where Reminders appears) is app-specific and outside the tool's control.
- The tool always overwrites the calendar file on regeneration (single snapshot per month); app-side completion state is managed by the calendar app, not by the CLI.
- No new external services or accounts are required for import; compatibility is delivered entirely through the file format.
- The prior behavior delivered events in the calendar file; this feature converts that output to tasks, so consumers (docs, tests, contracts) currently describing event-style entries will be updated accordingly.