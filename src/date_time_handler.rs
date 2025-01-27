use chrono::{Date, Datelike, NaiveDate, Timelike, Utc};
use egui::*;

pub struct DateTimehandler;

//ADD A SYSTEM TO CHECK NOTIFICATIONS IMPLEMENT IT IN THE
impl DateTimehandler {

    fn show(&mut self, ui : &mut Ui, mut selected_date : NaiveDate, mut selected_hour : u32, mut selected_minute : u32) -> Option<(NaiveDate, u32, u32)> {
        let mut is_confirmed = false;

        ui.label("Select Date: ");
        ui.horizontal(|ui| {
            if ui.button("Previous").clicked() {
                selected_date = selected_date.pred_opt().unwrap_or(selected_date);
            }

            ui.label(selected_date.to_string());

            if ui.button("Next").clicked() {
                selected_date = selected_date.succ_opt().unwrap_or(selected_date);
            }
        });

        ui.label("Select Time: ");
        ui.horizontal(|ui| {
            ui.label("Hour: ");
            if ui.add(DragValue::new(&mut selected_hour).range(0..=23)).changed() {
                selected_hour = selected_hour % 24;
            }

            ui.label("Minute: ");
            if ui.add(DragValue::new(&mut selected_minute).range(0..=59)).changed() {
                selected_minute = selected_minute % 60;
            }
        });

        if ui.button("Set").clicked() {
            is_confirmed = true;
        }

        if is_confirmed {
            Some((
                selected_date,
                selected_hour,
                selected_minute,
            ))
        }
        else {
            None
        }
    }
}