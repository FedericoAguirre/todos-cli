# Research: E2E Integration Test Strategy

## Research Tasks

### 1. Invoking the CLI binary from integration tests

**Finding**: Integration tests in `tests/` are compiled as separate crates and run in parallel by default. Two options for invoking the CLI:

| Approach | Speed | Reliability | Parallel-safe |
|----------|-------|-------------|---------------|
| `Command::new("cargo").args(["run", "--", ...])` | Slow (cargo re-evaluates build each call) | Fragile (depends on cargo being on PATH; stdout mixes build output) | Yes, but noisy |
| `env!("CARGO_BIN_EXE_todos-cli")` | Fast (direct binary path) | Robust (Cargo guarantees the env var at test compile time) | Yes |

The existing `tests/integration_calendar_test.rs` uses the `cargo run` approach. The new suite should use `env!("CARGO_BIN_EXE_todos-cli")` — Cargo sets this variable for integration tests of binary targets, giving a direct path to the compiled binary.

### 2. Temp directory strategy

**Finding**: The old tests use fixed paths (`/tmp/todos-test-ics`, `/tmp/todos-test-feb`). This risks collisions, stale state from prior runs, and leftover artifacts on failure.

Best practice: derive a unique directory per test under `std::env::temp_dir()` using process id + a counter/timestamp, e.g. `std::env::temp_dir().join(format!("todos-e2e-{}-{}", process::id(), nanos))`. No `tempfile` crate needed — the constitution permits dev-dependencies but prefers none when `std` suffices. The 001 feature already established the "no new deps" pattern.

Cleanup semantics (from Clarification 2026-08-07): remove the dir on success; preserve on failure. Implemented by only calling `fs::remove_dir_all` on the success path of each test.

### 3. Verifying template rendering for all 7 weekdays

**Finding**: `create_todos_file` renders each date via template `{weekday}.md` where `weekday = date.weekday().number_from_monday()` (1=Monday … 7=Sunday). The generated markdown contains headings of the form `## YYYYMMDD - <Spanish weekday>`.

To verify "all 7 days rendered correctly" the test should:
- Recompute the expected date sequence using the same logic as `get_days()` (chrono `NaiveDate` arithmetic, including leap-year February = 29 days).
- Assert each heading line `## YYYYMMDD - <Weekday>` appears exactly once, in order.
- Assert the weekday name matches the actual calendar weekday of that date, and that all 7 distinct weekday names (Lunes…Domingo) appear across a full month.
- Spanish names are the canonical set used by the templates and README.

A month with 31 days always contains all 7 weekdays at least once; months with 28–30 days also contain all 7 (any ≥28-day span covers every weekday). So a single month assertion is sufficient to prove all 7 templates are exercised.

### 4. VEVENT ↔ todo synchronization

**Finding**: `main.rs` parses the generated markdown with `MdParser` and generates one `VEVENT` per `TodoItem`. The number of `BEGIN:VEVENT` occurrences in the ICS must equal the number of `- [ ] ` lines in the markdown. Both are plain-text countable in the test via `matches("BEGIN:VEVENT").count()` and `matches("- [ ] ").count()`. Confirmed by existing `tests/integration_calendar_test.rs` which does exactly this.

The current templates contain a fixed 5 todo items per day (`1.md`…`7.md`), so total counts are deterministic: 31-day month → 155 items, 30-day → 150, 29-day → 145, 28-day → 140. The test should assert against the parsed markdown count (source of truth) rather than a hardcoded number, and additionally hardcode the February totals as a cross-check.

### 5. Header and month-boundary validation

**Finding**: `header.md` renders `# TODOS {{ YYYYMM }}`. The header is identical regardless of which weekday the month starts on, so "header across month boundaries" is validated by generating a set of months that start on different weekdays (e.g., sample months starting Monday through Sunday) and asserting the first line is always `# TODOS <YYYYMM>` and all `## YYYYMMDD` headings are contiguous (no gaps, no duplicates).

### 6. Leap-year February

**Finding**: `get_days()` uses chrono's `leap_year()`, which correctly implements the Gregorian rule (divisible by 4, except centuries not divisible by 400). 2024 → 29 days, 2023 → 28 days, 1900 → 28, 2000 → 29. The e2e test covers 2024 (leap, 29 day headings) and 2023 (non-leap, 28 day headings), matching the spec's FR-009.

## Decision

**Chosen approach**:
1. Use `env!("CARGO_BIN_EXE_todos-cli")` to invoke the binary directly (fast, robust, parallel-safe).
2. Create unique temp dirs via `std::env::temp_dir()` + pid + timestamp; clean up on success only.
3. Compute expected dates in-test with `chrono` (mirroring `get_days()` logic) and assert exact heading lines, weekday names, and ordering.
4. Assert `BEGIN:VEVENT` count == `- [ ] ` count, derived from the generated files themselves.
5. No new dependencies; std + existing `chrono` only.
6. Remove `tests/integration_calendar_test.rs`, absorbing its 2 cases into the new suite.

**Rationale**: Mirrors the 001 feature's "no new deps" precedent, produces deterministic parallel-safe tests, and validates the real CLI binary end-to-end (satisfying Principle V and the spec's FR-012).

**Alternatives considered**:
- **`cargo run` invocation**: Rejected — slow, noisy stdout, depends on cargo availability at test time.
- **`tempfile` dev-dependency**: Rejected — adds a crate when `std::env::temp_dir()` + manual creation suffices; the constitution favors minimal dependencies.
- **Fixed temp paths**: Rejected — collision-prone and leaves stale artifacts, violating the cleanup requirement.
- **Hardcoded todo totals**: Rejected as the primary check — deriving counts from the actual generated files is the truer synchronization verification; hardcoded February totals kept as a secondary cross-check.

## Implementation Guidance

- Helper `run_cli(year, month, out_dir) -> Output` using `Command::new(env!("CARGO_BIN_EXE_todos-cli"))`.
- Helper `temp_dir(prefix) -> PathBuf` returning a unique directory (created, not yet populated).
- Helper to count occurrences of a substring.
- Spanish weekday name array `["Lunes", "Martes", "Miércoles", "Jueves", "Viernes", "Sábado", "Domingo"]` indexed by `weekday.number_from_monday() - 1`.
- The ICS file is named `TODOS - YYYYMM.ics`; the markdown `TODOS - YYYYMM.md`; both live in the same temp dir.
