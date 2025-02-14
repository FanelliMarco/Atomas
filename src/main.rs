use atomas_core::{
    elements::{Data, Element, ElementType, SpecialAtom},
    ring::{CircularList, AdjMatrix},
};

mod gamestate;
mod parser;

fn main() {
    let path = "C:/Obsidian/Rust/atomas/assets/txt/elements.txt";
    let data = Data::load(path); 

    let board_image_path = "C:/Obsidian/Rust/atomas/assets/jpg/board.jpg";
    let game_state = parser::detect_game_state(&board_image_path, &data);

    println!("Detected Game State: {:?}", game_state);
}
