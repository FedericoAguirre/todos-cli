# Generated ICS Format Contract

## Purpose

Defines the expected structure of `TODOS - YYYYMM.ics` that the integration suite validates (RFC 5545 subset used by the CLI).

## File

Written alongside the markdown file as `TODOS - YYYYMM.ics`.

## Structure

```
BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//todos-cli//TODOS Calendar//EN
CALSCALE:GREGORIAN
X-WR-CALNAME:TODOS - YYYYMM
BEGIN:VEVENT
UID:<hash>@todos-cli
DTSTAMP:<YYYYMMDDTHHMMSSZ>
DTSTART:<YYYYMMDDTHHMMSSZ>
DTEND:<YYYYMMDDTHHMMSSZ>
SUMMARY:[P<n>] <description>
[ BEGIN:VALARM / TRIGGER:-PT<n>M / ACTION:DISPLAY / DESCRIPTION:Reminder / END:VALARM ]
END:VEVENT
...
END:VCALENDAR
```

## Validation Rules (asserted by tests)

- **Wrapper**: file begins with `BEGIN:VCALENDAR` and ends with `END:VCALENDAR`.
- **VEVENT count**: number of `BEGIN:VEVENT` occurrences equals the number of `- [ ] ` todo lines in the markdown file.
- **One VEVENT per todo**: each `- [ ]` item produces exactly one `VEVENT` (synchronization requirement).
- **Empty month**: a valid `VCALENDAR` with zero `VEVENT` blocks is still well-formed.
- **Encoding**: CRLF line endings; `VEVENT` (not `VTODO`) components; `DTEND` used instead of `DUE` (current implementation).

## Count Expectations (current templates, 5 todos/day)

| Month length | Example | VEVENT count |
|--------------|---------|--------------|
| 31 | 2026-07 | 155 |
| 30 | 2026-04 | 150 |
| 29 (leap Feb) | 2024-02 | 145 |
| 28 (non-leap Feb) | 2023-02 | 140 |
