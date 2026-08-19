# ICS File Format Contract (VTODO)

## File

Generated alongside `TODOS - YYYYMM.md` as `TODOS - YYYYMM.ics`.

## RFC 5545 Compliance

The file MUST conform to RFC 5545 (iCalendar). Key requirements:

- Line endings: CRLF (`\r\n`)
- Line length: Max 75 octets per line (folding with leading whitespace continuation)
- Content escaping: `\` → `\\`, `;` → `\;`, `,` → `\,`, `\n` → `\\n`
- Character set: UTF-8

## ICS Structure

```
BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//todos-cli//TODOS Calendar//EN
CALSCALE:GREGORIAN
X-WR-CALNAME:TODOS - YYYYMM
BEGIN:VTODO
UID:<unique-id>@todos-cli
DTSTAMP:<generation-timestamp-UTC>
DTSTART;VALUE=DATE:<YYYYMMDD>
SUMMARY:<[P<n>] escaped-description>
PRIORITY:<1-6>
DUE:<YYYYMMDDTHHMMSS>
STATUS:NEEDS-ACTION
[ BEGIN:VALARM
TRIGGER;RELATED=END:-PT<M>M
ACTION:DISPLAY
DESCRIPTION:Reminder
END:VALARM ]
END:VTODO
...
END:VCALENDAR
```

## Field Details

### UID
- MUST be globally unique per VTODO
- Format: `{hash(date + summary + priority + ordinal)}@todos-cli`
- Deterministic across regenerations; per-item ordinal appended so identical-looking todos stay distinct
- No dependency on `uuid` crate; hash-based avoids new deps

### DTSTAMP
- UTC timestamp of file generation
- Format: `YYYYMMDDTHHMMSSZ`
- Generated once per file, same for all VTODOs in the file

### DTSTART
- Value type: DATE (no time component, no timezone)
- Format: `DTSTART;VALUE=DATE:YYYYMMDD`
- Pins the task to its todo's day (all-day task placement)

### SUMMARY
- `[P<n>]` prefix + todo description with `[[ ]]` brackets stripped
- RFC 5545 content-line escaping applied

### PRIORITY
- Integer 1–6 (1 = highest), mapped directly from the CSV/markdown priority
- RFC 5545 defines 1 as highest, 9 as lowest

### DUE
- Format: `YYYYMMDDTHHMMSS` (floating local time — no timezone suffix, no TZID)
- Computed as: `todo.date + rule.hour` on matching weekday + priority
- Default (no CSV match): `YYYYMMDDT235959`
- MUST NOT precede the task's DTSTART date

### STATUS
- Always: `NEEDS-ACTION`
- RFC 5545 defines: NEEDS-ACTION, COMPLETED, IN-PROCESS, CANCELLED

### VALARM
- Trigger: `TRIGGER;RELATED=END:-PT{M}M` (M minutes before DUE)
- `RELATED=END` anchors the alarm to the DUE moment (correct for date-only DTSTART)
- Action: `DISPLAY` (shows popup reminder)
- Omitted entirely when no CSV match for the todo

## Compatibility Notes

- macOS/Apple: `VTODO` is the task format consumed by task-capable surfaces/processors; Calendar.app itself only renders `VEVENT` events and is not the target surface for tasks.
- Android: task-capable calendar/task apps and CalDAV processors honor `VTODO` with `DTSTART;VALUE=DATE` + `DUE`.
- Generic ICS processors: RFC 5545-compliant readers parse all fields; event-only importers ignore `VTODO` (expected behavior, not a defect).