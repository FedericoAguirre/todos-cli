# Spec Improvement: VEVENT vs VTODO Platform Compatibility

## The Bug

The ICS file used `VTODO` (task) components but macOS Removed VTODO import from Reminders.app starting in Monterey (2021). Importing the file into Calendar.app failed with "No valid events found" because Calendar only handles `VEVENT` (events).

## What the Spec Missed

### 1. Hardcoded VTODO as Implementation Detail

**FR-003**, **FR-005**, **FR-006**, **FR-009** committed to VTODO-specific fields (`PRIORITY`, `STATUS`, `DUE`) rather than describing the desired *outcome* — an ICS file importable into the user's calendar.

**Fix**: Requirements should have been technology-agnostic, e.g.:
> "The ICS file MUST be importable into Apple Calendar, Google Calendar, and macOS Reminders with all todo information preserved."

This would have forced the component-type decision to be validated against actual platform support.

### 2. No Platform Compatibility Audit

**US-3 / SC-003** said "import into a calendar application" but never specified which ones or tested macOS specifically. Acceptance scenarios assumed VTODO would work everywhere without verification.

**Fix**: Add a requirement:
> "The component type (VTODO/VEVENT) MUST be chosen for compatibility with all target platforms. The decision MUST be validated against the target platforms before implementation begins."

### 3. No Early Validation Against Target Apps

The spec skipped platform research before writing requirements. A quick check would have revealed that macOS Reminders dropped VTODO support years ago.

**Fix**: Add a research step to the spec template:
> "Research platform compatibility for the chosen ICS component type (VTODO/VEVENT) across all intended target applications before finalizing requirements."

## Lesson

Write specs in terms of **user-facing outcomes** ("the file can be imported into the user's calendar"), not **implementation choices** ("use VTODO with PRIORITY and STATUS"). Implementation details belong in the contract/design documents, not in the spec requirements. Platform research should precede requirement finalization.
