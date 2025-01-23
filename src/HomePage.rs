use crate::AppData::Page;

pub struct HomePage {
    current_page : Page,
}

impl HomePage {
    pub fn new() -> Self {
        Self {
            current_page : Page::HomePage,
        }
    }
}

impl eframe::App for HomePage {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("home_top").show(ctx, |ui| {
            ui.label("TodoList")
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            if let _ = ui.button("Add Todo").clicked() {
                self.current_page = Page::AddPage;
            }

            if let _ = ui.button("View Todos").clicked() {
                self.current_page = Page::ViewPage;
            }

            if let _ = ui.button("Edit Todos").clicked() {
                self.current_page = Page::EditPage;
            }

            if let _ = ui.button("Remove Todo").clicked() {
                self.current_page = Page::RemovePage;
            }
            
            if let _ = ui.button("Exit").clicked() {

            }
        });
            
    }
}