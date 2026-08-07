# CLI Test Invocation Contract

## Purpose

Defines how the end-to-end integration suite invokes the CLI binary, so tests exercise the real production entry point (Constitution Principle V).

## Invocation

```bash
# From within integration tests (tests/), the compiled binary path is provided by Cargo:
<CARGO_BIN_EXE_todos-cli> --year <YYYY> --month <MM> --path <TEMP_DIR>
```

Equivalent shell form for manual runs:

```bash
cargo run -- --year <YYYY> --month <MM> --path <TEMP_DIR>
```

## Arguments

| Arg | Required | Valid range | Behavior |
|-----|----------|-------------|----------|
| `--year` / `-y` | yes | any integer year | Sets target year |
| `--month` / `-m` | yes | 1–12 | Sets target month; rejected by clap outside range |
| `--path` / `-p` | no | any path | Output directory; falls back to `TODOS_DEFAULT_PATH`, then `.` |

## Exit Codes

| Code | Condition |
|------|-----------|
| 0 | Markdown generated successfully (ICS generated or warned) |
| 1 | Markdown generation failed (e.g., template error, unwritable path) |
| non-zero (clap) | Invalid arguments (e.g., month 13) rejected before generation |

## Output Files

| File | Condition | Location |
|------|-----------|----------|
| `TODOS - YYYYMM.md` | Always on exit 0 | `--path` directory |
| `TODOS - YYYYMM.ics` | Always alongside .md on exit 0 | Same directory as .md |

## Test Guarantees

- Tests invoke the real binary via `env!("CARGO_BIN_EXE_todos-cli")` — not `cargo run`.
- Output is written to a unique temp directory, never the repo or fixed `/tmp` paths.
- Success path removes the temp directory; failure path preserves it for debugging.
