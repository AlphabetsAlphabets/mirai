// THIS IS THE ADDTIONAL CHANGE
use eframe::{egui, epi};

#[derive(Default)]
struct App;

impl epi::App for App {
    fn name(&self) -> &str {
        "app"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::Window::new("Test").show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}

fn main() {
    let app = App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
