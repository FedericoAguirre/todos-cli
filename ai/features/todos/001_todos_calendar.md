# Todos calendar

## User story

As a **todos-cli user**, I want **to generate an ICS calendar from my monthly TODOS file**, so that **I can export and view my todos as VTODO components in any calendar app**.

## Acceptance criteria

**Scenario 1: Default calendar generation**
Given I run the CLI with `--year` and `--month` options,
When the monthly TODOS file is generated,
Then an ICS calendar file is also generated alongside it.

**Scenario 2: Calendar file in TODOS_DEFAULT_PATH**
Given `TODOS_DEFAULT_PATH` is set and I run the CLI with `--year` and `--month`,
When no explicit `--path` argument is provided,
Then the ICS calendar file is generated in the `TODOS_DEFAULT_PATH` directory.

**Scenario 3: Calendar file naming**
Given the monthly TODOS file is named `TODOS - YYYYMM.md`,
When the ICS calendar is generated,
Then it is named `TODOS - YYYYMM.ics` in the same directory.

**Scenario 4: VTODO per todo item**
Given a month with N todo items across all days,
When the ICS calendar is generated,
Then it contains exactly N VTODO components, one per todo item.

**Scenario 5: VTODO attributes — description, date, priority, due, status**
Given a todo item in the monthly TODOS file,
When the corresponding VTODO is generated,
Then it includes:
- Description: the todo text (with `[[ ]]` brackets removed)
- Date: the todo's date from the monthly file
- Priority: the same priority number from the monthly file
- Due: a due timestamp computed from the due rules
- Status: `NEEDS-ACTION`

**Scenario 6: Due timestamp from CSV lookup**
Given a todo with weekday W, priority P, and date D,
When the due timestamp is computed,
Then it equals `D + csv.hour` where `csv.weekday = W` and `csv.priority = P` in `templates/todos_due_times.csv`.

**Scenario 7: Alarm before due**
Given a todo with weekday W and priority P,
When the VTODO is generated,
Then it emits an alarm `csv.minutes` minutes before the due timestamp, where `csv.weekday = W` and `csv.priority = P`.

## Definition of Done

- The CLI generates a valid `.ics` file (parseable by an iCalendar parser) whenever it generates a monthly TODOS file.
- Each VTODO in the ICS file has correct Description, Date, Priority, Due, and Status fields.
- The Due timestamp and alarm offset are correctly resolved via the CSV lookup table.
- Existing monthly TODOS file generation is unaffected.
- Unit tests cover:
  - MD parsing (extracting date, weekday, priority, description)
  - CSV parsing
  - Due timestamp computation
  - Alarm offset computation
  - ICS file generation and VTODO structure

## Technical notes

### Due rules

The calendar must have as many VTODOs per day as the monthly TODOS file has.

The **due timestamp** for a VTODO is computed from the join between `csv.weekday` and `csv.priority` columns (from `templates/todos_due_times.csv`) and `md.weekday` and `md.priority` (from `TODOS - YYYYMM.md`), using `md.date + csv.hour`.

Formula:
```
due_timestamp = md.date + csv.hour
  where md.weekday = csv.weekday AND md.priority = csv.priority
```

Alarm formula:
```
alarm_timestamp = due_timestamp - csv.minutes
  where md.weekday = csv.weekday AND md.priority = csv.priority
```

### CSV format (`templates/todos_due_times.csv`)

```csv
weekday,priority,hour,minutes
Lunes,1,9:00,30
Lunes,2,16:00,30
...
```

Day names are in Spanish (Lunes, Martes, Miércoles, Jueves, Viernes, Sábado, Domingo).

### MD format expected

Each day entry in the monthly TODOS file follows this pattern:

```markdown
## YYYYMMDD - WeekdayName

- [ ] P. [[Description]]
```

Where:
- `YYYYMMDD` is the date
- `WeekdayName` is the Spanish day name (matches `csv.weekday`)
- `P` is a numeric priority (1-indexed)
- `Description` is the todo text

### Example

Given `TODOS - 202607.md` with day `20260701 - Miércoles` containing 4 todos:

| # | Description | Priority | Due (local) | Alarm (local) |
|---|-------------|----------|-------------|---------------|
| 1 | Ejercicio | 1 | 2026-07-01 09:00 | 2026-07-01 08:30 |
| 2 | Entrevistas | 2 | 2026-07-01 16:00 | 2026-07-01 15:30 |
| 3 | Leer 30 minutos | 3 | 2026-07-01 18:00 | 2026-07-01 17:50 |
| 4 | Ir por Erin | 4 | 2026-07-01 19:00 | 2026-07-01 18:50 |
