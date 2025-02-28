use eframe::epaint::{CircleShape, Pos2};
use eframe::glow::RED;
use egui::{Color32, Stroke};
use egui::accesskit::Point;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    points: Vec<Pos2>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            points: Vec::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn add_point(&mut self, point: Pos2) {
        self.points.push(point);
    }

    fn remove_point_if_intersect(&mut self, point: Pos2) {
        self.points.retain(|&p| {
            let distance = (p - point).length();
            distance > 20.0
        });
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let response = ui.interact(ui.max_rect(), ui.id(), egui::Sense::click_and_drag());

            // if pressed or dragged left mouse button
            if response.dragged() || response.clicked() {
                if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
                    if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary)) {
                        self.add_point(pos);
                    } else {
                        self.remove_point_if_intersect(pos);
                    }
                }
            }

            // draw points
            for &point in &self.points {
                painter.add(CircleShape {
                    center: point,
                    radius: 5.0,
                    fill: Color32::RED,
                    stroke: Stroke::new(1.0, Color32::BLACK),
                });
            }
        });
    }
}