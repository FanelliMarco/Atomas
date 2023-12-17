/*
* takes an image of the board, and outputs the current game state in some format
*/

use image::{DynamicImage, ImageBuffer, Luma};
use template_matching::{TemplateMatcher, MatchTemplateMethod, find_extremes};

use crate::elements::{Data, Element};

pub fn find_elements_in_image(data: &Data) -> Vec<&Element> {
    let path = "C:/Obsidian/Rust/atomas/assets/jpg/board.jpg";
    let input_image = image::open(path).unwrap();

    let mut found_elements = Vec::new();

    for element in &data.elements {
        let template_image = load_template_for_element(&element);
        let mut matcher = TemplateMatcher::new();
        
        let dynamic_template_image: DynamicImage = template_image.into();
        matcher.match_template(&input_image, &dynamic_template_image, MatchTemplateMethod::SumOfSquaredDifferences);
        
        let result = matcher.wait_for_result().unwrap();

        if let Some(_location) = find_best_match_location(&result) {
            found_elements.push(element);
        }
    }

    found_elements
}

fn load_template_for_element(element: &Element) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let path = format!("C:/Obsidian/Rust/atomas/assets/png/{}.png", element.name.to_lowercase());
    image::open(&path).unwrap().to_luma8()
}

fn find_best_match_location(result: &ImageBuffer<Luma<f32>, Vec<f32>>) -> Option<(u32, u32)> {
    find_extremes(result)
        .min()
        .map(|(x, y)| (x as u32, y as u32))
}

