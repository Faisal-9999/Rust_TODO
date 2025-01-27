use eframe::egui;
use chrono::{Date, Datelike, NaiveDate, Timelike, Utc};

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::io::Write;
use std::path::Path;

use crate::custom_err::CustomError;
use crate::todo::Todo;

pub struct TodoApp {
    current_page : Page,
    input_text : String,
    todo_list : Vec<Todo>,
    error_type : Option<CustomError>,
    error_message : String,
    error_occurred : bool,
    error_show : bool,
}

//MAKE CHANGES ACCOUNTING FOR THE TODO STRUCT
//FIX ALL ERRORS AND INITIALIZE WHERE ITS SUPPOSED TO BE
//also add a check notifications function to check notifications when the app is on



enum Page {
    HomePage,
    AddPage,
    ViewPage,
}

//OVERHAUL ERROR SYSTEM

impl TodoApp {

    pub fn new() -> TodoApp {

        TodoApp::create_database();

        TodoApp {
            current_page : Page::HomePage,
            input_text : String::new(),
            todo_list : Vec::new(),
            error_message : String::new(),
            error_type : None,
            error_occurred : false,
            error_show : false,
        }
    }

    fn handle_error(&mut self, result : Result<(), CustomError>) -> Option<CustomError> {
        match result {
            Ok(_) => None,
            Err(error) => {
                self.error_message = error.to_string();
                self.error_occurred = true;
                self.error_show = true;
                Some(error)
            }
        }
    }

    fn create_database() {
        let path = "todo.txt";

        if !Path::new(path).exists() {
            File::create(path).expect("Error While Creating File");
        }
    }

    fn load_databse(&mut self) -> Result<(), CustomError> {
        
        let file: File = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open("todo.txt")
                                .map_err(|_| CustomError::DatabaseLoadError)?;
        
        let reader: io::BufReader<File> = io::BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|_| CustomError::WriteLineError)?;
            lines.push(line);
        }

        self.todo_list = lines;

        Ok(())
    }

    fn save_database(&mut self) -> Result<(), CustomError> {
        let file = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create(true)
                            .open("todo.txt")
                            .map_err(|_| CustomError::DatabaseSaveError)?;
        
        let mut writer = io::BufWriter::new(file);

        for todo in &self.todo_list {
            let _ = writeln!(writer, "{}", line).map_err(|_| CustomError::WriteLineError)?;
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

                        if !self.error_occurred {
                            let holder = self.save_database();
                            self.error_type = self.handle_error(holder);
                        }

                        self.input_text.clear();
                    }

                    if ui.button("Back").clicked() {
                        self.current_page = Page::HomePage;
                        self.error_occurred = false;
                    }
                });

                if self.error_show {
                    egui::Window::new("ERROR OCCURRED")
                        .collapsible(false)
                        .resizable(false)
                        .show(ctx, |ui| {
                            ui.label(&self.error_message);

                        if ui.button("Close").clicked() {
                            self.error_message = String::new();
                            self.error_occurred = true;
                            self.error_show = false;
                            self.error_type = None;
                        }
                    });
                }
            },
            Page::ViewPage => {
                egui::TopBottomPanel::top("Add Page Top").show(ctx, |ui| {
                    ui.label("View Page")
                });


                //YOU FIXED THE PROBLEM WITH THE VIEW PAGE ERROR OCCURRED POP UP NOT CLOSING NOW FIND A WAY TO MAKE IT WORK FOR THE ADD PAGE

                if !self.error_occurred {
                    let holder = self.load_databse();
                    self.error_type = self.handle_error(holder);
                }

                if self.error_show {
                    egui::Window::new("ERROR OCCURRED")
                        .collapsible(false)
                        .resizable(false)
                        .show(ctx, |ui| {
                            ui.label(&self.error_message);

                        if ui.button("Close").clicked() {
                            self.error_message = String::new();
                            self.error_occurred = true;
                            self.error_show = false;
                            self.error_type = None;
                        }
                    });
                }
        
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
                        self.error_occurred = false;
                    }
                });
            },
        }
    }
}