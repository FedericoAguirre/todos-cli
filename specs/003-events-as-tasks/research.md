# Research: VTODO (Tasks) ICS Representation

## Research Tasks

### 1. VTODO compatibility across calendar processors (Android, macOS, others)

**Finding**: Tasks in iCalendar are represented by the `VTODO` component (RFC 5545 §3.6.2), distinct from the `VEVENT` event component. Processor support is uneven:

- **Google Calendar import**: only imports `VEVENT` entries; `VTODO` blocks inside an imported `.ics` are ignored (confirmed by Google help and community reports). Tasks sync into Google Tasks / Android calendars when imported through task-capable channels (GTasks-compatible CalDAV, Google Calendar app assignment), not via the general calendar import path.
- **Android task/calendar apps** (e.g., Tasks apps honoring CalDAV/CardDAV, K-9, Business Calendar, Todo.txt-style importers): generally support `VTODO` and render due dates.
- **macOS**: Apple Calendar ignores `VTODO`; Apple **Reminders** is the macOS task surface. Reminders native `.ics` import was removed starting macOS Monterey, so users import via task-capable bridges/apps (CalDAV-facing reminders services) rather than Calendar.app. `VTODO` remains the canonical interchange format for tasks across CalDAV servers and task clients.
- **RFC 5545-compliant processors** (CalDAV servers, task clients like eM Client, Outlook task views): fully support `VTODO`.

**Conclusion**: The feature's goal is "represented as tasks ... in calendar/ICS processors." `VTODO` is the only standards-correct way to carry a task (with due date, priority, status). This intentionally reverts the project's earlier VEVENT workaround, which traded task semantics for Calendar.app importability. Where an app (e.g., Calendar.app, Google Calendar import) only handles events, nothing can make a task appear in the *calendar-event* surface — the task must surface in that app's *task/reminders* surface or a task-capable processor. This is an acceptance boundary, not a defect.

**Alternatives considered**:
- **Stay on VEVENT**: events import everywhere but are not tasks (no `DUE/PRIORITY/STATUS` semantics, occupy time blocks). Rejected — this is precisely what the feature removes.
- **Dual output (VEVENT + VTODO)**: out of scope per spec; would double calendar noise and is noted as a possible future option only.

### 2. Date/time serialization for VTODO — timezone correctness

**Finding**: Real-world reports (tududi PR #1157 / issue #1146, tasks/tasks#4139, CalConnect devguide) show the two failure modes that matter here:

- Serializing a *date-only* due as a UTC **date-time** (e.g. `DUE:<date>T235900Z`) makes iOS/macOS schedule a timed reminder at 23:59 UTC, which shifts into the *previous/next day* for non-UTC offsets, and places tasks on the wrong day. Fix is to serialize date-only as `DUE;VALUE=DATE` / `DTSTART;VALUE=DATE` (no time, no UTC).
- Mixing a `DATE`-typed `DTSTART` with a time-of-day `DUE` requires the `DUE` to carry an unambiguous wall-clock. CalConnect recommends UTC for machine timestamps (DTSTAMP) and TZID/floating for human-facing start/due times.

**Decisions**:
1. **`DTSTART;VALUE=DATE:YYYYMMDD`** — date-only, no time, no UTC. Pins the task to its todo day (clarification Q2) and avoids timed-midnight/shift artifacts.
2. **`DUE:YYYYMMDDTHHMMSS`** — *floating local time* (no `Z`, no `TZID`), consistent with the spec assumption "absolute due moment (date + local time)" and with the original 001 contract format. Floating values display at the same wall-clock in the viewer's local timezone, which is the tool's existing interpretation (data-model.md). `DUE` value must be ≥ `DTSTART` date (RFC §3.8.2.3) — guaranteed since DUE is derived from the todo's own date.
3. **`DTSTAMP:YYYYMMDDTHHMMSSZ`** — UTC with `Z`, per RFC (generation timestamp).

**Alternatives considered**:
- **`DUE` as UTC (`...Z`)**: rejected — reintroduces the midnight/day-shift problem for non-UTC users (both macOS and Android).
- **`TZID` + VTIMEZONE component**: rejected — heavier output, adds a production dependency for IANA zones or hand-rolled VTIMEZONE, violates Minimum-Dependencies spirit for a local-time tool.
- **`DURATION` instead of `DUE`**: rejected — our due moment is absolute (derived from CSV hour), not a duration.

### 3. Alarm (VALARM) anchoring for tasks

**Finding**: RFC 5545 §3.6.2 + §3.8.6.3: a `VALARM` inside a `VTODO` defaults to `RELATED=START`. Because our `DTSTART` is date-only (`VALUE=DATE`, midnight for the todo day), a bare `TRIGGER:-PT30M` would fire 30 minutes *before midnight of the todo day* — wrong. `RELATED=END` anchors the trigger to the component's *end*; for a `VTODO` the "end" is the `DUE` value (RFC §3.8.6.3: for VTODO, END relates to DUE/DURATION). So the correct form is **`TRIGGER;RELATED=END:-PT{n}M`**.

**Decision**: Emit `TRIGGER;RELATED=END:-PT{n}M` (n = CSV minutes) for tasks that have a matching rule; omit VALARM entirely when there is no rule (FR-008).

**Alternatives considered**:
- **`TRIGGER:-PT{n}M` (RELATED=START default)**: rejected — anchors to midnight DTSTART, firing ~24h too early.
- **Compute an explicit absolute trigger datetime**: unnecessary; RELATED=END is the standards-correct relative form.

### 4. Priority and status representation

**Finding**: `VTODO` supports `PRIORITY` (integer 0–9, 1 = highest) and `STATUS` (`NEEDS-ACTION` = to-do not yet started). The tool's priority range is 1–6 (1 highest). The summary currently prefixes `[P<n>]` (from the VEVENT era where priority had no native field).

**Decision**: Emit `PRIORITY:<n>` (1–6 mapped directly; RFC 1 = highest aligns), and constant `STATUS:NEEDS-ACTION` (spec FR-009/FR-006). Keep the `[P<n>]` prefix in `SUMMARY` — it is the visible priority cue across apps that ignore the `PRIORITY` field, and removing it would be an unrelated change with no spec driver.

**Alternatives considered**: emit `[P<n>]` prefix only (no PRIORITY field) — rejected, native field is more standards-correct; PRIORITY field only — rejected, text cue is still needed for app surfaces that ignore PRIORITY.

### 5. Checked-off todos

**Finding**: `MdParser::parse` only matches lines with `- [ ] ` (unchecked); `- [x]` lines are already skipped at parse time. The calendar generator only receives already-parsed `TodoItem`s.

**Decision**: No parser change needed for FR-014. Add a parser unit test proving `- [x]` produces no `TodoItem` (documents the invariant). No harness change: existing e2e counting of `- [ ]` vs components stays valid.

---

## Decision Summary

| Topic | Decision | Rationale |
|-------|----------|-----------|
| Component type | `VTODO` (task) | Only standards-correct task representation; feature goal |
| Day pinning | `DTSTART;VALUE=DATE:YYYYMMDD` | Date-only, no timezone artifacts (Q2) |
| Due moment | `DUE:YYYYMMDDTHHMMSS` (floating local) | Matches local-time model; avoids UTC shift bugs |
| Timestamp | `DTSTAMP:...Z` (UTC) | RFC-conformant generation stamp |
| Alarm | `TRIGGER;RELATED=END:-PT{n}M` | Anchors to DUE, not to midnight DTSTART |
| Priority | `PRIORITY:<1-6>` + `[P<n>]` summary prefix | Native field plus visible cue |
| Status | `STATUS:NEEDS-ACTION` | RFC to-do state; constant |
| Checked-off todos | Excluded at parse (no change) | Parser never emits items for `- [x]` |
| Dependency | None added | Manual ICS generation retained (Principle I) |