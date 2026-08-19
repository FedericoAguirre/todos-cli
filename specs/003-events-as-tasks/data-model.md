# Data Model: Events as Tasks in Calendar

## Entities

### TodoItem

A single pending task parsed from the monthly TODOS markdown file. **Unchanged** from the existing parser.

| Field | Type | Description | Rules |
|-------|------|-------------|-------|
| `description` | `String` | Task description, stripped of `[[ ]]` brackets | Must not be empty |
| `priority` | `u8` | Numeric priority (1-indexed) | 1–6 (matching CSV range) |
| `date` | `NaiveDate` | Date this todo belongs to (YYYY-MM-DD) | Valid date within target month |
| `weekday_name` | `String` | Spanish weekday name (e.g., "Miércoles") | Must match CSV weekday column |

### DueTimeRule

A row from `templates/todos_due_times.csv` mapping weekday + priority to scheduling. **Unchanged**.

| Field | Type | Description | Rules |
|-------|------|-------------|-------|
| `weekday` | `String` | Spanish weekday name | Lunes–Domingo |
| `priority` | `u8` | Priority level | 1–6 |
| `hour` | `NaiveTime` | Due hour:minute | 00:00–23:59 |
| `alarm_minutes` | `u16` | Minutes before DUE to trigger alarm | 0–1440 |

### IcsCalendar

In-memory representation of the ICS file being built. Renamed collections: `events` → `todos`.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Calendar name (e.g., "TODOS - 202607") |
| `todos` | `Vec<IcsTodo>` | All VTODO components |

### IcsTodo *(replaces `IcsEvent`)*

A single VTODO task component ready for serialization.

| Field | Type | Description | Rules |
|-------|------|-------------|-------|
| `uid` | `String` | Globally unique identifier | Deterministic; unique per VTODO |
| `dtstamp` | `DateTime<Utc>` | Timestamp of ICS generation | Set once per file (UTC) |
| `summary` | `String` | `[P<n>] ` + description | From TodoItem.description |
| `date` | `NaiveDate` | Task day, serialized as `DTSTART;VALUE=DATE` | From TodoItem.date (Q2) |
| `priority` | `u8` | Task priority, serialized as `PRIORITY` | From TodoItem.priority |
| `due` | `NaiveDateTime` | Due date+time, serialized as `DUE` (floating local) | Computed via DueTimeRule lookup |
| `status` | `String` | Always `"NEEDS-ACTION"` | Constant |
| `alarm_minutes` | `Option<u16>` | Minutes before DUE for VALARM | `None` if no CSV match |

## State Transitions

```
TODOS YYYYMM.md  ──parse (skip - [x])──▶  Vec<TodoItem>
                                                 │
                                                 ▼
todos_due_times.csv ──parse──▶  Vec<DueTimeRule>
                                                 │
                                                 ▼ (join on weekday + priority)
                                          Vec<IcsTodo>
                                                 │
                                                 ▼ (serialize VTODO ICS)
                                          TODOS YYYYMM.ics
```

## Validation Rules

- **TodoItem.description**: Must not be empty after stripping `[[ ]]`. If empty, skip with warning.
- **TodoItem.priority**: If out of range 1–6, log warning and clamp to 6.
- **Checked-off todos (`- [x]`)**: Never parsed into `TodoItem`; produce no task (FR-014).
- **DueTimeRule.hour**: Parse as `HH:MM`. Invalid format → skip rule with warning.
- **DueTimeRule.alarm_minutes**: Must be u16. Invalid → default to 0.
- **DUE vs DTSTART ordering**: `DUE` is always on the todo's own date at the rule hour (or 23:59 default), so it is never before the task's day (RFC §3.8.2.3).
- **Duplicates**: Identical todos (same date + description + priority) still get distinct `uid` values because `uid` is derived from `(date, summary, priority)` with identical inputs colliding → append per-item ordinal to guarantee uniqueness (FR-010).
- **ICS output**: Must produce valid RFC 5545 output (line folding at 75 octets, proper escaping, CRLF line endings; `DTSTAMP` UTC, `DTSTART` date-only, `DUE` floating local).