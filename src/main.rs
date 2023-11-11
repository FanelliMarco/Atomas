// use std::fs::File;
// use std::io::{BufReader};
mod elements;
use crate::elements::{Data, Element};

fn main() {
    let path = "C:/Obsidian/Rust/atomas/assets/txt/elements.txt";
    let data = elements::Data::load(path);

    for element in &data.elements {
        println!("{}", element);
    }
}
