use chrono::Datelike;
use chrono::NaiveDate;
use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

pub struct Todos {
    // Add fields as needed, e.g. year, month, days, etc.
    pub year: i32,
    pub month: u32,
    pub path: PathBuf,
}

impl Todos {
    pub fn new(year: i32, month: u32, path: PathBuf) -> Self {
        Self { year, month, path }
    }
    pub fn get_days(&self) -> Vec<chrono::NaiveDate> {
        let days_in_month = match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if NaiveDate::from_ymd_opt(self.year, 2, 1)
                    .unwrap()
                    .leap_year()
                {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        };
        (1..=days_in_month)
            .filter_map(|day| NaiveDate::from_ymd_opt(self.year, self.month, day))
            .collect()
    }
}

pub fn create_todos_file(todos: &Todos) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("TODOS - {:04}{:02}.md", todos.year, todos.month);

    let output_path = todos.path.clone().join(&filename);

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

    println!("Archivo TODOS creado: {}", output_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_get_days_31_days_month() {
        let todos = Todos::new(2024, 1, PathBuf::from(".")); // January
        let days = todos.get_days();
        assert_eq!(days.len(), 31);
        assert_eq!(
            days.first(),
            Some(&NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        );
        assert_eq!(
            days.last(),
            Some(&NaiveDate::from_ymd_opt(2024, 1, 31).unwrap())
        );
    }

    #[test]
    fn test_get_days_30_days_month() {
        let todos = Todos::new(2024, 4, PathBuf::from(".")); // April
        let days = todos.get_days();
        assert_eq!(days.len(), 30);
        assert_eq!(
            days.first(),
            Some(&NaiveDate::from_ymd_opt(2024, 4, 1).unwrap())
        );
        assert_eq!(
            days.last(),
            Some(&NaiveDate::from_ymd_opt(2024, 4, 30).unwrap())
        );
    }

    #[test]
    fn test_get_days_february_leap_year() {
        let todos = Todos::new(2024, 2, PathBuf::from(".")); // Leap year
        let days = todos.get_days();
        assert_eq!(days.len(), 29);
        assert_eq!(
            days.first(),
            Some(&NaiveDate::from_ymd_opt(2024, 2, 1).unwrap())
        );
        assert_eq!(
            days.last(),
            Some(&NaiveDate::from_ymd_opt(2024, 2, 29).unwrap())
        );
    }

    #[test]
    fn test_get_days_february_non_leap_year() {
        let todos = Todos::new(2023, 2, PathBuf::from(".")); // Non leap year
        let days = todos.get_days();
        assert_eq!(days.len(), 28);
        assert_eq!(
            days.first(),
            Some(&NaiveDate::from_ymd_opt(2023, 2, 1).unwrap())
        );
        assert_eq!(
            days.last(),
            Some(&NaiveDate::from_ymd_opt(2023, 2, 28).unwrap())
        );
    }

    #[test]
    fn test_get_days_february_century_non_leap() {
        let todos = Todos::new(1900, 2, PathBuf::from(".")); // 1900 is not a leap year
        let days = todos.get_days();
        assert_eq!(days.len(), 28);
        assert_eq!(
            days.last(),
            Some(&NaiveDate::from_ymd_opt(1900, 2, 28).unwrap())
        );
    }

    #[test]
    fn test_get_days_february_century_leap() {
        let todos = Todos::new(2000, 2, PathBuf::from(".")); // 2000 is a leap year
        let days = todos.get_days();
        assert_eq!(days.len(), 29);
        assert_eq!(
            days.last(),
            Some(&NaiveDate::from_ymd_opt(2000, 2, 29).unwrap())
        );
    }

    #[test]
    fn test_get_days_invalid_month() {
        let todos = Todos::new(2024, 13, PathBuf::from(".")); // Invalid month
        let days = todos.get_days();
        assert!(days.is_empty());
    }

    #[test]
    fn test_create_todos_file() {
        let todos = Todos::new(2024, 2, PathBuf::from("."));
        let result = create_todos_file(&todos);
        assert!(result.is_ok());

        let expected_file = Path::new(".").join("TODOS - 202402.md");
        assert!(expected_file.exists());

        let first_line = fs::read_to_string(&expected_file)
            .unwrap()
            .lines()
            .next()
            .unwrap()
            .to_string();
        assert_eq!(first_line, "# TODOS 202402");

        let days = vec![
            "## 20240201 - Jueves",
            "## 20240202 - Viernes",
            "## 20240203 - Sábado",
            "## 20240204 - Domingo",
            "## 20240205 - Lunes",
            "## 20240206 - Martes",
            "## 20240207 - Miércoles",
            "## 20240208 - Jueves",
            "## 20240209 - Viernes",
            "## 20240210 - Sábado",
            "## 20240211 - Domingo",
            "## 20240212 - Lunes",
            "## 20240213 - Martes",
            "## 20240214 - Miércoles",
            "## 20240215 - Jueves",
            "## 20240216 - Viernes",
            "## 20240217 - Sábado",
            "## 20240218 - Domingo",
            "## 20240219 - Lunes",
            "## 20240220 - Martes",
            "## 20240221 - Miércoles",
            "## 20240222 - Jueves",
            "## 20240223 - Viernes",
            "## 20240224 - Sábado",
            "## 20240225 - Domingo",
            "## 20240226 - Lunes",
            "## 20240227 - Martes",
            "## 20240228 - Miércoles",
            "## 20240229 - Jueves",
        ];

        let file_days: Vec<String> = fs::read_to_string(&expected_file)
            .unwrap()
            .lines()
            .filter(|line| line.starts_with("## "))
            .map(|s| s.to_string())
            .collect();

        assert_eq!(file_days, days);

        // Clean up
        fs::remove_file(expected_file).unwrap();
    }
}
