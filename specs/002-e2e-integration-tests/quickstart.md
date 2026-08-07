# Quickstart: E2E Integration Tests

## Prerequisites

- Rust toolchain (rustc 1.75+, cargo)
- Existing `templates/` directory with `header.md`, `1.md`–`7.md`, and `todos_due_times.csv`
- No external services or network access required

## Setup

```bash
cargo build
```

No environment variables are required — the suite passes an explicit `--path` to a unique temp directory.

## Running the Suite

```bash
# Run the full test suite (unit + integration)
cargo test

# Run only the new end-to-end suite
cargo test --test e2e_generation_test

# Lint and format checks (constitution gates)
cargo clippy
cargo fmt --check
```

## Validation Scenarios

The new `tests/e2e_generation_test.rs` covers the six user stories from the spec:

### 1. Full pipeline generation (User Story 1)
Invokes the real CLI binary against a unique temp dir; asserts exit code 0 and that both `TODOS - YYYYMM.md` and `TODOS - YYYYMM.ics` exist side-by-side.

### 2. All-7-days template rendering (User Story 2)
Recomputes expected dates with chrono (mirroring `get_days()`), asserts each `## YYYYMMDD - <Weekday>` heading appears exactly once in order, with the correct zero-padded date and Spanish weekday name, and that all 7 weekday names appear.

### 3. ICS structure & synchronization (User Story 3)
Asserts `BEGIN:VCALENDAR`…`END:VCALENDAR` wrapping and that `BEGIN:VEVENT` count equals the markdown `- [ ] ` count.

### 4. Leap-year February (User Story 4)
Asserts 29 day headings for February 2024 (leap) and 28 for February 2023 (non-leap), plus matching VEVENT counts.

### 5. Header across month boundaries (User Story 5)
Asserts the header line is exactly `# TODOS <YYYYMM>` for sample months starting on different weekdays and that day headings are contiguous.

### 6. Temp directory cleanup (User Story 6)
Asserts the temp dir is removed after a successful run; a failure-path case asserts the directory is preserved.

## Verification Artifacts

- [Data model](./data-model.md) — runtime artifacts and expected-content model
- [CLI test invocation contract](./contracts/cli-test-invocation.md) — how tests invoke the binary
- [Generated markdown contract](./contracts/generated-markdown.md) — MD structure asserted
- [Generated ICS contract](./contracts/generated-ics.md) — ICS structure asserted
- [Research](./research.md) — rationale for invocation, temp-dir, and count strategies

## Expected Outcomes

- All 19 existing tests plus the new e2e cases pass (`cargo test`).
- No production source files changed unless a test uncovers a defect.
- `tests/integration_calendar_test.rs` removed (its 2 cases absorbed into the new suite).
- Temp directories cleaned up after successful runs; no leftover artifacts under the OS temp dir.
