use eframe::*;

mod AppData;

use crate::AppData::TodoApp;


fn main() -> Result<(), eframe::Error>{
    eframe::run_native("TodoApp",eframe::NativeOptions::default(), Box::new(|_| Ok(Box::new(TodoApp::new()))))
}
