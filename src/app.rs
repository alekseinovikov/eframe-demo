use crate::core::todo::Todo;
use crate::service::Service;
use crate::storage::Storage;
use egui::Ui;

pub struct TemplateApp {
    service: Service,
    selected_todo: Option<Todo>,
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let storage = Storage::new("todo.db".to_string());
        let service = Service::new(storage);
        TemplateApp {
            service,
            selected_todo: None,
        }
    }

    fn draw_top_bar(&self, ctx: &egui::Context, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.add_space(16.0);
            egui::widgets::global_theme_preference_buttons(ui);
        });
    }

    fn draw_todo_list(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let todos = self.service.get_all_todos();
        for todo in todos {
            let is_selected = self.selected_todo.as_ref().is_some_and(|t| t.id == todo.id);
            let button = ui.selectable_label(is_selected, todo.title.clone());
            if button.clicked() {
                self.selected_todo = Some(todo);
            }
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.draw_top_bar(ctx, ui);
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            self.draw_todo_list(ctx, ui);
        });
    }
}
