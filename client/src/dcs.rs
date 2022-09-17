#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Aircraft {
    F_16Cb50,
    FA_18C,
    AH_64D,
    Unknown,
}

impl Aircraft {
    pub fn get_friendly_name(&self) -> &str {
        match self {
            Aircraft::F_16Cb50 => "F-16C block 50",
            Aircraft::FA_18C => "F/A 18C Hornet",
            Aircraft::AH_64D => "AH-64D Apache",
            _ => "Unsupported aircraft",
        }
    }
}

pub fn aircraft_by_name(name: String) -> Aircraft {
    match name.as_str() {
        "F-16C_50" => Aircraft::F_16Cb50,
        _ => Aircraft::Unknown,
    }
}
