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
