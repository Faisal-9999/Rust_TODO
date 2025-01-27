use eframe::egui;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::io::Write;
use std::path::Path;

use crate::custom_err::CustomError;
use crate::to_do::Todo;
use crate::date_time_handler::DateTimeHandler;

pub struct TodoApp {
    current_page : Page,
    input_text : String,
    todo_list : Vec<Todo>,
    error_type : Option<CustomError>,
    error_message : String,
    error_occurred : bool,
    error_show : bool,
    current_todo : Todo,
}

//ADD AN EDIT TODO and delete todo option
//NOTIFICATION FEATURE HAS BEEN ABANDONED SINCE YOU ARE ASS AT RUST

enum Page {
    HomePage,
    AddPage,
    ViewPage,
    EditPage,
    DeletePage,
}

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
            current_todo : Todo::default(),
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

    //MAKE THEIR PAGES AND ADD THE FRONT END SHIT FOR TO TAKE INPUT FOR THESE FUNCTIONS TO WORK
    fn delete_todo(&mut self, index : usize) -> Result<(), CustomError> {
        if index < self.todo_list.len() {
            Err(CustomError::InvalidIndexError)?
        }

        self.todo_list.remove(index);

        Ok(())
    }

    fn edit_todo(&mut self, index : usize, new_text : String) -> Result<(), CustomError> {
        if index < self.todo_list.len() {
            Err(CustomError::InvalidIndexError)?
        }

        self.todo_list[index].text = new_text;

        Ok(())
    }

    fn load_databse(&mut self) -> Result<(), CustomError> {
        
        let file = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create(true)
                            .open("todo.txt")
                            .map_err(|_| CustomError::DatabaseLoadError)?;
        
        let reader: io::BufReader<File> = io::BufReader::new(file);
        let mut todos: Vec<Todo> = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|_| CustomError::WriteLineError)?;
            todos.push(Todo::from_string(&line));
        }

        self.todo_list = todos;

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
            let _ = writeln!(writer, "{}", todo.to_string()).map_err(|_| CustomError::WriteLineError)?;
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

                    DateTimeHandler::show(ui, &mut self.current_todo);


                    if ui.button("Save Todo").clicked() {

                        self.current_todo.text = self.input_text.clone();

                        self.todo_list.push(self.current_todo.clone());

                        if !self.error_occurred {
                            let holder = self.save_database();
                            self.error_type = self.handle_error(holder);
                        }

                        self.input_text.clear();
                        self.current_todo = Todo::default();
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
                            for todo in &self.todo_list  {
                                ui.label(format!("{}  Date: {} Time: {}, {}", todo.text.clone(), todo.selected_date.to_string().clone(), 
                                todo.selected_hour.to_string().clone(), todo.selected_minute.to_string().clone()));
                            }
                        }
                    );

                    if ui.button("Back").clicked() {
                        self.current_page = Page::HomePage;
                        self.error_occurred = false;
                    }
                });
            },
            Page::EditPage => {

            },
            Page::DeletePage => {

            }
        }
    }
}