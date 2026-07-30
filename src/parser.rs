use chrono::NaiveDate;
use chrono::NaiveTime;

#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub date: NaiveDate,
    pub weekday_name: String,
    pub priority: u8,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DueTimeRule {
    pub weekday: String,
    pub priority: u8,
    pub hour: NaiveTime,
    pub alarm_minutes: u16,
}

impl DueTimeRule {
    pub fn lookup<'a>(
        rules: &'a [DueTimeRule],
        weekday: &str,
        priority: u8,
    ) -> Option<&'a DueTimeRule> {
        rules
            .iter()
            .find(|r| r.weekday == weekday && r.priority == priority)
    }
}

pub struct MdParser;

impl MdParser {
    pub fn parse(content: &str) -> Vec<TodoItem> {
        let mut items = Vec::new();
        let mut current_date: Option<NaiveDate> = None;
        let mut current_weekday: Option<String> = None;

        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("## ") {
                let parts: Vec<&str> = rest.splitn(2, " - ").collect();
                if parts.len() == 2
                    && let Ok(date) = NaiveDate::parse_from_str(parts[0], "%Y%m%d")
                {
                    current_date = Some(date);
                    current_weekday = Some(parts[1].to_string());
                }
                continue;
            }

            if let Some(rest) = line.strip_prefix("- [ ] ")
                && let (Some(date), Some(ref weekday)) = (current_date, current_weekday.as_ref())
            {
                let todo_parts: Vec<&str> = rest.splitn(2, ". ").collect();
                if todo_parts.len() == 2 {
                    let priority: u8 = todo_parts[0].parse().unwrap_or(6).clamp(1, 6);
                    let description = todo_parts[1].trim().to_string();
                    let description = description.replace("[[", "").replace("]]", "");
                    items.push(TodoItem {
                        date,
                        weekday_name: weekday.to_string(),
                        priority,
                        description,
                    });
                }
            }
        }

        items
    }
}

pub struct CsvParser;

impl CsvParser {
    pub fn parse(content: &str) -> Vec<DueTimeRule> {
        let mut rules = Vec::new();

        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 4 {
                continue;
            }

            let weekday = parts[0].trim().to_string();
            let priority: u8 = match parts[1].trim().parse() {
                Ok(p) if (1..=6).contains(&p) => p,
                _ => continue,
            };
            let hour = match NaiveTime::parse_from_str(parts[2].trim(), "%H:%M") {
                Ok(h) => h,
                _ => continue,
            };
            let alarm_minutes: u16 = parts[3].trim().parse().unwrap_or_default();

            rules.push(DueTimeRule {
                weekday,
                priority,
                hour,
                alarm_minutes,
            });
        }

        rules
    }
}
