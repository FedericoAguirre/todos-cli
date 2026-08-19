use crate::parser::{DueTimeRule, TodoItem};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct IcsCalendar {
    pub name: String,
    pub todos: Vec<IcsTodo>,
}

pub struct IcsTodo {
    pub uid: String,
    pub dtstamp: DateTime<Utc>,
    pub summary: String,
    pub date: NaiveDate,
    pub priority: u8,
    pub due: NaiveDateTime,
    pub status: String,
    pub alarm_minutes: Option<u16>,
}

impl IcsCalendar {
    pub fn new(name: &str) -> Self {
        IcsCalendar {
            name: name.to_string(),
            todos: Vec::new(),
        }
    }

    pub fn add_todo(&mut self, todo: IcsTodo) -> &mut Self {
        self.todos.push(todo);
        self
    }

    pub fn format_ics(&self) -> String {
        let mut output = String::new();
        output.push_str("BEGIN:VCALENDAR\r\n");
        output.push_str("VERSION:2.0\r\n");
        output.push_str("PRODID:-//todos-cli//TODOS Calendar//EN\r\n");
        output.push_str("CALSCALE:GREGORIAN\r\n");
        output.push_str(&format!("X-WR-CALNAME:{}\r\n", self.name));

        for todo in &self.todos {
            output.push_str("BEGIN:VTODO\r\n");
            output.push_str(&format!("UID:{}\r\n", todo.uid));
            output.push_str(&format!(
                "DTSTAMP:{}\r\n",
                todo.dtstamp.format("%Y%m%dT%H%M%SZ")
            ));
            output.push_str(&format!(
                "DTSTART;VALUE=DATE:{}\r\n",
                todo.date.format("%Y%m%d")
            ));
            output.push_str(&format!("SUMMARY:{}\r\n", escape_ics(&todo.summary)));
            output.push_str(&format!("PRIORITY:{}\r\n", todo.priority));
            output.push_str(&format!("DUE:{}\r\n", todo.due.format("%Y%m%dT%H%M%S")));
            output.push_str(&format!("STATUS:{}\r\n", todo.status));
            if let Some(mins) = todo.alarm_minutes {
                output.push_str("BEGIN:VALARM\r\n");
                output.push_str(&format!("TRIGGER;RELATED=END:-PT{}M\r\n", mins));
                output.push_str("ACTION:DISPLAY\r\n");
                output.push_str("DESCRIPTION:Reminder\r\n");
                output.push_str("END:VALARM\r\n");
            }
            output.push_str("END:VTODO\r\n");
        }

        output.push_str("END:VCALENDAR\r\n");
        fold_lines(&output)
    }
}

fn escape_ics(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace('\n', "\\n")
        .replace('\r', "")
}

fn fold_lines(s: &str) -> String {
    let mut result = String::new();
    for line in s.lines() {
        let line = line.trim_end_matches('\r');
        if line.len() <= 75 {
            result.push_str(line);
            result.push_str("\r\n");
        } else {
            let mut pos = 0;
            while pos < line.len() {
                let end = (pos + 75).min(line.len());
                if pos == 0 {
                    result.push_str(&line[pos..end]);
                    result.push_str("\r\n");
                } else {
                    result.push(' ');
                    result.push_str(&line[pos..end]);
                    result.push_str("\r\n");
                }
                pos = end;
            }
        }
    }
    result
}

pub fn generate_uid(date: NaiveDate, summary: &str, priority: u8) -> String {
    generate_uid_with_ordinal(date, summary, priority, 0)
}

pub fn generate_uid_with_ordinal(
    date: NaiveDate,
    summary: &str,
    priority: u8,
    ordinal: u32,
) -> String {
    let mut hasher = DefaultHasher::new();
    date.hash(&mut hasher);
    summary.hash(&mut hasher);
    priority.hash(&mut hasher);
    ordinal.hash(&mut hasher);
    format!("{:x}@todos-cli", hasher.finish())
}

fn default_due_time() -> NaiveTime {
    NaiveTime::from_hms_opt(23, 59, 59).unwrap()
}

pub fn generate_ics(name: &str, items: &[TodoItem], rules: &[DueTimeRule]) -> String {
    let dtstamp = Utc::now();
    let mut calendar = IcsCalendar::new(name);

    let mut uid_counts: HashMap<(NaiveDate, String, u8), u32> = HashMap::new();

    for item in items {
        let uid_base = generate_uid(item.date, &item.description, item.priority);
        let key = (item.date, item.description.clone(), item.priority);
        let ordinal = uid_counts.entry(key).or_insert(0);
        *ordinal += 1;

        let uid = if *ordinal > 1 {
            generate_uid_with_ordinal(item.date, &item.description, item.priority, *ordinal)
        } else {
            uid_base
        };

        let (due, alarm_minutes) =
            if let Some(rule) = DueTimeRule::lookup(rules, &item.weekday_name, item.priority) {
                (item.date.and_time(rule.hour), Some(rule.alarm_minutes))
            } else {
                (item.date.and_time(default_due_time()), None)
            };

        let todo = IcsTodo {
            uid,
            dtstamp,
            summary: format!("[P{}] {}", item.priority, item.description),
            date: item.date,
            priority: item.priority,
            due,
            status: "NEEDS-ACTION".to_string(),
            alarm_minutes,
        };
        calendar.add_todo(todo);
    }

    calendar.format_ics()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{CsvParser, MdParser};

    fn sample_md() -> &'static str {
        "# TODOS 202608\n\n---\n\n## 20260801 - Lunes\n\n- [ ] 1. Ejercicio\n- [ ] 2. Trabajar en RSVR, 2 horas\n- [ ] 3. Trabajar en Ematrix, 2 horas\n"
    }

    fn sample_csv() -> &'static str {
        "weekday,priority,hour,minutes\nLunes,1,9:00,30\nLunes,2,16:00,30\nLunes,3,18:00,10\n"
    }

    #[test]
    fn test_ics_uses_vtodo_instead_of_vevent() {
        let md = sample_md();
        let rules = CsvParser::parse(sample_csv());
        let items = MdParser::parse(md);
        let ics = generate_ics("TODOS - 202608", &items, &rules);

        assert!(ics.contains("VTODO"), "Should contain VTODO");
        assert!(!ics.contains("VEVENT"), "Should not contain VEVENT");
    }

    #[test]
    fn test_ics_uses_due_instead_of_dtend() {
        let md = sample_md();
        let rules = CsvParser::parse(sample_csv());
        let items = MdParser::parse(md);
        let ics = generate_ics("TODOS - 202608", &items, &rules);

        assert!(ics.contains("DUE:"), "Should contain DUE:");
        assert!(!ics.contains("DTEND:"), "Should not contain DTEND:");
    }

    #[test]
    fn test_ics_has_needs_action_status() {
        let md = sample_md();
        let rules = CsvParser::parse(sample_csv());
        let items = MdParser::parse(md);
        let ics = generate_ics("TODOS - 202608", &items, &rules);

        assert!(
            ics.contains("STATUS:NEEDS-ACTION"),
            "Should contain STATUS:NEEDS-ACTION"
        );
    }

    #[test]
    fn test_ics_summary_includes_priority() {
        let md = sample_md();
        let rules = CsvParser::parse(sample_csv());
        let items = MdParser::parse(md);
        let ics = generate_ics("TODOS - 202608", &items, &rules);

        assert!(ics.contains("SUMMARY:[P1]"));
        assert!(ics.contains("SUMMARY:[P2]"));
        assert!(ics.contains("SUMMARY:[P3]"));
    }

    #[test]
    fn test_ics_date_pinned_and_due_on_day() {
        let md = sample_md();
        let rules = CsvParser::parse("weekday,priority,hour,minutes\nLunes,1,9:00,30\n");
        let items = MdParser::parse(md);

        let ics = generate_ics("TODOS - 202608", &items, &rules);

        assert!(
            ics.contains("DTSTART;VALUE=DATE:20260801\r\n"),
            "DTSTART should be date-only pinned to 20260801"
        );
        assert!(
            ics.contains("DUE:20260801T090000\r\n"),
            "DUE should be the floating local rule hour on the todo's day"
        );
    }

    #[test]
    fn test_ics_is_valid_calendar() {
        let md = sample_md();
        let rules = CsvParser::parse(sample_csv());
        let items = MdParser::parse(md);
        let ics = generate_ics("TODOS - 202608", &items, &rules);

        assert!(ics.starts_with("BEGIN:VCALENDAR\r\n"));
        assert!(ics.contains("VERSION:2.0\r\n"));
        assert!(ics.ends_with("END:VCALENDAR\r\n"));
    }

    #[test]
    fn test_ics_valarm_present_for_rules() {
        let md = sample_md();
        let rules = CsvParser::parse(sample_csv());
        let items = MdParser::parse(md);
        let ics = generate_ics("TODOS - 202608", &items, &rules);

        assert!(ics.contains("BEGIN:VALARM\r\n"));
        assert!(ics.contains("TRIGGER;RELATED=END:-PT30M\r\n"));
    }
}
