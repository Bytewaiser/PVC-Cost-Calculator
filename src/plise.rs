use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Debug)]
pub enum PliseName {
    Klasik,
    Genis,
    Ince,
}

#[derive(Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum ColorName {
    Beyaz,
    Boya,
    Ahsap,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PliseType {
    name: PliseName,
    color: ColorName,
    kasa_weight_times_six: f32,
    kanat_weight_times_six: f32,
}

impl Default for PliseType {
    fn default() -> Self {
        Self {
            name: PliseName::Klasik,
            color: ColorName::Beyaz,
            kasa_weight_times_six: 2.2,
            kanat_weight_times_six: 2.4,
        }
    }
}

impl PliseType {
    fn _new(name: PliseName, boya: ColorName) -> Self {
        match name {
            PliseName::Klasik => Self {
                name,
                color: boya,
                kasa_weight_times_six: 2.2,
                kanat_weight_times_six: 2.4,
            },
            PliseName::Genis => Self {
                name,
                color: boya,
                kasa_weight_times_six: 3.204,
                kanat_weight_times_six: 2.4,
            },
            PliseName::Ince => Self {
                name,
                color: boya,
                kasa_weight_times_six: 1.314,
                kanat_weight_times_six: 2.070,
            },
        }
    }

    pub fn set_name(&mut self, name: PliseName) {
        match name {
            PliseName::Klasik => {
                self.name = PliseName::Klasik;
                self.kasa_weight_times_six = 2.2;
                self.kanat_weight_times_six = 2.4;
            }
            PliseName::Genis => {
                self.name = PliseName::Genis;
                self.kasa_weight_times_six = 3.204;
                self.kanat_weight_times_six = 2.4;
            }
            PliseName::Ince => {
                self.name = PliseName::Ince;
                self.kasa_weight_times_six = 1.314;
                self.kanat_weight_times_six = 2.070;
            }
        }
    }

    pub fn set_color(&mut self, boya: ColorName) {
        self.color = boya;
    }

    pub fn get_kasa_weight(&self) -> f32 {
        self.kasa_weight_times_six / 6.0
    }

    pub fn get_kanat_weight(&self) -> f32 {
        self.kanat_weight_times_six / 6.0
    }

    pub fn get_color(&self) -> ColorName {
        self.color
    }

    pub fn get_name(&self) -> PliseName {
        self.name
    }
}
