use crate::plise::{ColorName, PliseName, PliseType};

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

    pub fn set_plise_name(&mut self, plise_name: PliseName) {
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

    pub fn get_plise_color(&self) -> ColorName {
        self.plise_type.get_color()
    }

    pub fn check_plise_color(&self, color: ColorName) -> bool {
        self.get_plise_color() == color
    }

    pub fn get_kasa_cm(&self) -> f32 {
        (2. * (self.width + self.height - 5.) * self.plise_type.get_kasa_weight()).round()
    }

    pub fn get_kanat_cm(&self) -> f32 {
        let val = (self.height - 8.) * self.plise_type.get_kanat_weight();
        if self.width < 150. {
            val.round()
        } else {
            (2. * val).round()
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

    pub fn generate_wh_html_table(&self, idx: usize) -> String {
        format!(
            r#"
    <tr>
        <td>{}</td>
        <td>{}x{}</td>
        <td>{}</td>
        <td>{}</td>
    </tr>
"#,
            idx, self.width, self.height,
            match self.get_plise_name() {
                PliseName::Klasik => "Klasik",
                PliseName::Genis => "Genis",
                PliseName::Ince => "Ince",
            },
            match self.get_plise_color() {
                ColorName::Beyaz => "Beyaz",
                ColorName::Boya => "Boya",
                ColorName::Ahsap => "Ahsap",
            },
        )
    }

    pub fn generate_html_table(&self, idx: usize) -> String {
        format!(
            r#"
    <tr>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
    </tr>
"#,
            idx,
            match self.get_plise_name() {
                PliseName::Klasik => "Klasik",
                PliseName::Genis => "Genis",
                PliseName::Ince => "Ince",
            },
            match self.get_plise_color() {
                ColorName::Beyaz => "Beyaz",
                ColorName::Boya => "Boya",
                ColorName::Ahsap => "Ahsap",
            },
            self.get_kasa_cm(),
            self.get_kanat_cm(),
            self.get_tul_cm_squared(),
            self.get_serit_cm(),
            self.get_kose_adet(),
            self.get_teker_adet(),
            self.get_klips_adet(),
            self.get_stop_adet(),
            self.get_donus_adet(),
        )
    }
}
