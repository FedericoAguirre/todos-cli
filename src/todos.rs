use std::path;

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
}
