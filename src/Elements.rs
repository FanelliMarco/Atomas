use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Serialize, Deserialize)]
enum Id {
    Single(char),
    Double([char; 2]),
    Triple([char; 3]),
}

impl Id {
    fn from_chars(chars: &[char]) -> Self {
        match chars.len() {
            1 => Id::Single(chars[0]),
            2 => Id::Double([chars[0], chars[1]]),
            3 => Id::Triple([chars[0], chars[1], chars[2]]),
            _ => panic!("Invalid id length"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Element {
    pub id: Id,
    pub name: String,
    pub rgb: (u8, u8, u8),
}

impl Element {
    pub fn new(id: &str, name: String, rgb: (u8, u8, u8)) -> Self {
        let id = Id::from_chars(id.chars().collect::<Vec<char>>().as_slice());

        Self { id, name, rgb }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.id {
            Id::Single(c) => write!(f, "{}", c)?,
            Id::Double(id) => write!(f, "{}", id.iter().collect::<String>())?,
            Id::Triple(id) => write!(f, "{}", id.iter().collect::<String>())?,
        }

        write!(
            f,
            " {} ({}, {}, {})",
            self.name, self.rgb.0, self.rgb.1, self.rgb.2
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub elements: Vec<Element>,
}

impl Data {
    pub fn load(path: &str) -> Data {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut elements = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(r"\-").collect();

            let id = parts[0];
            let name = parts[1];
            let rgb: Vec<&str> = parts[2].split(',').collect();

            let red = rgb[0].parse::<u8>().unwrap();
            let green = rgb[1].parse::<u8>().unwrap();
            let blue = rgb[2].parse::<u8>().unwrap();

            let element = Element::new(id, name.to_string(), (red, green, blue));
            elements.push(element);
        }

        Data { elements }
    }
}
