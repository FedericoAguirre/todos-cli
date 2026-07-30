use clap::Parser;
use std::fs;
use std::path::PathBuf;
use todos_cli::calendar::generate_ics;
use todos_cli::parser::{CsvParser, MdParser};
use todos_cli::{Todos, create_todos_file};

/// Command line arguments for todos-cli
#[derive(Parser)]
#[command(author = "Federico Aguirre", version = env!("CARGO_PKG_VERSION"), about = "This CLI creates a TODO file for a given month.", long_about = None)]
struct Args {
    /// Year for the TODOS file
    #[arg(short = 'y', long, required = true)]
    year: i32,

    /// Month for the TODOS file (1-12)
    #[arg(short = 'm', long, required = true, value_parser = clap::value_parser!(u32).range(1..=12))]
    month: u32,

    /// Output file path for the TODOS file
    #[arg(short = 'p', long)]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let path = args.path.unwrap_or_else(|| {
        if let Ok(env_path) = std::env::var("TODOS_DEFAULT_PATH") {
            env_path
        } else {
            String::from(".")
        }
    });

    let output_path: PathBuf = path.into();
    let todos = Todos::new(args.year, args.month, output_path.clone());
    if let Err(e) = create_todos_file(&todos) {
        eprintln!("Error creating TODOS file: {}", e);
        std::process::exit(1);
    }

    let filename = format!("TODOS - {:04}{:02}.md", args.year, args.month);
    let md_path = output_path.join(&filename);
    if let Ok(md_content) = fs::read_to_string(&md_path) {
        let items = MdParser::parse(&md_content);
        let rules = fs::read_to_string("templates/todos_due_times.csv")
            .map(|csv| CsvParser::parse(&csv))
            .unwrap_or_else(|e| {
                eprintln!("Warning: could not read CSV: {}, using defaults", e);
                vec![]
            });
        let ics_name = format!("TODOS - {:04}{:02}", args.year, args.month);
        let ics_content = generate_ics(&ics_name, &items, &rules);
        let ics_filename = format!("TODOS - {:04}{:02}.ics", args.year, args.month);
        let ics_path = output_path.join(&ics_filename);
        if let Err(e) = fs::write(&ics_path, ics_content) {
            eprintln!("Error creating ICS file: {}", e);
        } else {
            println!("Archivo ICS creado: {}", ics_path.display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_valid_args() {
        let args = vec!["test-bin", "-y", "2025", "-m", "10", "-p", "~"];
        let parsed = Args::parse_from(args);
        assert_eq!(parsed.year, 2025);
        assert_eq!(parsed.month, 10);
        assert_eq!(parsed.path.as_deref(), Some("~"));
    }

    #[test]
    fn parses_long_args() {
        let args = vec!["test-bin", "--year", "2024", "--month", "1", "--path", "~"];
        let parsed = Args::parse_from(args);
        assert_eq!(parsed.year, 2024);
        assert_eq!(parsed.month, 1);
        assert_eq!(parsed.path.as_deref(), Some("~"));
    }

    #[test]
    fn path_is_optional() {
        let args = vec!["test-bin", "-y", "2025", "-m", "5"];
        let parsed = Args::parse_from(args);
        assert_eq!(parsed.year, 2025);
        assert_eq!(parsed.month, 5);
        assert!(parsed.path.is_none());
    }

    // #[test]
    // #[should_panic(
    //     expected = "error: unexpected argument found"
    // )]
    // fn fails_on_invalid_month() {
    //     let args = vec!["test-bin", "-y", "2025", "-m", "13"];
    //     let _ = Args::parse_from(args);
    // }

    #[test]
    fn verify_args() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
