use chrono::NaiveDate;
use chrono::NaiveTime;
use todos_cli::calendar::{generate_ics, generate_uid, generate_uid_with_ordinal};
use todos_cli::parser::{DueTimeRule, TodoItem};

fn make_rule(weekday: &str, priority: u8, hour: &str, alarm_minutes: u16) -> DueTimeRule {
    DueTimeRule {
        weekday: weekday.to_string(),
        priority,
        hour: NaiveTime::parse_from_str(hour, "%H:%M").unwrap(),
        alarm_minutes,
    }
}

#[test]
fn test_ics_basic_structure() {
    let items = vec![TodoItem {
        date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        weekday_name: "Miércoles".to_string(),
        priority: 1,
        description: "Ejercicio".to_string(),
    }];

    let ics = generate_ics("TODOS - 202607", &items, &[]);

    assert!(
        ics.contains("BEGIN:VCALENDAR\r\n"),
        "Should start with VCALENDAR"
    );
    assert!(
        ics.contains("END:VCALENDAR\r\n"),
        "Should end with VCALENDAR"
    );
    assert!(ics.contains("BEGIN:VTODO\r\n"), "Should contain VTODO");
    assert!(ics.contains("END:VTODO\r\n"), "Should close VTODO");
    assert!(ics.contains("VERSION:2.0\r\n"), "Should have version");
    assert!(
        ics.contains("PRODID:-//todos-cli//TODOS Calendar//EN\r\n"),
        "Should have PRODID"
    );
    assert!(
        ics.contains("CALSCALE:GREGORIAN\r\n"),
        "Should have CALSCALE"
    );
    assert!(
        ics.contains("DTSTART;VALUE=DATE:20260701\r\n"),
        "DTSTART should be date-only pinned to the todo day"
    );
    assert!(
        !ics.contains("DTEND:"),
        "DTEND should not be present for tasks"
    );
}

#[test]
fn test_ics_vtodo_count() {
    let items = vec![
        TodoItem {
            date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            weekday_name: "Miércoles".to_string(),
            priority: 1,
            description: "Task 1".to_string(),
        },
        TodoItem {
            date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            weekday_name: "Miércoles".to_string(),
            priority: 2,
            description: "Task 2".to_string(),
        },
    ];

    let ics = generate_ics("TODOS - 202607", &items, &[]);
    let vtodo_count = ics.matches("BEGIN:VTODO").count();
    assert_eq!(vtodo_count, 2, "Should have 2 VTODOs for 2 items");
}

#[test]
fn test_ics_empty() {
    let items = vec![];
    let ics = generate_ics("TODOS - 202607", &items, &[]);
    assert!(
        ics.contains("BEGIN:VCALENDAR"),
        "Empty calendar should still be valid"
    );
    assert!(
        ics.contains("END:VCALENDAR"),
        "Empty calendar should still be valid"
    );
    assert!(
        !ics.contains("BEGIN:VTODO"),
        "Empty calendar should have no VTODOs"
    );
}

#[test]
fn test_due_timestamp_from_csv_match() {
    let items = vec![TodoItem {
        date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        weekday_name: "Miércoles".to_string(),
        priority: 1,
        description: "Ejercicio".to_string(),
    }];

    let rules = vec![make_rule("Miércoles", 1, "09:00", 30)];
    let ics = generate_ics("TODOS - 202607", &items, &rules);

    assert!(
        ics.contains("DUE:20260701T090000\r\n"),
        "DUE should be the floating local CSV hour"
    );
    assert!(
        ics.contains("TRIGGER;RELATED=END:-PT30M\r\n"),
        "VALARM should anchor to DUE with CSV minutes"
    );
}

#[test]
fn test_due_timestamp_no_csv_match() {
    let items = vec![TodoItem {
        date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        weekday_name: "Miércoles".to_string(),
        priority: 7,
        description: "No match".to_string(),
    }];

    let rules = vec![make_rule("Miércoles", 1, "09:00", 30)];
    let ics = generate_ics("TODOS - 202607", &items, &rules);

    assert!(
        ics.contains("DUE:20260701T235959\r\n"),
        "No CSV match should default to end of day 23:59"
    );
    assert!(
        !ics.contains("BEGIN:VALARM"),
        "No match should not have VALARM"
    );
}

#[test]
fn test_line_folding_max_75_octets() {
    let long_desc = "A".repeat(95);
    let items = vec![TodoItem {
        date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        weekday_name: "Miércoles".to_string(),
        priority: 1,
        description: long_desc,
    }];

    let ics = generate_ics("TODOS - 202607", &items, &[]);
    for line in ics.lines() {
        let line = line.trim_end_matches('\r');
        if !line.is_empty() {
            assert!(
                line.len() <= 75,
                "Line exceeds 75 octets: {} (len={})",
                line,
                line.len()
            );
        }
    }
}

#[test]
fn test_content_escaping() {
    let items = vec![TodoItem {
        date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        weekday_name: "Miércoles".to_string(),
        priority: 1,
        description: "Escape \\ ; comma , and\nnewline".to_string(),
    }];

    let ics = generate_ics("TODOS - 202607", &items, &[]);
    assert!(
        ics.contains("Escape \\\\ \\; comma \\, and\\nnewline"),
        "Special chars should be escaped"
    );
}

#[test]
fn test_only_vtodo_components_present() {
    let items = vec![
        TodoItem {
            date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            weekday_name: "Miércoles".to_string(),
            priority: 1,
            description: "Task 1".to_string(),
        },
        TodoItem {
            date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            weekday_name: "Miércoles".to_string(),
            priority: 2,
            description: "Task 2".to_string(),
        },
        TodoItem {
            date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            weekday_name: "Miércoles".to_string(),
            priority: 3,
            description: "Task 3".to_string(),
        },
    ];

    let ics = generate_ics("TODOS - 202607", &items, &[]);
    assert!(
        !ics.contains("VEVENT"),
        "Output should contain no VEVENT components"
    );
    assert_eq!(
        ics.matches("BEGIN:VTODO").count(),
        3,
        "Exactly 3 VTODO components for 3 todos"
    );
}

#[test]
fn test_uid_format() {
    let items = vec![TodoItem {
        date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        weekday_name: "Miércoles".to_string(),
        priority: 1,
        description: "Task".to_string(),
    }];

    let ics = generate_ics("TODOS - 202607", &items, &[]);
    assert!(ics.contains("UID:"), "Should have UID field");
    assert!(ics.contains("@todos-cli"), "UID should end with @todos-cli");
}

#[test]
fn test_dtstamp_format() {
    let items = vec![TodoItem {
        date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        weekday_name: "Miércoles".to_string(),
        priority: 1,
        description: "Task".to_string(),
    }];

    let ics = generate_ics("TODOS - 202607", &items, &[]);
    assert!(ics.contains("DTSTAMP:"), "Should have DTSTAMP field");
    assert!(
        ics.contains("Z\r\n"),
        "DTSTAMP should be in UTC with Z suffix"
    );
}

#[test]
fn test_crlf_line_endings() {
    let items = vec![TodoItem {
        date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        weekday_name: "Miércoles".to_string(),
        priority: 1,
        description: "Task".to_string(),
    }];

    let ics = generate_ics("TODOS - 202607", &items, &[]);
    assert!(ics.contains("\r\n"), "Should use CRLF line endings");
}

#[test]
fn test_generate_uid_uniqueness() {
    let date = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();
    let uid1 = generate_uid(date, "Task A", 1);
    let uid2 = generate_uid(date, "Task B", 1);
    let uid3 = generate_uid(date, "Task A", 2);
    assert_ne!(
        uid1, uid2,
        "Different summaries should produce different UIDs"
    );
    assert_ne!(
        uid1, uid3,
        "Different priorities should produce different UIDs"
    );
}

#[test]
fn test_generate_uid_distinct_for_identical_todos() {
    let date = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();
    let uid1 = generate_uid(date, "Task", 1);
    let uid2 = generate_uid(date, "Task", 1);
    assert_eq!(
        uid1, uid2,
        "generate_uid alone is deterministic and does not distinguish duplicates"
    );
    let uid_duplicate = generate_uid_with_ordinal(date, "Task", 1, 2);
    assert_ne!(
        uid1, uid_duplicate,
        "Ordinal suffix must disambiguate identical todos"
    );
}

#[test]
fn test_generate_ics_uids_stable_and_distinct() {
    let items = vec![
        TodoItem {
            date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            weekday_name: "Miércoles".to_string(),
            priority: 1,
            description: "Duplicated task".to_string(),
        },
        TodoItem {
            date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            weekday_name: "Miércoles".to_string(),
            priority: 1,
            description: "Duplicated task".to_string(),
        },
    ];

    let ics1 = generate_ics("TODOS - 202607", &items, &[]);
    let ics2 = generate_ics("TODOS - 202607", &items, &[]);

    let uids1: Vec<&str> = ics1.lines().filter(|l| l.starts_with("UID:")).collect();
    let uids2: Vec<&str> = ics2.lines().filter(|l| l.starts_with("UID:")).collect();

    assert_eq!(uids1.len(), 2, "One UID per todo");
    assert_eq!(
        uids1, uids2,
        "UID set should be stable across regenerations"
    );
    assert_ne!(uids1[0], uids1[1], "Identical todos need distinct UIDs");
}
