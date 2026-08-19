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
BEGIN:VTODO
UID:<hash>@todos-cli
DTSTAMP:<YYYYMMDDTHHMMSSZ>
DTSTART;VALUE=DATE:<YYYYMMDD>
SUMMARY:[P<n>] <description>
PRIORITY:<1-6>
DUE:<YYYYMMDDTHHMMSS>
STATUS:NEEDS-ACTION
[ BEGIN:VALARM / TRIGGER;RELATED=END:-PT<n>M / ACTION:DISPLAY / DESCRIPTION:Reminder / END:VALARM ]
END:VTODO
...
END:VCALENDAR
```

## Validation Rules (asserted by tests)

- **Wrapper**: file begins with `BEGIN:VCALENDAR` and ends with `END:VCALENDAR`.
- **VTODO count**: number of `BEGIN:VTODO` occurrences equals the number of `- [ ] ` todo lines in the markdown file.
- **One VTODO per todo**: each `- [ ]` item produces exactly one `VTODO` (synchronization requirement).
- **Empty month**: a valid `VCALENDAR` with zero `VTODO` blocks is still well-formed.
- **Encoding**: CRLF line endings; `VTODO` (not `VEVENT`) components; `DUE` used instead of `DTEND`; `DTSTART` is date-only (`VALUE=DATE`); `DUE` is floating local (no timezone suffix); `DTSTAMP` is UTC (`Z`).

## Count Expectations (current templates, 5 todos/day)

| Month length | Example | VTODO count |
|--------------|---------|-------------|
| 31 | 2026-07 | 155 |
| 30 | 2026-04 | 150 |
| 29 (leap Feb) | 2024-02 | 145 |
| 28 (non-leap Feb) | 2023-02 | 140 |
