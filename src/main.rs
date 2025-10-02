mod todos;
use clap::Parser;
use todos::Todos;

/// Command line arguments for todos-cli
#[derive(Parser, Debug)]
#[command(author = "Federico Aguirre", version = env!("CARGO_PKG_VERSION"), about = "This CLI creates a TODO file for a given month.", long_about = None)]
struct Args {
    /// Year for the TODOS file
    #[arg(short = 'y', long)]
    year: i32,

    /// Month for the TODOS file (1-12)
    #[arg(short = 'm', long, value_parser = clap::value_parser!(u32).range(1..=12))]
    month: u32,

    /// Output file path for the TODOS file
    #[arg(short = 'p', long)]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let todos = Todos::new(args.year, args.month, args.path);
    println!(
        "Todos struct created: year={}, month={}, path={:?}",
        todos.year, todos.month, todos.path
    );
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

    #[test]
    #[should_panic(
        expected = "error: Invalid value for '--month <MONTH>': invalid digit found in string"
    )]
    fn fails_on_invalid_month() {
        let args = vec!["test-bin", "-y", "2025", "-m", "13"];
        let _ = Args::parse_from(args);
    }
}
