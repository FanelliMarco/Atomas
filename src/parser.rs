/*
* takes an image of the board, and outputs the current game state in some format
*/
/*
 * 2023-12-17
 * Choosed to use template matching
*/
use image::{GenericImageView, ImageBuffer, RgbImage};
use template_matching::{match_template, MatchTemplateMethod, TemplateMatcher};

use crate::elements::{Data, Element};

pub fn find_elements_in_image(data: &Data) -> Vec<&Element> {
    let path = "C:/Obsidian/Rust/atomas/assets/jpg/board.jpg";
    let input_image = image::open(path).unwrap().to_rgb8();

    let mut found_elements = Vec::new();

    for element in &data.elements {
        let template_image = create_template_for_element(element);

        let mut matcher = TemplateMatcher::new();
        matcher.match_template(&input_image, &template_image, MatchTemplateMethod::SumOfSquaredDifferences);
        let result = matcher.wait_for_result().unwrap();
        
        if let Some(location) = find_best_match_location(&result) {
            found_elements.push(element.clone());
        }
    }

    found_elements
}

fn create_template_for_element(element: &Element) -> RgbImage {
    let (width, height) = (30, 30);
    
    ImageBuffer::from_fn(width, height, |x, y| {
        if x > 5 && x < width-5 && y > 5 && y < height-5 {
            Rgb([element.rgb.0, element.rgb.1, element.rgb.2]) 
        } else {
            Rgb([255, 255, 255])
        }
    })
}

fn find_best_match_location(result: &image::ImageBuffer<f32, Vec<f32>>) -> Option<(u32, u32)> {
    template_matching::find_extremes(result)
        .first()
        .map(|(x, y)| (x as u32, y as u32))
}
