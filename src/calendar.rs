use crate::parser::{DueTimeRule, TodoItem};
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct IcsCalendar {
    pub name: String,
    pub events: Vec<IcsEvent>,
}

pub struct IcsEvent {
    pub uid: String,
    pub dtstamp: DateTime<Utc>,
    pub summary: String,
    pub dtstart: DateTime<Utc>,
    pub dtend: DateTime<Utc>,
    pub alarm_minutes: Option<u16>,
}

impl IcsCalendar {
    pub fn new(name: &str) -> Self {
        IcsCalendar {
            name: name.to_string(),
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: IcsEvent) -> &mut Self {
        self.events.push(event);
        self
    }

    pub fn format_ics(&self) -> String {
        let mut output = String::new();
        output.push_str("BEGIN:VCALENDAR\r\n");
        output.push_str("VERSION:2.0\r\n");
        output.push_str("PRODID:-//todos-cli//TODOS Calendar//EN\r\n");
        output.push_str("CALSCALE:GREGORIAN\r\n");
        output.push_str(&format!("X-WR-CALNAME:{}\r\n", self.name));

        for event in &self.events {
            output.push_str("BEGIN:VEVENT\r\n");
            output.push_str(&format!("UID:{}\r\n", event.uid));
            output.push_str(&format!(
                "DTSTAMP:{}\r\n",
                event.dtstamp.format("%Y%m%dT%H%M%SZ")
            ));
            output.push_str(&format!(
                "DTSTART:{}\r\n",
                event.dtstart.format("%Y%m%dT%H%M%SZ")
            ));
            output.push_str(&format!(
                "DTEND:{}\r\n",
                event.dtend.format("%Y%m%dT%H%M%SZ")
            ));
            output.push_str(&format!("SUMMARY:{}\r\n", escape_ics(&event.summary)));
            if let Some(mins) = event.alarm_minutes {
                output.push_str("BEGIN:VALARM\r\n");
                output.push_str(&format!("TRIGGER:-PT{}M\r\n", mins));
                output.push_str("ACTION:DISPLAY\r\n");
                output.push_str("DESCRIPTION:Reminder\r\n");
                output.push_str("END:VALARM\r\n");
            }
            output.push_str("END:VEVENT\r\n");
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
    let mut hasher = DefaultHasher::new();
    date.hash(&mut hasher);
    summary.hash(&mut hasher);
    priority.hash(&mut hasher);
    format!("{:x}@todos-cli", hasher.finish())
}

fn default_start_time() -> NaiveTime {
    NaiveTime::from_hms_opt(9, 0, 0).unwrap()
}

pub fn generate_ics(name: &str, items: &[TodoItem], rules: &[DueTimeRule]) -> String {
    let dtstamp = Utc::now();
    let mut calendar = IcsCalendar::new(name);

    let local_offset = *Local::now().offset();

    for item in items {
        let uid = generate_uid(item.date, &item.description, item.priority);

        let (start_local, alarm_minutes) =
            if let Some(rule) = DueTimeRule::lookup(rules, &item.weekday_name, item.priority) {
                (item.date.and_time(rule.hour), Some(rule.alarm_minutes))
            } else {
                (item.date.and_time(default_start_time()), None)
            };

        let end_local = start_local + Duration::hours(1);

        let dtstart = local_offset
            .from_local_datetime(&start_local)
            .earliest()
            .unwrap()
            .to_utc();
        let dtend = local_offset
            .from_local_datetime(&end_local)
            .earliest()
            .unwrap()
            .to_utc();

        let event = IcsEvent {
            uid,
            dtstamp,
            summary: format!("[P{}] {}", item.priority, item.description),
            dtstart,
            dtend,
            alarm_minutes,
        };
        calendar.add_event(event);
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
    fn test_ics_uses_vevent_instead_of_vtodo() {
        let md = sample_md();
        let rules = CsvParser::parse(sample_csv());
        let items = MdParser::parse(md);
        let ics = generate_ics("TODOS - 202608", &items, &rules);

        assert!(!ics.contains("VTODO"), "Should not contain VTODO");
        assert!(ics.contains("VEVENT"), "Should contain VEVENT");
    }

    #[test]
    fn test_ics_uses_dtend_instead_of_due() {
        let md = sample_md();
        let rules = CsvParser::parse(sample_csv());
        let items = MdParser::parse(md);
        let ics = generate_ics("TODOS - 202608", &items, &rules);

        assert!(!ics.contains("DUE:"), "Should not contain DUE:");
        assert!(ics.contains("DTEND:"), "Should contain DTEND:");
    }

    #[test]
    fn test_ics_no_status_field() {
        let md = sample_md();
        let rules = CsvParser::parse(sample_csv());
        let items = MdParser::parse(md);
        let ics = generate_ics("TODOS - 202608", &items, &rules);

        assert!(!ics.contains("STATUS:"), "Should not contain STATUS:");
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
    fn test_ics_event_duration_one_hour() {
        let md = sample_md();
        let rules = CsvParser::parse("weekday,priority,hour,minutes\nLunes,1,9:00,30\n");
        let items = MdParser::parse(md);

        let ics = generate_ics("TODOS - 202608", &items, &rules);

        let lines: Vec<&str> = ics.lines().collect();
        let dtstart_idx = lines
            .iter()
            .position(|l| l.starts_with("DTSTART:20260801T"));
        let dtend_idx = lines.iter().position(|l| l.starts_with("DTEND:20260801T"));

        assert!(dtstart_idx.is_some(), "DTSTART for 20260801 not found");
        assert!(dtend_idx.is_some(), "DTEND for 20260801 not found");

        let dtstart_line = lines[dtstart_idx.unwrap()];
        let dtend_line = lines[dtend_idx.unwrap()];

        let start = &dtstart_line["DTSTART:".len()..];
        let end = &dtend_line["DTEND:".len()..];

        assert!(
            end > start,
            "DTEND ({}) should be after DTSTART ({})",
            end,
            start
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
        assert!(ics.contains("TRIGGER:-PT30M\r\n"));
    }
}
