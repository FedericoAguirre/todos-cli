use chrono::{Local, NaiveDate, NaiveTime, TimeZone};
use todos_cli::calendar::{generate_ics, generate_uid};
use todos_cli::parser::{DueTimeRule, TodoItem};

fn make_rule(weekday: &str, priority: u8, hour: &str, alarm_minutes: u16) -> DueTimeRule {
    DueTimeRule {
        weekday: weekday.to_string(),
        priority,
        hour: NaiveTime::parse_from_str(hour, "%H:%M").unwrap(),
        alarm_minutes,
    }
}

fn local_utc(date: NaiveDate, hour: u32, min: u32, sec: u32) -> String {
    let offset = *Local::now().offset();
    let naive = date.and_hms_opt(hour, min, sec).unwrap();
    let utc = offset
        .from_local_datetime(&naive)
        .earliest()
        .unwrap()
        .to_utc();
    utc.format("%Y%m%dT%H%M%SZ").to_string()
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
    assert!(ics.contains("BEGIN:VEVENT\r\n"), "Should contain VEVENT");
    assert!(ics.contains("END:VEVENT\r\n"), "Should close VEVENT");
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
        !ics.contains("DTSTART;VALUE=DATE"),
        "DTSTART should not use VALUE=DATE parameter"
    );
    assert!(
        ics.contains("DTSTART:20260701T"),
        "DTSTART should have date-time format"
    );
    assert!(ics.contains("Z\r\n"), "Times should be UTC with Z suffix");
}

#[test]
fn test_ics_vevent_count() {
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
    let vevent_count = ics.matches("BEGIN:VEVENT").count();
    assert_eq!(vevent_count, 2, "Should have 2 VEVENTs for 2 items");
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
        !ics.contains("BEGIN:VEVENT"),
        "Empty calendar should have no VEVENTs"
    );
}

#[test]
fn test_event_timestamp_from_csv_match() {
    let items = vec![TodoItem {
        date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        weekday_name: "Miércoles".to_string(),
        priority: 1,
        description: "Ejercicio".to_string(),
    }];

    let rules = vec![make_rule("Miércoles", 1, "09:00", 30)];
    let ics = generate_ics("TODOS - 202607", &items, &rules);

    let expected_dtstart = local_utc(NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(), 9, 0, 0);
    let expected_dtend = local_utc(NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(), 10, 0, 0);
    assert!(
        ics.contains(&format!("DTSTART:{}\r\n", expected_dtstart)),
        "DTSTART should match CSV hour converted to UTC"
    );
    assert!(
        ics.contains(&format!("DTEND:{}\r\n", expected_dtend)),
        "DTEND should be 1 hour after DTSTART"
    );
    assert!(
        ics.contains("TRIGGER:-PT30M\r\n"),
        "VALARM should use CSV minutes"
    );
}

#[test]
fn test_event_timestamp_no_csv_match() {
    let items = vec![TodoItem {
        date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        weekday_name: "Miércoles".to_string(),
        priority: 7,
        description: "No match".to_string(),
    }];

    let rules = vec![make_rule("Miércoles", 1, "09:00", 30)];
    let ics = generate_ics("TODOS - 202607", &items, &rules);

    let expected_dtstart = local_utc(NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(), 9, 0, 0);
    let expected_dtend = local_utc(NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(), 10, 0, 0);
    assert!(
        ics.contains(&format!("DTSTART:{}\r\n", expected_dtstart)),
        "No CSV match should default to 09:00 DTSTART"
    );
    assert!(
        ics.contains(&format!("DTEND:{}\r\n", expected_dtend)),
        "No CSV match should default to 10:00 DTEND"
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
