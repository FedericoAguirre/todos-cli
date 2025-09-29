# Commands

## Project initialization

These commands were used to create a light executable.

```bash
cargo add tera --features=preserve_order --no-default-features && \
cargo add clap --no-default-features --features=derive && \
cargo add chrono --no-default-features --features=std
```