// THIS IS THE ADDTIONAL CHANGE
// MORE!
use eframe::{egui, epi};

#[derive(Default)]
struct App;

impl epi::App for App {
    fn name(&self) -> &str {
        "app"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit()
                    }
                });
            });
        });

        egui::SidePanel::right("right_panel")
            .default_width(20.0)
            .show(ctx, |ui| {
                let mut settings_button = ui.button("Settings");
                if settings_button.clicked() {
                    println!("Opening settings.")
                }
            });

        let mut songs = vec!["We Lift Together", "September"];

        egui::CentralPanel::default().show(ctx, |ui| {
            let label = ui.add(egui::Label::new(songs[0]).sense(egui::Sense::click()));
            if label.clicked() {
                println!("Clicked!");
            }
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
