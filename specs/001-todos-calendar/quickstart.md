# Quickstart: Todos Calendar

## Prerequisites

- Rust toolchain (rustc 1.75+, cargo)
- `templates/todos_due_times.csv` exists with valid weekday/priority mapping
- `templates/` directory with day templates (1.md–7.md) and header.md

## Setup

```bash
# Build the CLI
cargo build

# (Optional) Set default output path
export TODOS_DEFAULT_PATH=/tmp/todos-test
```

## Validation Scenarios

### Scenario 1: Basic ICS generation

```bash
cargo run -- --year 2026 --month 7 --path /tmp/todos-test
```

**Expected outcomes**:
- `/tmp/todos-test/TODOS - 202607.md` exists (unchanged format)
- `/tmp/todos-test/TODOS - 202607.ics` exists
- ICS file contains one VTODO per todo item in the markdown file
- ICS file passes RFC 5545 validation (see verification below)

### Scenario 2: Default path

```bash
export TODOS_DEFAULT_PATH=/tmp/todos-default
cargo run -- --year 2026 --month 8
```

**Expected outcomes**:
- `/tmp/todos-default/TODOS - 202608.ics` is generated
- Same location as the .md file

### Scenario 3: CSV edge cases

1. **Missing CSV**: Rename/move `templates/todos_due_times.csv`, run the CLI — ICS file still generates with default due times (23:59) and no alarms; warning on stderr.
2. **No todos**: Use a month with zero todo entries — ICS file has zero VTODOs, valid RFC 5545.

### Scenario 4: ICS validation

```bash
# Check basic ICS structure — grep for key components
grep "BEGIN:VCALENDAR" /tmp/todos-test/TODOS\ -\ 202607.ics
grep "BEGIN:VTODO" /tmp/todos-test/TODOS\ -\ 202607.ics
grep "STATUS:NEEDS-ACTION" /tmp/todos-test/TODOS\ -\ 202607.ics
grep "BEGIN:VALARM" /tmp/todos-test/TODOS\ -\ 202607.ics

# Count VTODOs (should match todo count in markdown)
grep -c "BEGIN:VTODO" /tmp/todos-test/TODOS\ -\ 202607.ics
```

### Scenario 5: Due timestamp verification

For a known weekday+priority combination, verify the DUE field:

```bash
# Extract DUE lines for a specific date
grep "DUE:20260701" /tmp/todos-test/TODOS\ -\ 202607.ics
```

Given the CSV mapping `Miércoles,1,9:00,30`, the first VTODO for date `20260701` (Miércoles) with priority 1 should have:
- `DUE:20260701T090000`
- `TRIGGER:-PT30M` in its VALARM

## Testing

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test calendar
cargo test parser

# Lint and format check
cargo clippy
cargo fmt --check
```

## Verification artifacts

- [Data model](./data-model.md) — entity definitions and validation rules
- [ICS file format contract](./contracts/ics-file-format.md) — RFC 5545 structure details
- [CLI interface contract](./contracts/cli-interface.md) — argument and behavior spec
