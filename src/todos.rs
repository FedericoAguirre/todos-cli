use chrono::NaiveDate;

pub struct Todos {
    // Add fields as needed, e.g. year, month, days, etc.
    pub year: i32,
    pub month: u32,
    pub path: Option<String>,
}

impl Todos {
    pub fn new(year: i32, month: u32, path: Option<String>) -> Self {
        Self { year, month, path }
    }
    pub fn get_days(&self) -> Vec<chrono::NaiveDate> {
        let days_in_month = match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_days_31_days_month() {
        let todos = Todos::new(2024, 1, None); // January
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
        let todos = Todos::new(2024, 4, None); // April
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
        let todos = Todos::new(2024, 2, None); // Leap year
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
        let todos = Todos::new(2023, 2, None); // Non-leap year
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
        let todos = Todos::new(1900, 2, None); // 1900 is not a leap year
        let days = todos.get_days();
        assert_eq!(days.len(), 28);
        assert_eq!(
            days.last(),
            Some(&NaiveDate::from_ymd_opt(1900, 2, 28).unwrap())
        );
    }

    #[test]
    fn test_get_days_february_century_leap() {
        let todos = Todos::new(2000, 2, None); // 2000 is a leap year
        let days = todos.get_days();
        assert_eq!(days.len(), 29);
        assert_eq!(
            days.last(),
            Some(&NaiveDate::from_ymd_opt(2000, 2, 29).unwrap())
        );
    }

    #[test]
    fn test_get_days_invalid_month() {
        let todos = Todos::new(2024, 13, None); // Invalid month
        let days = todos.get_days();
        assert!(days.is_empty());
    }
}
