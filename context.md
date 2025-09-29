# todos-cli

## Objective

The todos-cli, programmed in Rust, has to create a "TODOS - YYYYMM.md" file is created from
templates.

Each template has a number 1.md for Monday, 2.md for Tuesday, and so on until
7.md for Sunday. There is another header.md template which represents the header
section of the "TODOS - YYYYMM.md" file.

The todos-cli will receive 3 input arguments:

1. Year (--year or -y). This represent the year for creating the "TODOS -
   YYYYMM.md" file.
2. Month (--month or -m). This represent the month for creating the "TODOS -
   YYYYMM.md" file.
3. File (--path or -p). This represent the output path for the "TODOS -
   YYYYMM.md" output file.

**Note**: If the --path is argument is ommited it is read from env variable TODOS_DEFAULT_PATH 

## Requirments

These are the Rust crates required for the **todos-cli**:

- [Tera](https://keats-github-io.translate.goog/tera/docs/) for templating.
- [Clap](https://docs.rs/clap/latest/clap/) for reading arguments.
- [Chrono](https://docs.rs/chrono/latest/chrono/) for dates handling.

