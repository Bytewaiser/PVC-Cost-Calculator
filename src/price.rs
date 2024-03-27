use crate::consumable::Consumable;
use crate::plise::{ColorName, PliseName};

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::{env, fs::File, fs::OpenOptions};

#[derive(Serialize, Deserialize)]
pub struct ColorPrice {
    pub beyaz: f32,
    pub boya: f32,
    pub ahsap: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Price {
    pub plise_name: PliseName,
    pub color_price: ColorPrice,
    pub tul_price: f32,
    pub serit_price: f32,
    pub teker_price: f32,
    pub klips_price: f32,
    pub stop_price: f32,
    pub donus_price: f32,
    pub klasik_kose_price: f32,
    pub genis_kose_price: f32,
    pub ince_kose_price: f32,
    pub klasik_kar: f32,
    pub genis_kar: f32,
    pub ince_kar: f32,
    pub isci_maliyeti: f32,
    pub kdv: f32,
}

impl Default for Price {
    fn default() -> Self {
        Price {
            plise_name: PliseName::Klasik,
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
            klasik_kar: 20.,
            genis_kar: 20.,
            ince_kar: 20.,
            isci_maliyeti: 30.,
            kdv: 20.,
        }
    }
}

impl Price {
    pub fn create_from_file() -> Self {
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

    pub fn to_file(&self) {
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

    pub fn calculate_prices(&self, consumables: &Vec<Consumable>, item_count: u32) -> (f32, f32, f32) {
        let mut maliyet = 0.;
        let mut total_price = 0.;

        for i in 0..item_count {
            maliyet += self.calculate_single_price(&consumables[i as usize]);
            let kar = match consumables[i as usize].get_plise_name() {
                PliseName::Klasik => self.klasik_kar,
                PliseName::Genis => self.genis_kar,
                PliseName::Ince => self.ince_kar,

            };

            total_price += maliyet * (1. + kar / 100.)
        }

        let total_price_kdv = total_price * (1. + self.kdv / 100.);

        (maliyet, total_price, total_price_kdv)
    }
}
