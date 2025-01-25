use chrono::{Date, Datelike, NaiveDate, Timelike, Utc};
use egui::*;

pub struct DateTimehandler {
    selected_date : NaiveDate,
    selected_hour : u32,
    selected_minute : u32,
}

impl DateTimehandler {
    pub fn new() -> DateTimehandler {

        let now = Utc::now();

        DateTimehandler {
            selected_date : now.date_naive(),
            selected_hour : now.hour(),
            selected_minute : now.minute(),
        }
        
    }
}