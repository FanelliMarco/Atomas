use std::fmt;

pub struct Element {
    name: String,
    rgb: (u8, u8, u8),
}

impl Element {
    pub fn new(name: String, rgb: (u8, u8, u8)) -> Self {
        Self {
            name,
            rgb
        }
    }
    
    pub fn get_brightness(&self) -> u8 {
        let r = self.rgb.0 as u32;
        let g = self.rgb.1 as u32;
        let b = self.rgb.2 as u32;

        let brightness = (r + g + b) / 3;

        brightness as u8
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}, {}, {})",
            self.name,
            self.rgb.0,
            self.rgb.1,
            self.rgb.2,
        )
    }
}
