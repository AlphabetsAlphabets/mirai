use dirs::config_dir;
use eframe::{egui, epi};
use std::process::Command;

#[derive(Default)]
struct App;

impl App {
    fn scan_songs(&self) {
        let home_dir = std::env::var("HOME").unwrap();
        let home_dir = home_dir.as_str();

        let output = Command::new("fd")
            .args(["-t", "f", ".mp3", home_dir])
            .output()
            .expect("You either do not have `fd` installed, or are on Windows.");

        for value in output.stdout {
            println!("{}", value as char);
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Mirai"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // Menu bar at the top
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit()
                    }
                });

                ui.menu_button("Search", |ui| {
                    let scan_btn = ui.button("Scan for songs");
                    if scan_btn.clicked() {
                        println!("Scanning.");
                        self.scan_songs();
                    } else {
                        scan_btn.on_hover_ui(|ui| {
                            ui.label("Scan for songs throughout the entire computer.");
                        });
                    }

                    let change_dir_btn = ui.button("Change default directory");
                    if change_dir_btn.clicked() {
                        println!("dir");
                    } else {
                        change_dir_btn.on_hover_ui(|ui| {
                            ui.label("Change the default directory Mirai looks for songs in.");
                        });
                    }
                });
            });
        });

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
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {});
        });
    }
}

fn main() {
    let app = App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
