mod todos;
use chrono::Datelike;
use clap::Parser;
use std::fs;
use std::path::Path;
use tera::{Context, Tera};
use todos::Todos;

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
    let todos = Todos::new(args.year, args.month, args.path);
    if let Err(e) = create_todos_file(&todos) {
        eprintln!("Error creating TODOS file: {}", e);
        std::process::exit(1);
    }
}

pub fn create_todos_file(todos: &Todos) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("TODOS - {:04}{:02}.md", todos.year, todos.month);

    let output_path = if let Some(ref path) = todos.path {
        Path::new(path).join(&filename)
    } else if let Ok(env_path) = std::env::var("TODOS_DEFAULT_PATH") {
        Path::new(&env_path).join(&filename)
    } else {
        Path::new(&filename).to_path_buf()
    };

    let tera = Tera::new("templates/*.md")?;

    let mut context = Context::new();
    let yyyymm = format!("{:04}{:02}", todos.year, todos.month);

    context.insert("YYYYMM", &yyyymm);

    let mut content = tera.render("header.md", &context)?;

    for date in todos.get_days() {
        let mut day_ctx = Context::new();
        let yyyymmdd = date.format("%Y%m%d").to_string();
        day_ctx.insert("YYYYMMDD", &yyyymmdd);
        let weekday = date.weekday().number_from_monday();
        let template_name = format!("{}.md", weekday);
        let day_content = tera.render(&template_name, &day_ctx)?;
        content.push_str(&day_content);
        content.push('\n');
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, content)?;

    println!("Archivo generado: {}", output_path.display());
    Ok(())
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
