use std::fs;
use std::path::Path;
use std::process::Command;

fn run_cli(year: i32, month: u32, out_dir: &str) -> std::process::Output {
    Command::new("cargo")
        .args([
            "run",
            "--",
            "--year",
            &year.to_string(),
            "--month",
            &month.to_string(),
            "--path",
            out_dir,
        ])
        .output()
        .expect("Failed to run CLI")
}

#[test]
fn test_cli_generates_ics() {
    let out_dir = "/tmp/todos-test-ics";
    let _ = fs::remove_dir_all(out_dir);
    fs::create_dir_all(out_dir).unwrap();

    let output = run_cli(2026, 7, out_dir);
    assert!(output.status.success(), "CLI should exit successfully");

    let md_path = Path::new(out_dir).join("TODOS - 202607.md");
    let ics_path = Path::new(out_dir).join("TODOS - 202607.ics");

    assert!(md_path.exists(), "MD file should exist");
    assert!(ics_path.exists(), "ICS file should exist");

    let ics_content = fs::read_to_string(&ics_path).unwrap();
    assert!(
        ics_content.contains("BEGIN:VCALENDAR"),
        "ICS should have VCALENDAR start"
    );
    assert!(
        ics_content.contains("END:VCALENDAR"),
        "ICS should have VCALENDAR end"
    );

    let md_content = fs::read_to_string(&md_path).unwrap();
    let md_todo_count = md_content.matches("- [ ]").count();
    let ics_vevent_count = ics_content.matches("BEGIN:VEVENT").count();
    assert_eq!(
        ics_vevent_count, md_todo_count,
        "VEVENT count should match todo count in MD"
    );

    fs::remove_dir_all(out_dir).unwrap();
}

#[test]
fn test_cli_calendar_for_february() {
    let out_dir = "/tmp/todos-test-feb";
    let _ = fs::remove_dir_all(out_dir);
    fs::create_dir_all(out_dir).unwrap();

    let output = run_cli(2024, 2, out_dir);
    assert!(
        output.status.success(),
        "CLI should exit successfully for Feb 2024"
    );

    let ics_path = Path::new(out_dir).join("TODOS - 202402.ics");
    assert!(ics_path.exists(), "ICS file should exist");

    let ics_content = fs::read_to_string(&ics_path).unwrap();
    let vevent_count = ics_content.matches("BEGIN:VEVENT").count();
    assert!(vevent_count > 0, "Feb 2024 should have VEVENTs");
    assert!(
        ics_content.contains("VERSION:2.0"),
        "Should have ICS version"
    );

    fs::remove_dir_all(out_dir).unwrap();
}
