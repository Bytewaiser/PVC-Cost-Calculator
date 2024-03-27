#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod consumable;
mod plise;
mod price;

use eframe::egui;

// use eframe::Theme;

use consumable::Consumable;
use plise::{ColorName, PliseName};
use price::Price;

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
                                    self.consumables[i as usize].check_plise_name(PliseName::Ince),
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
                                    self.consumables[i as usize]
                                        .check_plise_color(ColorName::Beyaz),
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
                                    self.consumables[i as usize]
                                        .check_plise_color(ColorName::Ahsap),
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

            ui.label("");

            ui.horizontal(|ui| {
                if ui.button("Maliyet Göster").clicked() {
                    self.visibility.show_maliyet = true;
                }
                if ui.button("Fiyat Göster").clicked() {
                    self.visibility.show_price = true;
                }
            })
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
                    let (maliyet, _, _) = self
                        .price
                        .calculate_prices(&self.consumables, self.item_count);
                    ui.horizontal(|ui| {
                        ui.strong("İşci Maliyeti:");
                        ui.add(
                            egui::DragValue::new(&mut self.price.isci_maliyeti)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        )
                    });
                    ui.label("");
                    ui.strong(format!("Maliyet: {:.2}", maliyet));
                    ui.label("");
                    if ui.button("Kapat").clicked() {
                        self.visibility.show_maliyet = false;
                    }
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
                    let (_, total_price, total_price_kdv) = self
                        .price
                        .calculate_prices(&self.consumables, self.item_count);

                    ui.horizontal(|ui| {
                        ui.strong("Kdv:");
                        ui.add(
                            egui::DragValue::new(&mut self.price.kdv)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                    });
                    ui.label("");
                    ui.strong(format!("Fiyat: {:.2}", total_price));
                    ui.strong(format!(
                        "Fiyat (Kdv Dahil - %{:.0}): {:.2}",
                        self.price.kdv, total_price_kdv
                    ));
                    ui.label("");
                    if ui.button("Kapat").clicked() {
                        self.visibility.show_price = false;
                    }
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
                        ui.strong("İnce Seri Kar:");
                        ui.add(
                            egui::DragValue::new(&mut self.price.ince_kar)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                        ui.label("");
                        ui.strong("Klasik Seri Kar:");
                        ui.add(
                            egui::DragValue::new(&mut self.price.klasik_kar)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                        ui.label("");
                        ui.strong("Geniş Seri Kar:");
                        ui.add(
                            egui::DragValue::new(&mut self.price.genis_kar)
                                .clamp_range(0..=100)
                                .speed(0.1),
                        );
                    });
                    ui.label("");
                    ui.horizontal(|ui| {
                        if ui.button("Fiyatları Güncelle").clicked() {
                            self.price.to_file();
                        }
                        if ui.button("Kapat").clicked() {
                            self.visibility.show_settings = false;
                        }
                    });
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.visibility.show_settings = false;
                }
            },
        );
    }
}
