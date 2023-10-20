use std::fs::File;
use std::io::{BufReader};
mod Elements;
use crate::Elements::Element;

fn main() {
    let file = File::open("../assets/txt/elements.txt").unwrap();
    let reader = BufReader::new(file);
    
    let hydrogen = Element::new(
        "Hydrogen".to_string(),
        (10,12,14)
    );

    println!("{}", hydrogen);

/*
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split("\\-").collect();

        if parts.len() == 2 {
            let rgb: Vec<&str> = parts[1].split(',').collect();
            let r = rgb[0].parse::<u8>().unwrap();
            let g = rgb[1].parse::<u8>().unwrap();
            let b = rgb[2].parse::<u8>().unwrap();

            let element = Element::Element {
                name: parts[0].to_string(),
                rgb: (r, g, b)
            };

            elements.push(element);
        }
    }
    for element in &elements {
        println!("{}", element.name);
        println!(" - R:{}, G:{}, B:{}",
                 element.rgb.0,
                 element.rgb.1,
                 element.rgb.2,
        );
    }
    */
}


