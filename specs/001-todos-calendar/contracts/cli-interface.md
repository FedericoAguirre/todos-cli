# CLI Interface Contract

## Invocation

```bash
todos-cli --year <YYYY> --month <MM> [--path <OUTPUT_DIR>]
```

## Behavior Change

No new CLI arguments are added. The ICS file is a **side-effect output** — always generated when the markdown file is generated.

## Output Files

| File | Condition | Location |
|------|-----------|----------|
| `TODOS - YYYYMM.md` | Always (unchanged) | `--path` or `TODOS_DEFAULT_PATH` |
| `TODOS - YYYYMM.ics` | Always alongside .md | Same directory as .md |

## Error Behavior

| Scenario | Behavior |
|----------|----------|
| CSV file missing | Warning on stderr; generate ICS with end-of-day due, no alarms |
| CSV parse error | Warning on stderr; skip malformed rows, use defaults for missing |
| Malformed todo line | Warning on stderr; skip line, continue processing |
| No todos in month | Generate valid ICS with zero VTODOs |
| Invalid year/month | Existing clap validation rejects before generation |
