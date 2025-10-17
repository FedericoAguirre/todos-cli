# todos-cli

## Objective

todos-cli is a Rust command-line tool to generate a monthly TODOs file compatible with the [Obsidian](https://obsidian.md/) software using the [Ivy Lee method](https://doodle.com/es/the-ivy-lee-method-six-tasks-to-productivity/) for productivity.

## Usage

The CLI accepts three arguments:

- `--year` or `-y`: Year for the TODOs file
- `--month` or `-m`: Month for the TODOs file
- `--path` or `-p`: Output file path for the generated markdown

**Note**: If the --path is argument is omitted it is read from env variable TODOS_DEFAULT_PATH.

Examples:

```sh
cargo run -- --year 2025 --month 9 --path ~/Documents/Mapas/TODOS
cargo run -- --year 2025 --month 9
cargo run -- -y 2025 -m 9 -p ~/Documents/Mapas/TODOS
cargo run -- -y 2025 -m 9
```

### ENV Setting

To set the TODOS_DEFAULT_PATH env variable in all sessions:

```shell
echo 'export TODOS_DEFAULT_PATH="/Users/$USER/Documents/Mapas/TODOS"' >> ~/.zshrc
source ~/.zshrc
```

To set the TODOS_DEFAULT_PATH env variable in the current session:

```shell
export TODOS_DEFAULT_PATH="/Users/$USER/Documents/Mapas/TODOS"
```

## Templates

Templates are stored in the `templates/` directory:

- `header.md`: Header for the TODOs file
- `1.md` to `7.md`: Templates for each day (Monday to Sunday)

## Dependencies

- [Tera](https://keats.github.io/tera/docs/) — Templating engine
- [Clap](https://docs.rs/clap/latest/clap/) — Argument parsing
- [Chrono](https://docs.rs/chrono/latest/chrono/) — Date handling

## Development

- Build: `cargo build`
- Test: `cargo test`
- Run: `cargo run -- -y 2025 -m 11 -p .`
- Format: `cargo fmt`
- Add dependencies: `cargo add <crate>`
- Release: `cargo build -r`

## Project Structure

- `src/main.rs`: Main CLI logic
- `src/lib.rs`: todos-cli logic
- `templates/`: Markdown templates
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
