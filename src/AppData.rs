use eframe::*;
use eframe::egui;
use egui::{CentralPanel, Context, Label, TopBottomPanel};

use crate::HomePage::HomePage;

pub struct TodoApp {
    pub current_page : Page
}

pub enum Page {
    HomePage,
    AddPage,
    ViewPage,
    EditPage,
    RemovePage,
}

impl Default for TodoApp {
    fn default() -> TodoApp {
        TodoApp {
            current_page : Page::HomePage
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        match self.current_page {
            Page::HomePage => {
                let mut page = HomePage::new();
                page.update(ctx, frame);
            },
            Page::AddPage => {
                
            },
            Page::ViewPage => {

            },
            Page::EditPage => {

            },
            Page::RemovePage => {

            }
        }
    }
}