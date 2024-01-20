#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
// use eframe::Theme;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        // default_theme: Theme::Light,
        // follow_system_theme: false,
        ..Default::default()
    };
    eframe::run_native("Pvc", options, Box::new(|_cc| Box::<MyApp>::default()))
}

#[derive(Clone)]
enum PliseName {
    Klasik,
    Genis,
    Ince,
}

#[derive(Clone)]
enum ColorName {
    Beyaz,
    Boya,
    Ahsap,
}

#[derive(Clone)]
struct PliseType {
    name: PliseName,
    boya: ColorName,
    kasa: f32,
    kanat: f32,
}

impl PliseType {
    fn new(name: PliseName, boya: ColorName) -> Self {
        match name {
            PliseName::Klasik => Self {
                name,
                boya,
                kasa: 2.2,
                kanat: 2.4,
            },
            PliseName::Genis => Self {
                name,
                boya,
                kasa: 3.204,
                kanat: 2.4,
            },
            PliseName::Ince => Self {
                name,
                boya,

                kasa: 1.314,
                kanat: 2.070,
            },
        }
    }
}

struct Aluminyum {
    beyaz: f32,
    boya: f32,
    ahsap: f32,
}

struct Kose {
    ince: f32,
    klasik: f32,
    genis: f32,
}

struct Price {
    aluminyum: Aluminyum,
    tul: f32,
    serit: f32,
    kose: Kose,
    teker: f32,
    klips: f32,
    stop: f32,
    donus: f32,
}

impl Price {
    fn new() -> Self {
        Price {
            aluminyum: Aluminyum {
                beyaz: 120.,
                boya: 130.,
                ahsap: 140.,
            },
            tul: 30.,
            serit: 3.,
            kose: Kose {
                ince: 1.,
                klasik: 1.,
                genis: 4.5,
            },
            teker: 2.5,
            klips: 1.,
            stop: 1.,
            donus: 1.,
        }
    }
}

fn calculate_single_price(width: u32, height: u32, plise_type: &PliseType, price: &Price) -> f32 {
    let width = width as f32;
    let height = height as f32;

    let kasa_weight = plise_type.kasa / 6.;
    let kanat_weight = plise_type.kanat / 6.;

    let kose_price = match plise_type.name {
        PliseName::Klasik => price.kose.klasik,
        PliseName::Genis => price.kose.genis,
        PliseName::Ince => price.kose.ince,
    };

    let alum_price = match plise_type.boya {
        ColorName::Beyaz => price.aluminyum.beyaz,
        ColorName::Boya => price.aluminyum.boya,
        ColorName::Ahsap => price.aluminyum.ahsap,
    } / 100.;

    let kasa = (2. * (width + height - 5.) * kasa_weight) * alum_price;
    let kanat = if width < 150. {
        (height - 8.) * kanat_weight
    } else {
        2. * (height - 8.) * kanat_weight
    } * alum_price;

    let tul = width * height * price.tul / 10000.;
    let serit = if width < 150. {
        2. * (height - 5.)
    } else {
        4. * (height - 5.)
    } * price.serit
        / 100.;
    let kose = 4. * kose_price;
    let teker = if width < 150. { 2. } else { 4. } * price.teker;
    let klips = if width < 150. { 4. } else { 8. } * price.klips;
    let stop = if width < 150. { 2. } else { 4. } * price.stop;
    let donus = if width < 150. { 2. } else { 0. } * price.donus;

    let sum = kasa + kanat + tul + serit + kose + teker + klips + stop + donus;

    sum
}

fn calculate_prices(
    widths: &Vec<u32>,
    heights: &Vec<u32>,
    plise_types: &Vec<PliseType>,
    item_count: u32,
    price: &Price,
) -> f32 {
    let mut sum = 0.;

    for i in 0..item_count {
        sum += calculate_single_price(
            widths[i as usize],
            heights[i as usize],
            &plise_types[i as usize],
            &price,
        );
    }

    sum
}

struct MyApp {
    item_count: u32,
    show_settings: bool,
    widths: Vec<u32>,
    heights: Vec<u32>,
    plise_types: Vec<PliseType>,
    price: Price,
}

impl Default for MyApp {
    fn default() -> Self {
        let price = Price::new();

        Self {
            item_count: 1,
            show_settings: false,
            widths: vec![40; 100],
            heights: vec![40; 100],
            plise_types: vec![
                PliseType {
                    name: PliseName::Klasik,
                    boya: ColorName::Beyaz,
                    kasa: 1.314,
                    kanat: 2.070,
                };
                100
            ],
            price,
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
                ui.strong("Plise Adedi: ");
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
                        ui.strong("Plise");
                        ui.strong("En");
                        ui.strong("Boy");
                        ui.label("");
                        ui.strong("Kasa Tipi");
                        ui.label("");
                        ui.label("");
                        ui.strong("Boya Tipi");
                        ui.label("");
                        ui.end_row();

                        for i in 0..self.item_count {
                            ui.strong(format!("{}:", i + 1));
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
                                    match self.plise_types[i as usize].name {
                                        PliseName::Ince => true,
                                        _ => false,
                                    },
                                    "Ince",
                                ))
                                .clicked()
                            {
                                self.plise_types[i as usize] =
                                    PliseType::new(PliseName::Ince, ColorName::Beyaz);
                            }
                            if ui
                                .add(egui::RadioButton::new(
                                    match self.plise_types[i as usize].name {
                                        PliseName::Klasik => true,
                                        _ => false,
                                    },
                                    "Klasik",
                                ))
                                .clicked()
                            {
                                self.plise_types[i as usize] =
                                    PliseType::new(PliseName::Klasik, ColorName::Beyaz);
                            }
                            if ui
                                .add(egui::RadioButton::new(
                                    match self.plise_types[i as usize].name {
                                        PliseName::Genis => true,
                                        _ => false,
                                    },
                                    "Genis",
                                ))
                                .clicked()
                            {
                                self.plise_types[i as usize] =
                                    PliseType::new(PliseName::Genis, ColorName::Beyaz);
                            }

                            if ui
                                .add(egui::RadioButton::new(
                                    match self.plise_types[i as usize].boya {
                                        ColorName::Beyaz => true,
                                        _ => false,
                                    },
                                    "Beyaz",
                                ))
                                .clicked()
                            {
                                self.plise_types[i as usize].boya = ColorName::Beyaz;
                            }
                            if ui
                                .add(egui::RadioButton::new(
                                    match self.plise_types[i as usize].boya {
                                        ColorName::Boya => true,
                                        _ => false,
                                    },
                                    "Boyalı",
                                ))
                                .clicked()
                            {
                                self.plise_types[i as usize].boya = ColorName::Boya;
                            }
                            if ui
                                .add(egui::RadioButton::new(
                                    match self.plise_types[i as usize].boya {
                                        ColorName::Ahsap => true,
                                        _ => false,
                                    },
                                    "Ahşap",
                                ))
                                .clicked()
                            {
                                self.plise_types[i as usize].boya = ColorName::Ahsap;
                            }
                            ui.end_row();
                        }
                    });
                });
            let calculated_price = calculate_prices(
                &self.widths,
                &self.heights,
                &self.plise_types,
                self.item_count,
                &self.price,
            );
            ui.strong(format!("Maliyet: {}", calculated_price.ceil()));
        });

        if self.show_settings {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Fiyatlar Listesi")
                    .with_inner_size([500.0, 250.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.heading("Fiyatlar");
                        });
                    });
                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui::Grid::new("grid").min_col_width(70.).show(ui, |ui| {
                            ui.label("");
                            ui.strong("Boya Fiyatı");
                            ui.label("");
                            ui.label("");
                            ui.strong("Köşe Fiyatı");
                            ui.label("");
                            ui.end_row();
                            ui.colored_label(egui::Color32::WHITE, "Beyaz");
                            ui.colored_label(egui::Color32::GREEN, "Boyalı");
                            ui.colored_label(egui::Color32::YELLOW, "Ahsap");
                            ui.strong("İnce");
                            ui.strong("Klasik");
                            ui.strong("Geniş");
                            ui.end_row();

                            ui.add(
                                egui::DragValue::new(&mut self.price.aluminyum.beyaz)
                                    .clamp_range(0..=10000),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.price.aluminyum.boya)
                                    .clamp_range(0..=10000),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.price.aluminyum.ahsap)
                                    .clamp_range(0..=10000),
                            );

                            ui.add(
                                egui::DragValue::new(&mut self.price.kose.ince)
                                    .clamp_range(0..=100)
                                    .speed(0.1),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.price.kose.klasik)
                                    .clamp_range(0..=100)
                                    .speed(0.1),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.price.kose.genis)
                                    .clamp_range(0..=100)
                                    .speed(0.1),
                            );
                            ui.end_row();
                            ui.end_row();

                            ui.strong("Tul");
                            ui.strong("Şerit");
                            ui.strong("Teker");
                            ui.strong("Klips");
                            ui.strong("Stop");
                            ui.strong("Dönüş");
                            ui.end_row();

                            ui.add(
                                egui::DragValue::new(&mut self.price.tul)
                                    .clamp_range(0..=1000)
                                    .speed(0.2),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.price.serit)
                                    .clamp_range(0..=100)
                                    .speed(0.1),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.price.teker)
                                    .clamp_range(0..=100)
                                    .speed(0.1),
                            );

                            ui.add(
                                egui::DragValue::new(&mut self.price.klips)
                                    .clamp_range(0..=100)
                                    .speed(0.1),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.price.stop)
                                    .clamp_range(0..=100)
                                    .speed(0.1),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.price.donus)
                                    .clamp_range(0..=100)
                                    .speed(0.1),
                            );
                        });
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
