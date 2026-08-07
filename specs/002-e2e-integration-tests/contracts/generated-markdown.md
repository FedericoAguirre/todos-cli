# Generated Markdown Format Contract

## Purpose

Defines the expected structure of `TODOS - YYYYMM.md` that the integration suite validates.

## File

Written to the `--path` (or `TODOS_DEFAULT_PATH`) directory as `TODOS - YYYYMM.md`.

## Structure

```
# TODOS YYYYMM

---

## YYYYMMDD - <SpanishWeekday>

- [ ] 1. [[Ejercicio]]
- [ ] 2. Trabajar en CBI (09:00-17:00)
- [ ] 3. Trabajar en [[Ematrix]], 2 horas
- [ ] 4. Convivir con Erin 30 minutos
- [ ] 5. Leer 30 minutos

## YYYYMMDD+1 - <SpanishWeekday>
...
```

## Validation Rules (asserted by tests)

- **Header line** (first non-empty line): exactly `# TODOS YYYYMM` where `YYYYMM` is zero-padded year+month (e.g., `# TODOS 202402`).
- **Day headings**: one `## YYYYMMDD - <Weekday>` line per day of the month, in ascending date order, with no gaps or duplicates.
- **YYYYMMDD**: zero-padded `%Y%m%d`, matching the actual calendar date.
- **Weekday names**: canonical Spanish set — `Lunes, Martes, Miércoles, Jueves, Viernes, Sábado, Domingo` — matching the actual weekday of that date and the template number used (1.md = Lunes … 7.md = Domingo).
- **Day count** by month length: 28 (non-leap Feb), 29 (leap Feb), 30, or 31.
- **Todo lines**: `- [ ] <priority>. <description>`; total count is deterministic from templates (5 per day) but asserted by counting from the file.

## Template Mapping

| Template | Weekday | weekday().number_from_monday() |
|----------|---------|-------------------------------|
| 1.md | Lunes | 1 |
| 2.md | Martes | 2 |
| 3.md | Miércoles | 3 |
| 4.md | Jueves | 4 |
| 5.md | Viernes | 5 |
| 6.md | Sábado | 6 |
| 7.md | Domingo | 7 |
