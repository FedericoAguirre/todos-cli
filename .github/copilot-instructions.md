# Copilot Instructions for `todos-cli`

## Project Overview
- This is a Rust command-line application named `todos-cli`.
- The entry point is `src/main.rs`.
- The project is at an early stage; currently, it only prints "Hello, world!".

## Key Files & Structure
- `src/main.rs`: Main entry point for the CLI logic.
- `Cargo.toml`: Project manifest for dependencies, metadata, and build configuration.
- No additional modules, libraries, or external dependencies are present yet.

## Developer Workflows
- **Build:**
  - Run `cargo build` to compile the project.
- **Run:**
  - Use `cargo run` to execute the CLI.
- **Test:**
  - No tests are defined yet. Add tests in `tests/` or as `#[cfg(test)]` modules in source files.
- **Debug:**
  - Use `cargo run` or `cargo test` with `RUST_BACKTRACE=1` for stack traces.

## Conventions & Patterns
- Standard Rust CLI project layout.
- Follow idiomatic Rust patterns for error handling, modularization, and documentation.
- Add new features as subcommands or arguments in `main.rs` or split into modules as the project grows.

## Integration & Extensibility
- No external APIs or integrations yet.
- Add dependencies by editing `Cargo.toml` and running `cargo build`.

## Example: Adding a Subcommand
```rust
// In src/main.rs
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "add" {
        // handle 'add' subcommand
    }
}
```

## When in Doubt
- Follow Rust best practices and keep the CLI interface simple and discoverable.
- Document new commands and options in the README as the project evolves.
