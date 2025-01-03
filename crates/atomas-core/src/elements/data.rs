use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::elements::element::Element;
use crate::elements::id::Id;
use crate::elements::types::ElementType;

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
                element_type: ElementType::Periodic(1),
                name: Box::leak(name.into_boxed_str()),
                rgb: (red, green, blue),
            };
            elements.push(element);
        }

        Data { elements }
    }
}