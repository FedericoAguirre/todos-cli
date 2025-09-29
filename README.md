# todos-cli

## Objective

todos-cli is a Rust command-line tool to generate a monthly TODOs markdown file ("TODOS - YYYYMM.md") from templates.

## Usage

The CLI accepts three arguments:

- `--year` or `-y`: Year for the TODOs file
- `--month` or `-m`: Month for the TODOs file
- `--path` or `-p`: Output file path for the generated markdown

**Note**: If the --path is argument is ommited it is read from env variable TODOS_DEFAULT_PATH 

Examples:
```sh
cargo run -- --year 2025 --month 9 --path "~/Documents/Mapas"
cargo run -- --year 2025 --month 9
cargo run -- -y 2025 -m 9 -p "~/Documents/Mapas"
cargo run -- -y 2025 -m 9
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
- Run: `cargo run -- [args]`
- Format: `cargo fmt`
- Add dependencies: `cargo add <crate>`

## Project Structure

- `src/main.rs`: Main CLI logic
- `templates/`: Markdown templates
- `Cargo.toml`: Project manifest

---

See `context.md` for more details on requirements and design.
