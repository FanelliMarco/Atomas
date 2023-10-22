use std::fmt;

pub struct Element {
    id: [char; 2],
    name: String,
    rgb: (u8, u8, u8),
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    elements: Vec<Element>,
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

impl Data {
pub fn load(path: &str) -> Data {
         let file = File::open("../assets/txt/elements.txt").unwrap();
         let reader = BufReader::new(file);
         let elements = Vec::new();
         for line in reader.lines() {
             
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
