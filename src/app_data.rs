use eframe::egui;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::io::Write;
use std::path::Path;
use std::usize;

use crate::custom_err::CustomError;
use crate::to_do::Todo;
use crate::date_time_handler::DateTimeHandler;

pub struct TodoApp {
    current_page : Page,
    input_text : String,
    todo_list : Vec<Todo>,
    error_message : String,
    error_occurred : bool,
    error_show : bool,
    current_todo : Todo,
    entered_number : String,
    parsed_number : usize,
    show_edit_options : bool,
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
            error_occurred : false,
            error_show : false,
            current_todo : Todo::default(),
            entered_number : String::new(),
            parsed_number : usize::max_value(),
            show_edit_options : false,
        }
    }

    fn handle_error(&mut self, result : Result<(), CustomError>) -> Option<CustomError> {
        match result {
            Ok(_) => None,
            Err(error) => {
                self.error_message = error.to_string();
                self.error_occurred = true;
                println!("{}\n", error.to_string());
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

    fn display_list(&mut self, ui :  &mut egui::Ui) {
        egui::ScrollArea::vertical()
        .max_height(100.0)
        .show(ui, |ui| {
            for todo in &self.todo_list  {
                ui.label(format!("{}  Date: {} Time: {}, {}", todo.text.clone(), todo.selected_date.to_string().clone(), 
                todo.selected_hour.to_string().clone(), todo.selected_minute.to_string().clone()));
            }
        });
    }

    fn show_error_window(&mut self, ctx : &egui::Context) {

        egui::Window::new("ERROR OCCURRED")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(self.error_message.to_string());

            if ui.button("Close").clicked() {
                self.error_message = String::new();
                self.error_occurred = true;
                self.error_show = false;
            }
        });
    }

    fn display_back_button(&mut self, ui : &mut egui::Ui) {
        if ui.button("Back").clicked() {
            self.current_page = Page::HomePage;
            self.error_occurred = false;
            self.input_text = String::new();
            self.parsed_number = usize::MAX;
        }
    }

    fn display_top(&self, ctx :&egui::Context, title : String) {
        egui::TopBottomPanel::top(title.clone()).show(ctx, |ui| {
            ui.label(title);
        });
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ADD LOAD DATABASE HERE OR SOMEWHERE ELSE AFTER OVERHAULING ERROR SYSTEM
        match self.current_page {
            Page::HomePage => {
                self.display_top(ctx, String::from("TodoList"));

                egui::CentralPanel::default().show(ctx, |ui| {
        
                    if ui.button("Add Todo").clicked() {
                        self.current_page = Page::AddPage;
                    }
        
                    if ui.button("View Todos").clicked() {
                        self.current_page = Page::ViewPage;
                    }

                    if ui.button("Edit Todo").clicked() {
                        self.current_page = Page::EditPage;
                    }

                    if ui.button("Delete Todo").clicked() {
                        self.current_page = Page::DeletePage;
                    }
        
                    if ui.button("Exit").clicked() {
                        std::process::exit(0)
                    }
                });
            },
            Page::AddPage => {
                self.display_top(ctx, String::from("Add Page"));
        
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Enter Todo: ");
                    ui.add(egui::TextEdit::singleline(&mut self.input_text));   

                    DateTimeHandler::show(ui, &mut self.current_todo);

                    if ui.button("Save Todo").clicked() {

                        self.current_todo.text = self.input_text.clone();

                        self.todo_list.push(self.current_todo.clone());

                        if !self.error_occurred {
                            let holder = self.save_database();
                            let _ = self.handle_error(holder);
                        }

                        self.input_text.clear();
                        self.current_todo = Todo::default();
                    }

                    if self.error_show {
                        self.show_error_window(ctx);
                    }

                    self.display_back_button(ui);
                });

                if self.error_show {
                    self.show_error_window(ctx);
                }
            },
            Page::ViewPage => {
                self.display_top(ctx, String::from("View Page"));

                if !self.error_occurred {
                    let holder = self.load_databse();
                    let _ = self.handle_error(holder);
                }

                if self.error_show {
                    self.show_error_window(ctx);
                }
        
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.display_list(ui);
                    self.display_back_button(ui);
                });
            },
            Page::EditPage => {
                self.display_top(ctx, String::from("Edit Page"));

                if !self.error_occurred {
                    let holder = self.load_databse();
                    let _ = self.handle_error(holder);
                }

                if self.error_show {
                    self.show_error_window(ctx);
                }

                egui::CentralPanel::default().show(ctx, |ui| {
                    self.display_list(ui);

                    ui.add_space(25.0);

                    ui.label("Enter Number Of Which Todo You Want To Edit");
                    ui.add(egui::TextEdit::singleline(&mut self.entered_number));


                    if ui.button("Edit").clicked() {

                        match self.entered_number.trim().parse::<usize>() {
                            Ok(value) => {
                                self.parsed_number = value;
                                self.show_edit_options = true;
                            },
                            Err(_) => {
                                println!("Error OCCURRED IN THE ERR");
                                self.error_message = CustomError::InvalidIndexError.to_string();
                                self.error_show = true;
                            }
                        }
                    }

                    if self.show_edit_options {

                        ui.add_space(25.0);
                        ui.label("New Todo");
                        ui.add(egui::TextEdit::singleline(&mut self.input_text));

                        if ui.button("Confirm").clicked() {

                            if self.parsed_number > self.todo_list.len() || self.parsed_number < 1 {
                                self.error_show = true;
                                self.error_message = CustomError::InvalidIndexError.to_string();
                            }
                            else {
                                self.todo_list[self.parsed_number - 1].text = self.input_text.clone();
                                self.entered_number = String::new();
                                self.parsed_number = usize::max_value();
                                self.input_text = String::new();
                                self.show_edit_options = false;

                                let _ = self.save_database();
                                let _ = self.load_databse();
                            }
                        }
                    }

                    self.display_back_button(ui);
                });
            },

            Page::DeletePage => {
                self.display_top(ctx, String::from("Delete Page"));

                egui::CentralPanel::default().show(ctx, |ui |{
                    self.display_back_button(ui);
                });
            }
        }
    }
}