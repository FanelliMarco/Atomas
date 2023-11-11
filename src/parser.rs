/*
 * takes an image of the board, and outputs the current game state in some format
*/
use crate::elements::{Data, Element};
use image::{self, GenericImageView, Rgba};

pub fn match_rgb(rgb: Rgba, data: &Data) -> bool {
    //unimplemented
    false
}

pub fn load_pixel() {
    let path = "C:/Obsidian/Rust/atomas/assets/jpg/merge.jpg";
    let img = image::open(path).expect("File not found");

    for pixel in img.pixels() {
        println!("{:?}", pixel);
    }
}
