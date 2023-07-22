#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui_notify::{Anchor, Toast, Toasts};

use std::{sync::mpsc, thread};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(250.0, 150.0)),
        ..Default::default()
    };
    eframe::run_native(
        "femboytester",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    toasts: Toasts,
    closable: bool,
    duration: f32,

    name: String,
    result: String,

    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            toasts: Toasts::default().with_anchor(Anchor::BottomRight),
            closable: true,
            duration: 3.5,

            name: String::new(),
            result: String::new(),

            tx,
            rx,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            let cb = |t: &mut Toast| {
                // Callback for the toast
                t.set_closable(self.closable)
                    .set_duration(Some(std::time::Duration::from_millis(
                        (1000. * self.duration) as u64,
                    )));
            };
            
                ui.heading("                       Name");
                ui.text_edit_singleline(&mut self.name);

            if ui.button("Check").clicked() {
                let api = "https://api.rghosting.xyz/femboy.php?name=";
                let url = format!("{}{}", api, self.name);
                cb(self.toasts.info("Fetching..."));

                let tx = self.tx.clone();
                thread::spawn(move || {
                    let result = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(fetch_result(&url));
                    tx.send(result).expect("Failed to send result");
                });
            }

            if let Ok(result) = self.rx.try_recv() {
                cb(self.toasts.success("Fetched!"));
                println!("Result: {}", result);
                self.result = result;
            } 

            ui.horizontal(|ui| {
                ui.label("Result:");
                ui.label("%");
                ui.label(self.result.trim());
            });
        });
        self.toasts.show(ctx); // Requests to render toasts
    }
}

// Async function to fetch the result from the API
async fn fetch_result(url: &str) -> String {
    let resp = reqwest::get(url).await.unwrap();
    let body = resp.text().await.unwrap();
    return body;
}
