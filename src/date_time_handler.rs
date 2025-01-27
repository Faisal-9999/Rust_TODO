use chrono::{Date, Datelike, NaiveDate, Timelike, Utc};
use egui::*;

use crate::to_do::Todo;
pub struct DateTimeHandler;

impl DateTimeHandler {
    pub fn show(ui : &mut Ui, todo : &mut Todo) {

        ui.label("Select Date: ");
        ui.horizontal(|ui| {
            if ui.button("Previous").clicked() {
                todo.selected_date = (todo.selected_date).pred_opt().unwrap_or(todo.selected_date);
            }

            ui.label(todo.selected_date.to_string());

            if ui.button("Next").clicked() {
                todo.selected_date = (todo.selected_date).succ_opt().unwrap_or(todo.selected_date);
            }
        });

        ui.label("Select Time: ");
        ui.horizontal(|ui| {
            ui.label("Hour: ");
            if ui.add(DragValue::new(&mut todo.selected_hour).range(0..=23)).changed() {
                todo.selected_hour = todo.selected_hour % 24;
            }

            ui.label("Minute: ");
            if ui.add(DragValue::new(&mut todo.selected_minute).range(0..=59)).changed() {
                todo.selected_minute = todo.selected_minute % 60;
            }
        });
    }
}