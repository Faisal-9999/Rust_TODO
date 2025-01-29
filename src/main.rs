use eframe::*;

mod app_data;
mod custom_err;
mod date_time_handler;
mod to_do;
mod page_data;

use crate::app_data::TodoApp;

fn main() -> Result<(), eframe::Error>{
    eframe::run_native("TodoApp",eframe::NativeOptions::default(), Box::new(|_| Ok(Box::new(TodoApp::new()))))
}