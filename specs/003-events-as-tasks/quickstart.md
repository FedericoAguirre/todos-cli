# Quickstart: Events as Tasks in Calendar

## Prerequisites

- Rust toolchain (rustc 1.85+, cargo)
- `templates/todos_due_times.csv` exists with valid weekday/priority mapping
- `templates/` directory with day templates (1.md–7.md) and header.md

## Setup

```bash
# Build the CLI
cargo build
```

## Validation Scenarios

### Scenario 1: Tasks instead of events

```bash
cargo run -- --year 2026 --month 7 --path /tmp/todos-test
```

**Expected outcomes**:
- `/tmp/todos-test/TODOS - 202607.md` exists (unchanged format)
- `/tmp/todos-test/TODOS - 202607.ics` exists
- ICS file contains one `VTODO` per pending todo item (no `VEVENT`)

### Scenario 2: Task structure verification

```bash
grep "BEGIN:VTODO" "/tmp/todos-test/TODOS - 202607.ics"
grep "DTSTART;VALUE=DATE" "/tmp/todos-test/TODOS - 202607.ics"
grep "DUE:" "/tmp/todos-test/TODOS - 202607.ics"
grep "STATUS:NEEDS-ACTION" "/tmp/todos-test/TODOS - 202607.ics"
grep "PRIORITY:" "/tmp/todos-test/TODOS - 202607.ics"
grep "TRIGGER;RELATED=END" "/tmp/todos-test/TODOS - 202607.ics"
grep -c "BEGIN:VTODO" "/tmp/todos-test/TODOS - 202607.ics"  # == pending todo count
```

### Scenario 3: Due timestamp from rules

Given CSV mapping `Miércoles,1,9:00,30`, a task for date `20260701` (Miércoles, priority 1) must show:
- `DUE:20260701T090000`
- `TRIGGER;RELATED=END:-PT30M` in its VALARM

### Scenario 4: Checked-off todos excluded

Add a `- [x] 1. Done task` line to the markdown, regenerate, and confirm no `VTODO` is produced for it (the parser only reads `- [ ]`).

### Scenario 5: Edge cases

1. **Missing CSV**: Rename/move `templates/todos_due_times.csv`, run the CLI — ICS still generates with `DUE` at 23:59 and no VALARM; warning on stderr.
2. **No todos**: A month with zero pending todos yields a valid VCALENDAR with zero VTODOs.
3. **No CSV match**: A weekday+priority with no rule → `DUE:<date>T235959`, no VALARM.

### Scenario 6: Calendar app import

- **macOS / Android task processors**: Import the `.ics` into a task-capable client and confirm each todo appears as a task with its due date and priority, and is checkable.
- **RFC 5545 validation**: Run the generated file through an iCalendar validator — zero structural errors.

## Testing

```bash
# Run all tests
cargo test

# Specific modules
cargo test calendar
cargo test parser

# Lint and format check
cargo clippy
cargo fmt --check
```

## Verification artifacts

- [Data model](./data-model.md) — entity definitions and validation rules
- [ICS file format contract](./contracts/ics-file-format.md) — VTODO field details
- [Research](./research.md) — format decisions and rationale