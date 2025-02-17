use eframe::{egui, App, NativeOptions};
use egui_extras::RetainedImage;
use egui::ColorImage;
use rand::Rng;
use std::time::Instant;

pub struct ZippustApp {
    folder_icon: RetainedImage,
    dots: Vec<(egui::Pos2, f32, f32)>,
    last_update: Instant,
}

impl Default for ZippustApp {
    fn default() -> Self {
        let image_data = include_bytes!("../assets/folder_icon.png");
        let color_image = image::load_from_memory(image_data)
            .expect("Failed to load image")
            .to_rgba8();
        let size = [color_image.width() as usize, color_image.height() as usize];
        let pixels = color_image.into_raw();
        let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
        let folder_icon = RetainedImage::from_color_image("folder_icon", color_image);

        let mut rng = rand::thread_rng();
        let dots = (0..50)
            .map(|_| {
                let x = rng.gen_range(0.0..531.0);
                let y = rng.gen_range(0.0..540.0);
                let speed = rng.gen_range(0.5..1.0);
                let drift = rng.gen_range(-0.5..0.5);
                (egui::pos2(x, y), speed, drift)
            })
            .collect();

        Self {
            folder_icon,
            dots,
            last_update: Instant::now(),
        }
    }
}

impl App for ZippustApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let delta_time = (now - self.last_update).as_secs_f32();
        self.last_update = now;

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_gray(20)))
            .show(ctx, |ui| {
                let painter = ui.painter();

                // Animate dots
                for (pos, speed, drift) in &mut self.dots {
                    pos.y += *speed * delta_time * 100.0;
                    pos.x += *drift * delta_time * 50.0;

                    if pos.y > 540.0 || pos.x < 0.0 || pos.x > 531.0 {
                        pos.y = 0.0;
                        pos.x = rand::thread_rng().gen_range(0.0..531.0);
                    }

                    let alpha = ((pos.y / 540.0) * 255.0) as u8;
                    painter.circle_filled(
                        *pos,
                        1.5,
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
                    );
                }

                ui.vertical_centered(|ui| {
                    // Increased initial spacing to move content down
                    ui.add_space(120.0);  // Changed from 50.0 to 120.0
                    
                    self.folder_icon.show_size(ui, egui::vec2(80.0, 80.0));
                    ui.add_space(20.0);
                    
                    ui.label(
                        egui::RichText::new("Drag your documents, photos, or videos here to start encrypting.")
                            .size(18.0)
                            .color(egui::Color32::LIGHT_GRAY),
                    );
                    ui.add_space(15.0);
                    
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Browse files")
                                    .size(18.0)
                                    .color(egui::Color32::WHITE),
                            )
                            .min_size(egui::vec2(150.0, 40.0)),
                        )
                        .clicked()
                    {
                        println!("Browse clicked");
                    }
                });
            });

        ctx.request_repaint();
    }
}

pub fn run() {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(535.0, 540.0)),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Zippust",
        options,
        Box::new(|_cc| Box::new(ZippustApp::default())),
    );
}