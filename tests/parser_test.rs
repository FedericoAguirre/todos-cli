use chrono::NaiveDate;
use todos_cli::parser::{CsvParser, MdParser};

#[test]
fn test_md_parser_extracts_single_todo() {
    let md = "\
## 20260701 - Miércoles
- [ ] 1. Ejercicio
";
    let items = MdParser::parse(md);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].date, NaiveDate::from_ymd_opt(2026, 7, 1).unwrap());
    assert_eq!(items[0].weekday_name, "Miércoles");
    assert_eq!(items[0].priority, 1);
    assert_eq!(items[0].description, "Ejercicio");
}

#[test]
fn test_md_parser_extracts_multiple_days() {
    let md = "\
## 20260701 - Miércoles
- [ ] 1. Task 1
- [ ] 2. Task 2

## 20260702 - Jueves
- [ ] 1. Task 3
";
    let items = MdParser::parse(md);
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].description, "Task 1");
    assert_eq!(items[1].description, "Task 2");
    assert_eq!(items[2].description, "Task 3");
    assert_eq!(items[2].date, NaiveDate::from_ymd_opt(2026, 7, 2).unwrap());
    assert_eq!(items[2].weekday_name, "Jueves");
}

#[test]
fn test_md_parser_removes_wiki_links() {
    let md = "\
## 20260701 - Miércoles
- [ ] 1. [[Ejercicio]]
- [ ] 2. Leer [[Rust]] book
";
    let items = MdParser::parse(md);
    assert_eq!(items[0].description, "Ejercicio");
    assert_eq!(items[1].description, "Leer Rust book");
}

#[test]
fn test_md_parser_handles_empty() {
    let items = MdParser::parse("");
    assert!(items.is_empty());
}

#[test]
fn test_csv_parser_parses_rules() {
    let csv = "\
weekday,priority,hour,minutes
Lunes,1,9:00,30
Lunes,2,16:00,30
Martes,1,9:00,30
";
    let rules = CsvParser::parse(csv);
    assert_eq!(rules.len(), 3);

    let lunes1 = &rules[0];
    assert_eq!(lunes1.weekday, "Lunes");
    assert_eq!(lunes1.priority, 1);
    assert_eq!(lunes1.hour.format("%H:%M").to_string(), "09:00");
    assert_eq!(lunes1.alarm_minutes, 30);
}

#[test]
fn test_csv_parser_skips_header() {
    let csv = "\
weekday,priority,hour,minutes
Lunes,1,9:00,30
";
    let rules = CsvParser::parse(csv);
    assert_eq!(rules.len(), 1);
}
