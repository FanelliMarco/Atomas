use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Id {
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

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Element<'a> {
    pub id: Id,
    #[serde(borrow)]
    pub name: &'a str,
    pub rgb: (u8, u8, u8),
}

impl<'a> fmt::Display for Element<'a> {
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

#[derive(Debug)]
pub struct Data<'a> {
    pub elements: Vec<Element<'a>>,
}

impl Data<'static> {
    pub fn load(path: &str) -> Data<'static> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut elements = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(r"\-").map(|s| s.trim()).collect();
            let parts: Vec<String> = parts.iter().map(|s| s.to_string()).collect();

            let id = parts[0].clone();
            let name = parts[1].clone();
            let rgb: Vec<&str> = parts[2].split(',').map(|s| s.trim()).collect();

            let red = rgb[0].parse::<u8>().unwrap();
            let green = rgb[1].parse::<u8>().unwrap();
            let blue = rgb[2].parse::<u8>().unwrap();

            let element = Element {
                id: Id::from_chars(id.chars().collect::<Vec<char>>().as_slice()),
                name: Box::leak(name.into_boxed_str()),
                rgb: (red, green, blue),
            };
            elements.push(element);
        }

        Data { elements }
    }
}
