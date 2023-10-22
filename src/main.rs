use std::fs::File;
use std::io::{BufReader};
mod Elements;
use crate::Elements::Element;

fn main() { 
    let hydrogen = Element::new(
        ['H','e'],
        "Hydrogen".to_string(),
        (10,12,14)
    );

    println!("{}", hydrogen);
}


