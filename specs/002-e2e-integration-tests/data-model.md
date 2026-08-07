# Data Model: E2E Integration Test Suite

This feature produces no new production data. The "data model" describes the runtime artifacts the tests create, validate, and clean up, plus the expected-content model used for assertions.

## Entities

### TestOutputDirectory

A unique temporary directory created per test run to isolate CLI output.

| Field | Type | Description | Rules |
|-------|------|-------------|-------|
| `path` | `PathBuf` | Directory under the OS temp dir | `std::env::temp_dir()/todos-e2e-<pid>-<nanos>` |
| `state` | Created → Populated → Cleaned | Lifecycle of the directory | Created before CLI run; removed on success; preserved on failure |

### GeneratedMarkdown

The `TODOS - YYYYMM.md` file written by the CLI.

| Field | Type | Description | Rules |
|-------|------|-------------|-------|
| `filename` | `String` | `TODOS - YYYYMM.md` | `YYYYMM` = zero-padded year+month |
| `header_line` | `String` | First line | Exactly `# TODOS YYYYMM` |
| `day_headings` | `Vec<String>` | All `## YYYYMMDD - <Weekday>` lines | One per day of month, contiguous, no duplicates |
| `todo_count` | `usize` | Number of `- [ ] ` lines | Equals ICS `BEGIN:VEVENT` count |

### GeneratedICS

The `TODOS - YYYYMM.ics` file written by the CLI.

| Field | Type | Description | Rules |
|-------|------|-------------|-------|
| `filename` | `String` | `TODOS - YYYYMM.ics` | Same `YYYYMM` as markdown |
| `starts_with_vcalendar` | `bool` | Begins with `BEGIN:VCALENDAR` | Must be true |
| `ends_with_vcalendar` | `bool` | Ends with `END:VCALENDAR` | Must be true |
| `vevent_count` | `usize` | Number of `BEGIN:VEVENT` occurrences | Must equal markdown `todo_count` |

### ExpectedDay

A single expected day derived in-test from chrono calendar arithmetic.

| Field | Type | Description | Rules |
|-------|------|-------------|-------|
| `date` | `NaiveDate` | A day within the target month | 1..=days_in_month |
| `yyyymmdd` | `String` | Zero-padded `%Y%m%d` | Matches heading suffix |
| `weekday_name` | `String` | Spanish name from the weekday array | `Lunes`…`Domingo` |
| `template_num` | `u8` | `weekday().number_from_monday()` | 1–7 |

## Relationships

```
TestOutputDirectory (1) ──contains──▶ GeneratedMarkdown (1)
        │                                 │ todo_count
        └────contains────▶ GeneratedICS (1) │ vevent_count
                                              ▼
                               Assert: todo_count == vevent_count
```

The markdown's `day_headings` is compared 1:1 against the sequence of `ExpectedDay` values (length, order, `YYYYMMDD`, and weekday name).

## Validation Rules

- `day_headings.len()` == expected days in month (28/29/30/31).
- All 7 distinct weekday names appear across the month (any month of ≥28 days covers all weekdays).
- Every `## YYYYMMDD - <Weekday>` line uses the correct date and Spanish name for that actual calendar day.
- `todo_count` (markdown) == `vevent_count` (ICS).
- Header line is exactly `# TODOS YYYYMM`.
- February: 2024 → 29 headings, 2023 → 28 headings (Gregorian leap rule via chrono).
- Temp dir exists after CLI run; removed after successful validation; preserved on failure.
