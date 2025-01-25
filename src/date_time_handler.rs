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

    fn show(&mut self, ui : &mut Ui) -> Option<(NaiveDate, u32, u32)> {
        let mut is_confirmed = false;

        ui.label("Select Date: ");
        ui.horizontal(|ui| {
            if ui.button("Previous").clicked() {
                self.selected_date = self.selected_date.pred_opt().unwrap_or(self.selected_date);
            }

            ui.label(self.selected_date.to_string());

            if ui.button("Next").clicked() {
                self.selected_date = self.selected_date.succ_opt().unwrap_or(self.selected_date);
            }
        });

        ui.label("Select Time: ");
        ui.horizontal(|ui| {
            ui.label("Hour: ")
            if ui.add(DragValue::new(&mut self.selected_hour).range(0..=23)).changed() {
                self.selected_hour = self.selected_hour % 24;
            }

            ui.label("Minute: ");
            if ui.add(DragValue::new(&mut self.selected_minute).range(0..=59)).changed() {
                self.selected_minute = self.selected_minute % 60;
            }
        });

        if ui.button("Set").clicked() {
            is_confirmed = true;
        }

        if is_confirmed {
            Some((
                self.selected_date,
                self.selected_hour,
                self.selected_minute,
            ))
        }
        else {
            None
        }
    }
}