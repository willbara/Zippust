use eframe::{egui, App, NativeOptions};
use egui::epaint::CircleShape;

pub struct ZippustApp;

impl App for ZippustApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click());
            let painter = ui.painter_at(rect);

            if response.clicked() {
                println!("clicked");
            }

            let center = rect.center();
            let num_circles = 6;
            let max_radius = rect.width().min(rect.height()) / 2.0;

            for i in 1..=num_circles {
                let radius = max_radius * (i as f32) / (num_circles as f32);
                painter.add(CircleShape {
                    center,
                    radius,
                    fill: egui::Color32::TRANSPARENT,
                    stroke: egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
                });
            }

            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                "+",
                egui::FontId::proportional(50.0),
                egui::Color32::from_gray(80),
            );

            painter.text(
                center + egui::vec2(0.0, 150.0),
                egui::Align2::CENTER_CENTER,
                "drag or drop a file here or click to select",
                egui::FontId::monospace(24.0),
                egui::Color32::WHITE,
            );
        });
    }
}

// Add this function to be called from main.rs
pub fn run() {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Zippust",
        options,
        Box::new(|_cc| Box::new(ZippustApp)),
    );
}