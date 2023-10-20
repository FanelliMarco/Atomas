use std::fmt;

pub struct Element {
    id: [char; 2],
    name: String,
    rgb: (u8, u8, u8),
}

impl Element {
    pub fn new(id: [char; 2], name: String, rgb: (u8, u8, u8)) -> Self {
        Self {
            id,
            name,
            rgb
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} ({}, {}, {})",
            self.id.iter().collect::<String>(),
            self.name,
            self.rgb.0,
            self.rgb.1,
            self.rgb.2,
        )
    }
}
