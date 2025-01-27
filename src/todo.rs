use chrono::{Date, Datelike, NaiveDate, Timelike, Utc};
use egui::*;


pub struct Todo {
    pub text : String,
    pub selected_date : NaiveDate,
    pub selected_hour : u32,
    pub selected_minute : u32,
}

impl ToString for Todo {
    fn to_string(&self) -> String {
        format!("{},{},{},{}", self.text.clone(), self.selected_date.to_string(), self.selected_hour.to_string(), self.selected_minute.to_string()).clone()
    }
}