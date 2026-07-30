# Data Model: Todos Calendar

## Entities

### TodoItem

A single task parsed from the monthly TODOS markdown file.

| Field | Type | Description | Rules |
|-------|------|-------------|-------|
| `description` | `String` | Task description, stripped of `[[ ]]` brackets | Must not be empty |
| `priority` | `u8` | Numeric priority (1-indexed) | 1–6 (matching CSV range) |
| `date` | `NaiveDate` | The date this todo belongs to (YYYY-MM-DD) | Valid date within target month |
| `weekday_name` | `String` | Spanish weekday name (e.g., "Miércoles") | Must match CSV weekday column |

### DueTimeRule

A row from `templates/todos_due_times.csv` mapping weekday + priority to scheduling.

| Field | Type | Description | Rules |
|-------|------|-------------|-------|
| `weekday` | `String` | Spanish weekday name | Lunes–Domingo |
| `priority` | `u8` | Priority level | 1–6 |
| `hour` | `NaiveTime` | Due hour:minute | 00:00–23:59 |
| `alarm_minutes` | `u16` | Minutes before due to trigger alarm | 0–1440 |

### IcsCalendar

In-memory representation of the ICS file being built.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Calendar name (e.g., "TODOS - 202607") |
| `todos` | `Vec<IcsTodo>` | All VTODO components |

### IcsTodo

A single VTODO component ready for serialization.

| Field | Type | Description | Rules |
|-------|------|-------------|-------|
| `uid` | `String` | Globally unique identifier | Must be unique per VTODO |
| `dtstamp` | `DateTime<Utc>` | Timestamp of ICS generation | Set once per file |
| `summary` | `String` | Task description | From TodoItem.description |
| `dtstart` | `NaiveDate` | Task date | From TodoItem.date |
| `priority` | `u8` | Task priority | From TodoItem.priority |
| `due` | `NaiveDateTime` | Due date+time | Computed via DueTimeRule lookup |
| `status` | `String` | Always `"NEEDS-ACTION"` | Constant |
| `alarm_minutes` | `Option<u16>` | Minutes before due for VALARM | `None` if no CSV match |

## State Transitions

```
TODOS YYYYMM.md  ──parse──▶  Vec<TodoItem>
                                     │
                                     ▼
todos_due_times.csv ──parse──▶  Vec<DueTimeRule>
                                     │
                                     ▼ (join on weekday + priority)
                              Vec<IcsTodo>
                                     │
                                     ▼ (serialize ICS text)
                              TODOS YYYYMM.ics
```

## Validation Rules

- **TodoItem.description**: Must not be empty after stripping `[[ ]]`. If empty, skip with warning.
- **TodoItem.priority**: If out of range 1–6, log warning and clamp to 6.
- **DueTimeRule.hour**: Parse as `HH:MM`. Invalid format → skip rule with warning.
- **DueTimeRule.alarm_minutes**: Must be u16. Invalid → default to 0.
- **ICS output**: Must produce valid RFC 5545 output (line folding at 75 octets, proper escaping, CRLF line endings).
