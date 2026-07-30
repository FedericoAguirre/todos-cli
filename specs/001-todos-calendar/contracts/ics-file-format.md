# ICS File Format Contract

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
X-WR-CALNAME:TODOS - YYYYMM
BEGIN:VTODO
UID:<unique-id>
DTSTAMP:<generation-timestamp>
DTSTART;VALUE=DATE:<YYYYMMDD>
SUMMARY:<escaped-description>
PRIORITY:<1-9>
DUE:<YYYYMMDDTHHMMSS>
STATUS:NEEDS-ACTION
BEGIN:VALARM
TRIGGER:-PT<M>M
ACTION:DISPLAY
DESCRIPTION:Reminder
END:VALARM
END:VTODO
...
END:VCALENDAR
```

## Field Details

### UID
- MUST be globally unique per VTODO
- Generated as: `{sha256(date + summary + priority)}@todos-cli`
- No dependency on `uuid` crate; hash-based avoids new deps

### DTSTAMP
- UTC timestamp of file generation
- Format: `YYYYMMDDTHHMMSSZ`
- Generated once per file, same for all VTODOs in the file

### DTSTART
- Value type: DATE (no time component)
- Format: `DTSTART;VALUE=DATE:YYYYMMDD`
- Uses the todo's date from the monthly file

### SUMMARY
- Todo description with `[[ ]]` brackets stripped
- RFC 5545 content-line escaping applied

### PRIORITY
- Integer 1–9 (1 = highest)
- Maps from CSV priority range 1–6 directly
- RFC 5545 defines 1 as highest, 9 as lowest

### DUE
- Format: `YYYYMMDDTHHMMSS` (local time, no timezone suffix)
- Computed as: `md.date + csv.hour` on matching weekday + priority
- Default (no CSV match): `YYYYMMDDT235959`

### STATUS
- Always: `NEEDS-ACTION`
- RFC 5545 defines: NEEDS-ACTION, COMPLETED, IN-PROCESS, CANCELLED

### VALARM
- Trigger: `-PT{M}M` (M minutes before DUE)
- Action: `DISPLAY` (shows popup reminder)
- Omitted entirely when no CSV match for the todo
