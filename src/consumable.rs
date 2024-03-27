use crate::plise::{PliseName, ColorName, PliseType};

#[derive(Clone)]
pub struct Consumable {
    pub plise_type: PliseType,
    pub width: f32,
    pub height: f32,
}

impl Default for Consumable {
    fn default() -> Self {
        Self {
            plise_type: PliseType::default(),
            width: 40.,
            height: 40.,
        }
    }
}

impl Consumable {
    fn _new(plise_type: PliseType, width: u32, height: u32) -> Self {
        Self {
            plise_type,
            width: width as f32,
            height: height as f32,
        }
    }

    pub fn set_plise_name(&mut self, plise_name: PliseName){
        self.plise_type.set_name(plise_name)
    }
    
    pub fn get_plise_name(&self) -> PliseName {
        self.plise_type.get_name()
    }
    
    pub fn check_plise_name(&self, plise_name: PliseName) -> bool {
        self.get_plise_name() == plise_name
    }

    pub fn set_plise_color(&mut self, color: ColorName) {
        self.plise_type.set_color(color)
    }
    
    pub fn get_plise_color(&self) ->  ColorName {
        self.plise_type.get_color()
    }

    pub fn check_plise_color(&self, color: ColorName) -> bool {
        self.get_plise_color() == color
    }


    pub fn get_kasa_cm(&self) -> f32 {
        2. * (self.width + self.height - 5.) * self.plise_type.get_kasa_weight()
    }

    pub fn get_kanat_cm(&self) -> f32 {
        let val = (self.height - 8.) * self.plise_type.get_kanat_weight();
        if self.width < 150. {
            val
        } else {
            2. * val
        }
    }

    pub fn get_tul_cm_squared(&self) -> f32 {
        self.width * self.height
    }

    pub fn get_serit_cm(&self) -> f32 {
        if self.width < 150. {
            2. * (self.height - 5.)
        } else {
            4. * (self.height - 5.)
        }
    }

    pub fn get_kose_adet(&self) -> f32 {
        4.
    }

    pub fn get_teker_adet(&self) -> f32 {
        if self.width < 150. {
            2.
        } else {
            4.
        }
    }

    pub fn get_klips_adet(&self) -> f32 {
        if self.width < 150. {
            4.
        } else {
            8.
        }
    }

    pub fn get_stop_adet(&self) -> f32 {
        if self.width < 150. {
            2.
        } else {
            4.
        }
    }

    pub fn get_donus_adet(&self) -> f32 {
        if self.width < 150. {
            2.
        } else {
            0.
        }
    }
}

