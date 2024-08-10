mod circularlist;
mod elements;
mod gamestate;
mod parser;

use crate::elements::Data;
use crate::gamestate::*;
use crate::parser::*;

fn main() {
    let path = "C:/Obsidian/Rust/atomas/assets/txt/elements.txt";
    let data = elements::Data::load(path);

    let board_image_path = "C:/Obsidian/Rust/atomas/assets/jpg/board.jpg";
    let game_state = detect_game_state(board_image_path, &data);

    println!("Detected Game State: {:?}", game_state);
}
