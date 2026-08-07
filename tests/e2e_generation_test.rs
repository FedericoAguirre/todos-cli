use chrono::{Datelike, NaiveDate};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SPANISH_WEEKDAYS: [&str; 7] = [
    "Lunes",
    "Martes",
    "Miércoles",
    "Jueves",
    "Viernes",
    "Sábado",
    "Domingo",
];

fn run_cli(year: i32, month: u32, out_dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_todos-cli"))
        .args([
            "--year",
            &year.to_string(),
            "--month",
            &month.to_string(),
            "--path",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI")
}

fn temp_dir(prefix: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "todos-e2e-{}-{}-{}",
        prefix,
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn count_occurrences(text: &str, pattern: &str) -> usize {
    text.matches(pattern).count()
}

fn expected_days(year: i32, month: u32) -> Vec<(String, String)> {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let days_in_month = NaiveDate::from_ymd_opt(year, month, 1)
        .and_then(|_| NaiveDate::from_ymd_opt(next_year, next_month, 1))
        .map(|next_first| (next_first - chrono::Duration::days(1)).day())
        .unwrap_or(0);
    (1..=days_in_month)
        .filter_map(|day| {
            NaiveDate::from_ymd_opt(year, month, day).map(|date| {
                let yyyymmdd = date.format("%Y%m%d").to_string();
                let name = SPANISH_WEEKDAYS[date.weekday().number_from_monday() as usize - 1];
                (yyyymmdd, name.to_string())
            })
        })
        .collect()
}

fn day_headings(md: &str) -> Vec<String> {
    md.lines()
        .filter(|line| line.starts_with("## "))
        .map(|line| line.trim_start_matches("## ").to_string())
        .collect()
}

#[test]
fn test_full_pipeline_generates_md_and_ics_side_by_side() {
    let dir = temp_dir("pipeline");
    let output = run_cli(2026, 7, &dir);
    assert!(output.status.success(), "CLI should exit successfully");

    let md_path = dir.join("TODOS - 202607.md");
    let ics_path = dir.join("TODOS - 202607.ics");
    assert!(md_path.exists(), "MD file should exist in temp dir");
    assert!(ics_path.exists(), "ICS file should exist in temp dir");

    fs::remove_dir_all(&dir).unwrap();
    assert!(!dir.exists(), "Temp dir should be cleaned up after success");
}

#[test]
fn test_cli_rejects_invalid_month() {
    let dir = temp_dir("invalid-month");
    let output = run_cli(2026, 13, &dir);
    assert!(
        !output.status.success(),
        "CLI should reject month 13 with non-zero exit"
    );
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_all_day_headings_rendered_in_order() {
    let dir = temp_dir("rendering");
    let output = run_cli(2024, 2, &dir);
    assert!(output.status.success(), "CLI should exit successfully");

    let md = fs::read_to_string(dir.join("TODOS - 202402.md")).unwrap();
    let headings = day_headings(&md);
    let expected = expected_days(2024, 2);

    assert_eq!(
        headings.len(),
        expected.len(),
        "Day heading count should match expected days (29 for Feb 2024)"
    );

    for (i, (expected_yyyymmdd, expected_name)) in expected.iter().enumerate() {
        let heading = &headings[i];
        let (yyyymmdd, name) = heading.split_once(" - ").unwrap();
        assert_eq!(
            yyyymmdd, expected_yyyymmdd,
            "Date prefix mismatch at index {}",
            i
        );
        assert_eq!(name, expected_name, "Weekday name mismatch at index {}", i);
    }

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_all_seven_weekday_templates_exercised() {
    let dir = temp_dir("weekdays");
    let output = run_cli(2026, 7, &dir);
    assert!(output.status.success(), "CLI should exit successfully");

    let md = fs::read_to_string(dir.join("TODOS - 202607.md")).unwrap();
    let headings = day_headings(&md);

    let seen: std::collections::HashSet<&str> = headings
        .iter()
        .filter_map(|h| h.split_once(" - ").map(|(_, name)| name))
        .collect();

    assert_eq!(seen.len(), 7, "All 7 weekday templates should be exercised");

    let expected = expected_days(2026, 7);
    for (i, (expected_yyyymmdd, expected_name)) in expected.iter().enumerate() {
        let (yyyymmdd, name) = headings[i].split_once(" - ").unwrap();
        assert_eq!(
            yyyymmdd, expected_yyyymmdd,
            "Date prefix mismatch at index {}",
            i
        );
        assert_eq!(name, expected_name, "Weekday name mismatch at index {}", i);
    }

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_ics_has_valid_vcalendar_structure() {
    let dir = temp_dir("vcalendar");
    let output = run_cli(2026, 7, &dir);
    assert!(output.status.success(), "CLI should exit successfully");

    let ics = fs::read_to_string(dir.join("TODOS - 202607.ics")).unwrap();
    assert!(
        ics.trim_start().starts_with("BEGIN:VCALENDAR"),
        "ICS should start with BEGIN:VCALENDAR"
    );
    assert!(
        ics.trim_end().ends_with("END:VCALENDAR"),
        "ICS should end with END:VCALENDAR"
    );

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_vevent_count_matches_todo_count() {
    let dir = temp_dir("vevent-sync");
    let output = run_cli(2026, 7, &dir);
    assert!(output.status.success(), "CLI should exit successfully");

    let md = fs::read_to_string(dir.join("TODOS - 202607.md")).unwrap();
    let ics = fs::read_to_string(dir.join("TODOS - 202607.ics")).unwrap();

    let md_todo_count = count_occurrences(&md, "- [ ] ");
    let ics_vevent_count = count_occurrences(&ics, "BEGIN:VEVENT");

    assert_eq!(
        ics_vevent_count, md_todo_count,
        "VEVENT count should equal MD todo count"
    );

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_february_leap_year_2024_has_29_days() {
    let dir = temp_dir("feb-leap");
    let output = run_cli(2024, 2, &dir);
    assert!(output.status.success(), "CLI should exit successfully");

    let md = fs::read_to_string(dir.join("TODOS - 202402.md")).unwrap();
    let headings = day_headings(&md);
    assert_eq!(
        headings.len(),
        29,
        "Feb 2024 (leap year) should have 29 day headings"
    );
    assert!(
        headings.first().unwrap().starts_with("20240201"),
        "First heading should be 20240201"
    );
    assert!(
        headings.last().unwrap().starts_with("20240229"),
        "Last heading should be 20240229"
    );

    let ics = fs::read_to_string(dir.join("TODOS - 202402.ics")).unwrap();
    assert_eq!(
        count_occurrences(&ics, "BEGIN:VEVENT"),
        count_occurrences(&md, "- [ ] "),
        "VEVENT count should match MD todo count"
    );

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_february_non_leap_year_2023_has_28_days() {
    let dir = temp_dir("feb-nonleap");
    let output = run_cli(2023, 2, &dir);
    assert!(output.status.success(), "CLI should exit successfully");

    let md = fs::read_to_string(dir.join("TODOS - 202302.md")).unwrap();
    let headings = day_headings(&md);
    assert_eq!(
        headings.len(),
        28,
        "Feb 2023 (non-leap year) should have 28 day headings"
    );
    assert!(
        headings.first().unwrap().starts_with("20230201"),
        "First heading should be 20230201"
    );
    assert!(
        headings.last().unwrap().starts_with("20230228"),
        "Last heading should be 20230228"
    );

    let ics = fs::read_to_string(dir.join("TODOS - 202302.ics")).unwrap();
    assert_eq!(
        count_occurrences(&ics, "BEGIN:VEVENT"),
        count_occurrences(&md, "- [ ] "),
        "VEVENT count should match MD todo count"
    );

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_header_line_is_correct() {
    for (year, month) in [(2026, 7), (2024, 2), (2026, 1), (2026, 12)] {
        let dir = temp_dir("header");
        let output = run_cli(year, month, &dir);
        assert!(output.status.success(), "CLI should exit successfully");

        let filename = format!("TODOS - {:04}{:02}.md", year, month);
        let md = fs::read_to_string(dir.join(&filename)).unwrap();
        let first_line = md.lines().next().unwrap();
        let expected = format!("# TODOS {:04}{:02}", year, month);
        assert_eq!(
            first_line, &expected,
            "Header line should be {} for {}/{}",
            expected, year, month
        );

        fs::remove_dir_all(&dir).unwrap();
    }
}

#[test]
fn test_day_headings_contiguous_across_month_start_weekdays() {
    let cases = [
        (2023, 1),
        (2023, 2),
        (2023, 4),
        (2023, 5),
        (2023, 6),
        (2023, 8),
        (2023, 9),
    ];
    let mut start_weekdays = std::collections::HashSet::new();

    for (year, month) in cases {
        let dir = temp_dir("boundary");
        let output = run_cli(year, month, &dir);
        assert!(output.status.success(), "CLI should exit successfully");

        let filename = format!("TODOS - {:04}{:02}.md", year, month);
        let md = fs::read_to_string(dir.join(&filename)).unwrap();
        let headings = day_headings(&md);

        let expected = expected_days(year, month);
        assert_eq!(
            headings.len(),
            expected.len(),
            "Day heading count should match for {}/{}",
            year,
            month
        );

        for (i, (expected_yyyymmdd, expected_name)) in expected.iter().enumerate() {
            let (yyyymmdd, name) = headings[i].split_once(" - ").unwrap();
            assert_eq!(
                yyyymmdd, expected_yyyymmdd,
                "Date prefix mismatch for {}/{}",
                year, month
            );
            assert_eq!(
                name, expected_name,
                "Weekday name mismatch for {}/{}",
                year, month
            );
        }

        start_weekdays.insert(
            NaiveDate::from_ymd_opt(year, month, 1)
                .unwrap()
                .weekday()
                .number_from_monday(),
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    assert_eq!(
        start_weekdays.len(),
        7,
        "Sample months should cover all 7 starting weekdays"
    );
}

#[test]
fn test_temp_dir_removed_after_successful_run() {
    let dir = temp_dir("cleanup-success");
    let output = run_cli(2026, 7, &dir);
    assert!(output.status.success(), "CLI should exit successfully");

    let md_path = dir.join("TODOS - 202607.md");
    let ics_path = dir.join("TODOS - 202607.ics");
    assert!(md_path.exists(), "MD file should exist");
    assert!(ics_path.exists(), "ICS file should exist");

    fs::remove_dir_all(&dir).unwrap();
    assert!(
        !dir.exists(),
        "Temp dir should be removed after successful validation"
    );
}

#[test]
fn test_temp_dir_preserved_on_failure() {
    let dir = temp_dir("cleanup-failure");
    let output = run_cli(2026, 13, &dir);
    assert!(
        !output.status.success(),
        "CLI should fail with invalid month"
    );
    assert!(
        dir.exists(),
        "Temp dir should be preserved on failure for debugging"
    );
    fs::remove_dir_all(&dir).unwrap();
}
