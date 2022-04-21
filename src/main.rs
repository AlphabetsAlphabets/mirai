// THIS IS THE ADDTIONAL CHANGE
// MORE!
use eframe::{egui, epi};

#[derive(Default)]
struct App;

impl epi::App for App {
    fn name(&self) -> &str {
        "Mirai"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit()
                    }
                });

                ui.menu_button("Search", |ui| {
                    if ui.button("Scan for songs").clicked() {
                        println!("Scanning.");
                    }
                });
            });
        });

        let mut songs = vec!["We Lift Together", "September"];

        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(100.0)
            .show(ctx, |ui| {
                ui.columns(3, |columns| {
                    // columns is just an vec of Ui
                    let previous_btn = egui::RichText::new("Previous").size(20.0);
                    let previous_btn = egui::Button::new(previous_btn).small();
                    columns[0].add(previous_btn);

                    let pause_play_btn = egui::RichText::new("Play").size(20.0);
                    columns[1].button(pause_play_btn);

                    let next_btn = egui::RichText::new("Next").size(20.0);
                    columns[2].button(next_btn);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let label = ui.add(egui::Label::new(songs[0]).sense(egui::Sense::click()));
            if label.clicked() {
                println!("Clicked!");
            }
        });
    }
}

fn main() {
    let app = App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
