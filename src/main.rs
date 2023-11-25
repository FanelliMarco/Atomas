// use std::fs::File;
// use std::io::{BufReader};
mod elements;
mod parser;

#[allow(unused_imports)]
use crate::elements::{Data, Element};
use crate::parser::*;

fn main() {
    let path = "C:/Obsidian/Rust/atomas/assets/txt/elements.txt";
    let data = elements::Data::load(path);

    let mut elements_set = find_elements_in_image(&data);

    for element in elements_set {
        println!("{}", element);
    }
}
