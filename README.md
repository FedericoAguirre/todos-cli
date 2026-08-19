# todos-cli

## Objective

todos-cli is a Rust command-line tool to generate a monthly TODOs file compatible with the [Obsidian](https://obsidian.md/) software using the [Ivy Lee method](https://doodle.com/es/the-ivy-lee-method-six-tasks-to-productivity/) for productivity.

## Usage

The CLI accepts three arguments:

- `--year` or `-y`: Year for the TODOs file
- `--month` or `-m`: Month for the TODOs file
- `--path` or `-p`: Output file path for the generated files

**Note**: If the `--path` argument is omitted it is read from env variable `TODOS_DEFAULT_PATH`.

Examples:

```sh
cargo run -- --year 2025 --month 9 --path ~/Documents/Mapas/TODOS
cargo run -- --year 2025 --month 9
cargo run -- -y 2025 -m 9 -p ~/Documents/Mapas/TODOS
cargo run -- -y 2025 -m 9
```

### Todos Calendar

Since v0.2.0, the CLI also generates an **ICS calendar file** (`TODOS - YYYYMM.ics`) alongside the markdown file. The ICS file follows the [RFC 5545](https://tools.ietf.org/html/rfc5545) iCalendar standard and can be imported as **tasks** into task-capable processors such as Android task apps, task-oriented iOS/macOS surfaces, and CalDAV clients.

Each pending todo item from the markdown file becomes a **VTODO** component (task) with:

| ICS Field | Description |
|-----------|-------------|
| `UID` | Deterministic hash-based identifier (`<hash>@todos-cli`); identical todos get an ordinal suffix so they stay distinct |
| `DTSTART` | Date-only (`DTSTART;VALUE=DATE`) pinning the task to its todo's day |
| `SUMMARY` | `[P<N>]` prefix + description with `[[ ]]` wiki-link brackets removed |
| `PRIORITY` | Task priority `1`–`6` (1 = highest) |
| `DUE` | Due moment in floating local time — configurable per weekday + priority (see below) |
| `STATUS` | Always `NEEDS-ACTION` |
| `VALARM` | Optional reminder that fires N minutes before `DUE` (`TRIGGER;RELATED=END:-PT<N>M`) |

**Why VTODO?** Tasks are represented as checkable `VTODO` components so each todo shows up as a task (not a calendar block) in Android and Mac task surfaces and other ICS processors. Note that the standalone Calendar.app only renders `VEVENT` events — the task output targets task-capable importers, which honor `VTODO` with `DTSTART;VALUE=DATE` + `DUE`. Event-only importers ignore `VTODO` (expected behavior, not a defect).

**Task scheduling logic**: The CLI reads `templates/todos_due_times.csv` to map each weekday + priority combination to a due hour and alarm offset. If a match is found, `DUE` is set to `md.date + csv.hour` (floating local) and a `VALARM` triggers `csv.minutes` minutes before `DUE`. If no match exists, `DUE` defaults to `23:59` on the todo's day with no `VALARM`.

Example mapping (from `templates/todos_due_times.csv`):

```csv
weekday,priority,hour,minutes
Lunes,1,9:00,30
Lunes,2,16:00,30
...
```

The CSV uses Spanish weekday names (Lunes, Martes, ..., Domingo) matching the markdown output.

The ICS file is generated automatically — no extra CLI flags needed. Both `TODOS - YYYYMM.md` and `TODOS - YYYYMM.ics` are written to the same output directory.

### ENV Setting

To set the TODOS_DEFAULT_PATH env variable in all sessions:

```shell
echo 'export TODOS_DEFAULT_PATH="$HOME/Documents/Mapas/TODOS"' >> ~/.zshrc
source ~/.zshrc
```

To set the TODOS_DEFAULT_PATH env variable in the current session:

```shell
export TODOS_DEFAULT_PATH="$HOME/Documents/Mapas/TODOS"
```

## Templates

Templates are stored in the `templates/` directory:

- `header.md`: Header for the TODOs file
- `1.md` to `7.md`: Templates for each day (Monday to Sunday)

## Dependencies

- [Tera](https://keats.github.io/tera/docs/) — Templating engine for markdown generation
- [Clap](https://docs.rs/clap/latest/clap/) — Argument parsing
- [Chrono](https://docs.rs/chrono/latest/chrono/) — Date handling, calendar arithmetic, and weekday resolution

## Development

- Build: `cargo build`
- Test: `cargo test`
- Run: `cargo run -- -y 2025 -m 11 -p .`
- Format: `cargo fmt`
- Add dependencies: `cargo add <crate>`
- Release: `cargo build -r`

## Project Structure

- `src/main.rs`: CLI entry point (argument parsing, orchestration)
- `src/lib.rs`: Core logic — `Todos` struct and `create_todos_file()`
- `src/calendar.rs`: ICS calendar generation (VTODO, VALARM, RFC 5545)
- `src/parser.rs`: Markdown and CSV parsing
- `templates/`: Markdown templates (`header.md`, `1.md`–`7.md`)
- `templates/todos_due_times.csv`: Due time mapping (weekday + priority → hour + alarm)
- `Cargo.toml`: Project manifest

## Templates explanation

The project has 8 templates to create the "TODOS - &lt;YYYYMM&gt;.md" file.

The **Markdown** templates are located under the **templates/** folder.

The templating system uses the [Tera crate](https://crates.io/crates/tera) to generate the final "TODOS - &lt;YYYYMM&gt;.md" file.

### header.md

The **header.md** template has the header of the file. It contains the data to show first in the final "TODOS - &lt;YYYYMM&gt;.md" file.

It reads the YYYYMM variable that represent the year (YYYY) in 4 digits format and the month (MM) in 2 digits format.

The actual contents of **header.md** template are:

```Markdown
# TODOS {{ YYYYMM }}

---


```

### Templates from 1.md to 7.md

Each template from 1.md to 7.md represents a day of the week, starting from Monday (1.md), Tuesday (2.md), ..., until Sunday (7.md).

Each day template includes only six tasks, following the [Ivy Lee method](https://doodle.com/es/the-ivy-lee-method-six-tasks-to-productivity/) for productivity.

The templates read the YYYYMMDD variable that represent the year (YYYY) in 4 digits format, the month (MM) in 2 digits format, and the day (DD) in 2 digits format.

The tasks are designed to be displayed as checkboxes in the [Obsidian](https://obsidian.md/) software.

The actual contents of **1.md** template are:

```Markdown
## {{YYYYMMDD}} - Lunes

- [ ] 1. [[Ejercicio]]
- [ ] 2. Ticket
- [ ] 3. Trabajar en RSVR
- [ ] 4. Leer 33 estrategias de la guerra
- [ ] 5. Tarea5
- [ ] 6. Tarea6

```

In Obsidian, you can represent a task by adding a line like this:

```Markdown
- [ ] <task>
```

In this example, the numerals were added to keep track up to 6 TODOS per day.

You can add references to other Obsidian notes putting the title name between `[[ ]]`, as in the *Ejercicio* line.

```Markdown
- [ ] 1. [[Ejercicio]]
```

---

See [context.md](context.md) for more details on requirements and design.
