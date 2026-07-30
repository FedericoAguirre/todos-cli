# Research: ICS Crate Decision for Todos Calendar

## Research Tasks

### 1. Evaluate Rust ICS crate options

**Finding**: Three primary candidates for ICS generation in Rust:

| Crate | VTODO | VALARM | chrono integration | Maintenance | Downloads |
|-------|-------|--------|-------------------|-------------|-----------|
| `icalendar` | ✅ Yes | ✅ Yes | ✅ Native | Active (48 releases, Jun 2025) | ~3.7K/mo |
| `ics` | ⚠️ Event-focused | ❌ No | ❌ No | Archived (last update Sep 2022) | Low |
| `ezcal` | ✅ Yes | ✅ Yes | ✅ Optional | New (2026), less proven | Low |
| Manual string | N/A | N/A | N/A | N/A | N/A |

### 2. Evaluate manual ICS generation viability

**Finding**: The ICS subset needed (VCALENDAR + VTODO + VALARM) is small but has several RFC 5545 requirements:

- **Line folding**: Lines must not exceed 75 octets; continuation with leading whitespace
- **Content escaping**: `\`, `;`, `,`, `\n` must be escaped in property values
- **Date-time format**: Must be `YYYYMMDDTHHMMSS` with optional timezone
- **Property parameters**: e.g., `VALUE=DATE`, `TZID=...`
- **VTODO required properties**: `DTSTAMP`, `UID`, `SUMMARY` — must be unique per VTODO

### 3. Evaluate constitutional compliance

**Finding**: The constitution (Principle I, Toolchain) requires justification for any new production dependency. The `icalendar` crate would add ~170KB and ~3.5K SLoC to the build. However, it handles all RFC 5545 edge cases correctly.

## Decision

**Chosen approach**: Manual ICS string generation (no new crate).

**Rationale**:
1. The ICS subset needed is narrow: VTODO + VALARM inside a VCALENDAR, no recurrence, no timezone math, no parsing
2. The constitution prioritizes minimum dependencies — adding a crate for ~200 lines of ICS output is disproportionate
3. Manual generation gives full control over line folding, escaping, and formatting without pulling in an external dependency
4. The `icalendar` crate pulls in ~1.8–5MB of transitive dependencies (~72K SLoC), which violates the spirit of Principle I

**Alternatives considered**:
- **`icalendar` crate**: Rejected as heavyweight for the narrow use case (VTODO + VALARM only). Worth reconsidering if future features need recurrence rules, timezone handling, or ICS parsing.
- **`ics` crate**: Rejected due to no VALARM support and being unmaintained.
- **`ezcal` crate**: Rejected as too new and unproven; minimal community adoption.

## Implementation Guidance

- Implement a `calendar` module with a `generate_ics()` function that takes parsed todo data and returns a String
- Use a builder-like internal API:
  - `Calendar::new() → IcsCalendar`
  - `IcsCalendar::add_todo(todo) → &mut IcsCalendar`
  - `IcsCalendar::to_string() → String`
- Use `uuid` crate (already common in Rust projects) or hash-based UID generation for each VTODO
- Generate `DTSTAMP` using `chrono::Utc::now()`
- Follow RFC 5545 line folding at 75 octets
- Test with an RFC 5545 validator

**Note**: If `uuid` is not already in `Cargo.toml`, a simple hash-based UID (e.g., `sha256(date + summary + priority)`) avoids adding yet another dependency.
