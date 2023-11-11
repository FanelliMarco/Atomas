use std::fmt;

enum Id {
    Single(char),
    Double([char; 2]),
}

impl Id {
    fn from_chars(chars: &[char]) -> Self {
        match chars.len() {
            1 => Id::Single(chars[0]),
            2 => Id::Double([chars[0], chars[1]]),
            _ => panic!("Invalid id length"),
        }
    }
}

pub struct Element {
    id: Id,
    name: String,
    rgb: (u8, u8, u8),
}
/*
#[derive(Serialize, Deserialize)]
pub struct Data {
    elements: Vec<Element>,
}
*/
impl Element {
    pub fn new(id: &str, name: String, rgb: (u8, u8, u8)) -> Self {
        let id = Id::from_chars(id.chars().collect::<Vec<char>>().as_slice());

        Self { id, name, rgb }
    }
}
/*
impl Data {
pub fn load(path: &str) -> Data {
         let file = File::open("../assets/txt/elements.txt").unwrap();
         let reader = BufReader::new(file);
         let elements = Vec::new();
         for line in reader.lines() {

         }


    }
}
*/
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.id {
            Id::Single(c) => write!(f, "{}", c)?,
            Id::Double(id) => write!(f, "{}", id.iter().collect::<String>())?,
        }

        write!(
            f,
            " {} ({}, {}, {})",
            self.name, self.rgb.0, self.rgb.1, self.rgb.2
        )
    }
}
