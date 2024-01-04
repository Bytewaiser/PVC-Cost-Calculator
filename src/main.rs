#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("Pvc", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
    item_count: u32,
    show_settings: bool,
    widths: Vec<u32>,
    heights: Vec<u32>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            item_count: 3,
            show_settings: false,
            widths: vec![40; 100],
            heights: vec![40; 100],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Plise Maliyet Analizi");
            ui.horizontal(|ui| {
                if ui.button("-").clicked() & (self.item_count > 1) {
                    self.item_count -= 1;
                }
                ui.label(format!("{}", self.item_count));
                if ui.button("+").clicked() & (self.item_count < 100) {
                    self.item_count += 1;
                }
            });

            for i in 0..self.item_count {
                ui.horizontal(|ui| {
                    ui.label(format!("Plise - {}:", i));
                    ui.add(
                        egui::DragValue::new(&mut self.widths[i as usize])
                            .clamp_range(40..=500)
                            .prefix("En: ")
                            .suffix(" cm"),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.heights[i as usize])
                            .clamp_range(40..=500)
                            .prefix("Boy: ")
                            .suffix(" cm"),
                    );
                });
            }

            if ui.button("Ayarlar").clicked() {
                self.show_settings = true;
            }
        });

        if self.show_settings {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Immediate Viewport")
                    .with_inner_size([200.0, 100.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Hello from immediate viewport");
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_settings = false;
                    }
                },
            );
        }
    }
}
