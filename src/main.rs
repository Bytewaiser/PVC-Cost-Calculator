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

#[derive(Clone, PartialEq)]
struct PliseType {
    name: String,
    kasa: f32,
    kanat: f32,
    kose_price: f32,
}

struct Price {
    aluminyum: f32,
    tul: f32,
    serit: f32,
    teker: f32,
    klips: f32,
    stop: f32, // Same as dönüs
}

fn calculate_single_price(width: u32, height: u32, plise_type: PliseType, price: Price) -> f32 {
    let width = width as f32;
    let height = height as f32;

    let kasa = 2. * (width + height - 5.) * plise_type.kasa / 6.;
    let kanat = if width < 150. {
        height - 8. * plise_type.kanat / 6.
    } else {
        height - 8. * plise_type.kanat / 6.
    } * 2.;
    let tul = 1.6 * (width * height);
    let serit = if width < 150. {
        2. * (height - 5.)
    } else {
        4. * (height - 5.)
    };
    let kose = 4. * plise_type.kose_price;
    let teker = if width < 150. { 2. } else { 4. };
    let klips = if width < 150. { 2. } else { 4. };
    let stop = if width < 150. { 2. } else { 4. };
    let dönüş = if width < 150. { 2. } else { 0. };

    let toplam = kasa * price.aluminyum
        + kanat * price.aluminyum
        + tul * price.tul
        + serit * price.serit
        + kose
        + teker * price.teker
        + klips * price.klips
        + stop * price.stop
        + dönüş * price.stop;

    toplam
}

fn calculate_prices(widths: Vec<u32>, heights: Vec<u32>, item_count: u32) {
    todo!()
}

struct MyApp {
    item_count: u32,
    show_settings: bool,
    widths: Vec<u32>,
    heights: Vec<u32>,
    plise_types: Vec<PliseType>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            item_count: 3,
            show_settings: false,
            widths: vec![0; 100],
            heights: vec![0; 100],
            plise_types: vec![
                PliseType {
                    name: "Ince".to_string(),
                    kasa: 1.314,
                    kanat: 2.070,
                    kose_price: 1.,
                };
                100
            ],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.5);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("Plise Maliyet Analizi");
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Ayarlar").clicked() {
                self.show_settings = true;
            }
            ui.horizontal(|ui| {
                ui.label("Plise Adedi: ");
                if ui.button("-").clicked() & (self.item_count > 1) {
                    self.item_count -= 1;
                }
                ui.label(format!("{}", self.item_count));
                if ui.button("+").clicked() & (self.item_count < 100) {
                    self.item_count += 1;
                }
            });

            egui::ScrollArea::vertical()
                .max_height(200.)
                .show(ui, |ui| {
                    egui::Grid::new("grid").min_col_width(70.).show(ui, |ui| {
                        ui.label("Plise");
                        ui.label("En");
                        ui.label("Boy");
                        ui.label("");
                        ui.label("Kasa Tipi");
                        ui.label("");
                        ui.end_row();

                        for i in 0..self.item_count {
                            ui.label(format!("{}:", i + 1));
                            ui.add(
                                egui::DragValue::new(&mut self.widths[i as usize])
                                    .clamp_range(0..=500)
                                    .suffix(" cm"),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.heights[i as usize])
                                    .clamp_range(0..=500)
                                    .suffix(" cm"),
                            );
                            if ui
                                .add(egui::RadioButton::new(
                                    self.plise_types[i as usize].name == "Ince",
                                    "Ince",
                                ))
                                .clicked()
                            {
                                self.plise_types[i as usize] = PliseType {
                                    name: "Ince".to_string(),
                                    kasa: 1.314,
                                    kanat: 2.070,
                                    kose_price: 1.,
                                };
                            }
                            if ui
                                .add(egui::RadioButton::new(
                                    self.plise_types[i as usize].name == "Klasik",
                                    "Klasik",
                                ))
                                .clicked()
                            {
                                self.plise_types[i as usize] = PliseType {
                                    name: "Klasik".to_string(),
                                    kasa: 2.2,
                                    kanat: 2.4,
                                    kose_price: 1.,
                                }
                            }
                            if ui
                                .add(egui::RadioButton::new(
                                    self.plise_types[i as usize].name == "Genis",
                                    "Genis",
                                ))
                                .clicked()
                            {
                                self.plise_types[i as usize] = PliseType {
                                    name: "Genis".to_string(),
                                    kasa: 3.204,
                                    kanat: 2.4,
                                    kose_price: 4.5,
                                };
                            }
                            ui.end_row();
                        }
                    });
                });
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
