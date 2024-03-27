#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod consumable;
mod plise;

use eframe::egui;
use std::io::{Read, Write};
use std::{env, fs::File, fs::OpenOptions};

use serde::{Deserialize, Serialize};
// use eframe::Theme;

use consumable::Consumable;
use plise::{ColorName, PliseName};

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

// Recrate Aluminyum so that every key has named (fiyat:f32, kar_fiyat:f32)
#[derive(Serialize, Deserialize)]
struct ColorPrice {
    beyaz: f32,
    boya: f32,
    ahsap: f32,
}

#[derive(Serialize, Deserialize)]
struct PlisePrice {
    name: PliseName,
    kar: f32,
}

impl PlisePrice {
    fn new(name: PliseName) -> Self {
        match name {
            PliseName::Klasik => PlisePrice {
                name: PliseName::Klasik,
                kar: 20.,
            },
            PliseName::Genis => PlisePrice {
                name: PliseName::Genis,
                kar: 20.,
            },
            PliseName::Ince => PlisePrice {
                name: PliseName::Ince,
                kar: 20.,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Price {
    plise_price: PlisePrice,
    color_price: ColorPrice,
    tul_price: f32,
    serit_price: f32,
    teker_price: f32,
    klips_price: f32,
    stop_price: f32,
    donus_price: f32,
    klasik_kose_price: f32,
    genis_kose_price: f32,
    ince_kose_price: f32,
    isci_maliyeti: f32,
    kdv: f32,
}

impl Default for Price {
    fn default() -> Self {
        Price {
            plise_price: PlisePrice::new(PliseName::Klasik),
            color_price: ColorPrice {
                beyaz: 120.,
                boya: 130.,
                ahsap: 140.,
            },
            tul_price: 30.,
            serit_price: 3.,
            teker_price: 2.5,
            klips_price: 1.,
            stop_price: 1.,
            donus_price: 1.,
            klasik_kose_price: 1.,
            genis_kose_price: 1.,
            ince_kose_price: 4.5,
            isci_maliyeti: 30.,
            kdv: 20.,
        }
    }
}

impl Price {
    fn create_from_file() -> Self {
        let mut exe_path = env::current_exe().unwrap();
        exe_path.set_file_name("prices.json");

        if !exe_path.exists() {
            let mut file = File::create(&exe_path).unwrap();
            file.write_all(serde_json::to_string(&Price::default()).unwrap().as_bytes())
                .unwrap();
        }

        let mut file = File::open(&exe_path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let data = serde_json::from_str(&buffer).unwrap();

        data
    }

    fn to_file(&self) {
        let mut exe_path = env::current_exe().unwrap();
        exe_path.set_file_name("prices.json");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&exe_path)
            .unwrap();
        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())
            .unwrap();
    }

    fn calculate_single_price(&self, consumable: &Consumable) -> f32 {
        let alum_price = match consumable.get_plise_color() {
            ColorName::Beyaz => self.color_price.beyaz,
            ColorName::Boya => self.color_price.boya,
            ColorName::Ahsap => self.color_price.ahsap,
        };

        let kose_price = match consumable.get_plise_name() {
            PliseName::Klasik => self.klasik_kose_price,
            PliseName::Genis => self.genis_kose_price,
            PliseName::Ince => self.ince_kose_price,
        };

        let kasa_maliyet = consumable.get_kasa_cm() * alum_price / 100.;
        let kanat_maliyet = consumable.get_kanat_cm() * alum_price / 100.;
        let tul_maliyet = consumable.get_tul_cm_squared() * self.tul_price / 10000.;
        let serit_maliyet = consumable.get_serit_cm() * self.serit_price / 100.;
        let kose_maliyet = consumable.get_kose_adet() * kose_price;

        let teker_maliyet = consumable.get_teker_adet() * self.teker_price;
        let klips_maliyet = consumable.get_klips_adet() * self.klips_price;
        let stop_maliyet = consumable.get_stop_adet() * self.stop_price;
        let donus_maliyet = consumable.get_donus_adet() * self.donus_price;

        let mut sum_maliyet = kasa_maliyet
            + kanat_maliyet
            + tul_maliyet
            + serit_maliyet
            + kose_maliyet
            + teker_maliyet
            + klips_maliyet
            + stop_maliyet
            + donus_maliyet;

        sum_maliyet *= 1. + self.isci_maliyeti / 100.;
        sum_maliyet
    }

    fn calculate_prices(&self, consumables: &Vec<Consumable>, item_count: u32) -> (f32, f32, f32) {
        let mut maliyet = 0.;

        for i in 0..item_count {
            maliyet += self.calculate_single_price(&consumables[i as usize]);
        }

        let total_price = maliyet * (1. + self.plise_price.kar / 100.);
        let total_price_kdv = total_price * (1. + self.kdv / 100.);

        (maliyet, total_price, total_price_kdv)
    }
}

struct Visibility {
    show_settings: bool,
    show_maliyet: bool,
    show_price: bool,
}

impl Visibility {
    fn new() -> Self {
        Self {
            show_settings: false,
            show_maliyet: false,
            show_price: false,
        }
    }
}

struct MyApp {
    item_count: u32,
    visibility: Visibility,
    consumables: Vec<Consumable>,
    price: Price,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            item_count: 1,
            visibility: Visibility::new(),
            consumables: vec![Consumable::default(); 100],
            price: Price::create_from_file(),
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
                self.visibility.show_settings = true;
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
                                egui::DragValue::new(&mut self.consumables[i as usize].width)
                                    .clamp_range(0..=500)
                                    .suffix(" cm"),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.consumables[i as usize].height)
                                    .clamp_range(0..=500)
                                    .suffix(" cm"),
                            );
                            if ui
                                .add(egui::RadioButton::new(
                                    match self.consumables[i as usize].get_plise_name() {
                                        PliseName::Ince => true,
                                        _ => false,
                                    },
                                    "Ince",
                                ))
                                .clicked()
                            {
                                self.consumables[i as usize].set_plise_name(PliseName::Ince);
                            }
                            if ui
                                .add(egui::RadioButton::new(
                                    self.consumables[i as usize]
                                        .check_plise_name(PliseName::Klasik),
                                    "Klasik",
                                ))
                                .clicked()
                            {
                                self.consumables[i as usize].set_plise_name(PliseName::Klasik);
                            }
                            if ui
                                .add(egui::RadioButton::new(
                                    self.consumables[i as usize].check_plise_name(PliseName::Genis),
                                    "Genis",
                                ))
                                .clicked()
                            {
                                self.consumables[i as usize].set_plise_name(PliseName::Genis);
                            }

                            if ui
                                .add(egui::RadioButton::new(
                                    self.consumables[i as usize].check_plise_color(ColorName::Beyaz),
                                    "Beyaz",
                                ))
                                .clicked()
                            {
                                self.consumables[i as usize].set_plise_color(ColorName::Beyaz);
                            }
                            if ui
                                .add(egui::RadioButton::new(
                                    self.consumables[i as usize].check_plise_color(ColorName::Boya),
                                    "Boyalı",
                                ))
                                .clicked()
                            {
                                self.consumables[i as usize].set_plise_color(ColorName::Boya);
                            }
                            if ui
                                .add(egui::RadioButton::new(
                                    self.consumables[i as usize].check_plise_color(ColorName::Ahsap),
                                    "Ahşap",
                                ))
                                .clicked()
                            {
                                self.consumables[i as usize].set_plise_color(ColorName::Ahsap);
                            }
                            ui.end_row();
                        }
                    });
                });

            if ui.button("Maliyet Göster").clicked() {
                self.visibility.show_maliyet = true;
            }
            if ui.button("Fiyat Göster").clicked() {
                self.visibility.show_price = true;
            }
        });

        if self.visibility.show_maliyet {
            self.show_maliyet(ctx, _frame);
        }

        if self.visibility.show_price {
            self.show_price(ctx, _frame);
        }

        if self.visibility.show_settings {
            self.show_settings(ctx, _frame);
        }
    }
}

impl MyApp {
    fn show_maliyet(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("maliyet_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Maliyet")
                .with_inner_size([500.0, 250.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    let (maliyet, _, _) = self.price.calculate_prices(
                        &self.consumables,
                        self.item_count,
                    );
                    ui.strong(format!("Maliyet: {:.2}", maliyet));
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.visibility.show_maliyet = false;
                }
            },
        );
    }

    fn show_price(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("price_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Fiyat")
                .with_inner_size([500.0, 250.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    let (_, total_price, total_price_kdv) = self.price.calculate_prices(
                        &self.consumables,
                        self.item_count,
                    );
                    ui.strong(format!("Fiyat: {:.2}", total_price));
                    ui.strong(format!(
                        "Fiyat (Kdv Dahil - %{:.0}): {:.2}",
                        self.price.kdv, total_price_kdv
                    ));
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.visibility.show_price = false;
                }
            },
        );
    }

    fn show_settings(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("settings_viewport"),
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
                        ui.strong("Beyaz");
                        ui.strong("Boyalı");
                        ui.strong("Ahsap");
                        ui.strong("İnce");
                        ui.strong("Klasik");
                        ui.strong("Geniş");
                        ui.end_row();

                        ui.add(
                            egui::DragValue::new(&mut self.price.color_price.beyaz)
                                .clamp_range(0..=10000),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.price.color_price.boya)
                                .clamp_range(0..=10000),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.price.color_price.ahsap)
                                .clamp_range(0..=10000),
                        );

                        ui.add(
                            egui::DragValue::new(&mut self.price.ince_kose_price)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.price.klasik_kose_price)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.price.genis_kose_price)
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
                            egui::DragValue::new(&mut self.price.tul_price)
                                .clamp_range(0..=1000)
                                .speed(0.2),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.price.serit_price)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.price.teker_price)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );

                        ui.add(
                            egui::DragValue::new(&mut self.price.klips_price)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.price.stop_price)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.price.donus_price)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                    });

                    ui.label("");
                    ui.horizontal(|ui| {
                        ui.label("İşci Maliyeti:");
                        ui.add(
                            egui::DragValue::new(&mut self.price.isci_maliyeti)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                        ui.label("Kar:");
                        ui.add(
                            egui::DragValue::new(&mut self.price.plise_price.kar)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                        ui.label("Kdv:");
                        ui.add(
                            egui::DragValue::new(&mut self.price.kdv)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                    });
                    if ui.button("Fiyatları Güncelle").clicked() {
                        self.price.to_file();
                    }
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.visibility.show_settings = false;
                }
            },
        );
    }
}
