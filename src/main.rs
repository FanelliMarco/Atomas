// use std::fs::File;
// use std::io::{BufReader};
mod elements;
use crate::elements::Element;

fn main() {
    let hydrogen = Element::new("He", "Hydrogen".to_string(), (10, 12, 14));
    let plus = Element::new("+", "_Plus".to_string(), (201, 66, 62));

    println!("{}", hydrogen);
    println!("{}", plus);
}
