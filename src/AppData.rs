use eframe::egui;
use egui::*;

use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::io::Write;

pub struct TodoApp {
    current_page : Page,
    input_text : String,
    todo_list : Vec<String>,
}

pub enum CustomeError {

}

pub enum Page {
    HomePage,
    AddPage,
    ViewPage,
}

//OVERHAUL ERROR SYSTEM

impl TodoApp {
    pub fn new() -> TodoApp {
        TodoApp {
            current_page : Page::HomePage,
            input_text : String::new(),
            todo_list : Vec::new(),
        }
    }

    fn load_databse(&mut self) -> Result<(), String> {
        
        let file = OpenOptions::new()
                                 .read(true)
                                 .write(false)
                                 .create(true)
                                 .open("database_todo.txt")
                                 .map_err(|_| format!("Error Occurred While Opening File"))
                                 .unwrap();
        
        let reader = io::BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|_| format!("Error Occurred While Reading Line")).unwrap();
            lines.push(line);
        }

        self.todo_list = lines;

        println!("Successfully Loaded Database");

        Ok(())
    }

    fn save_database(&mut self) -> Result<(), String> {
        let file = OpenOptions::new()
                                         .read(false)
                                         .write(true)
                                         .open("database_todo.txt")
                                         .map_err(|_| format!("Error Occurred While Opening File"))
                                         .unwrap();
        
        let mut writer = io::BufWriter::new(file);
        for line in &self.todo_list {
            let _ = writeln!(writer, "{}", line).map_err(|_| format!("Error Occurred While Writing To Line"));
        }

        Ok(())
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // ADD LOAD DATABASE HERE OR SOMEWHERE ELSE AFTER OVERHAULING ERROR SYSTEM
        match self.current_page {
            Page::HomePage => {
                egui::TopBottomPanel::top("home_top").show(ctx, |ui| {
                    ui.label("TodoList")
                });
        
                egui::CentralPanel::default().show(ctx, |ui| {
        
                    if ui.button("Add Todo").clicked() {
                        self.current_page = Page::AddPage;
                    }
        
                    if ui.button("View Todos").clicked() {
                        self.current_page = Page::ViewPage;
                    }
        
                    if ui.button("Exit").clicked() {
                        std::process::exit(0)
                    }
                });
            },
            Page::AddPage => {
                egui::TopBottomPanel::top("Add Page Top").show(ctx, |ui| {
                    ui.label("Add Page");
                });
        
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Enter Todo: ");
                    ui.add(egui::TextEdit::singleline(&mut self.input_text));   

                    if ui.button("Save Todo").clicked() {
                        self.todo_list.push(self.input_text.to_string());
                        

                        //YOU WERE ABOUT TO TADD THE SAVE DATABASE FUNCTION HERE BUT NOW YOU NEED TO OVERHAUL THE ERROR SYSTEM
                        //FIX ERROR SYSTEM FOR THE IMPLEMENTATION BLOCK ABOVE THIS ONE THEN YOU CAN PROCEED FURTHER 
                        //SO FAR YOU HAVE FIXED THE PROGRAM TO SAVE THE TODOS IN THE STRUCT VECTOR BUT NOT IN A FILE
                        //AFTER USING THE SAVE DATABASE FUNCTION YOUR TODOS WILL BE SAVED TO A FILE
                    }

                    if ui.button("Back").clicked() {
                        self.current_page = Page::HomePage;
                    }
                });
            },
            Page::ViewPage => {
                egui::TopBottomPanel::top("Add Page Top").show(ctx, |ui| {
                    ui.label("View Page")
                });
        
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(100.0)
                        .show(ui, |ui| {
                            for (i, todo) in self.todo_list.iter().enumerate() {
                                ui.label(todo);
                            }
                        }
                    );

                    if ui.button("Back").clicked() {
                        self.current_page = Page::HomePage;
                    }
                });
            },
        }
    }
}