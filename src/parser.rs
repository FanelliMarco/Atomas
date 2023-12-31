/*
* takes an image of the board, and outputs the current game state in some format
*/

use image::{ImageBuffer, Luma};
use image::{Rgba, RgbaImage};
use template_matching::{find_extremes, Image, MatchTemplateMethod, TemplateMatcher};

use crate::elements::{Data, Element};

pub fn find_elements_in_image(data: &Data) -> Vec<&Element> {
    let path = "C:/Obsidian/Rust/atomas/assets/jpg/board.jpg";
    let input_image = image::open(path).unwrap().to_luma32f();
    let mut found_elements = Vec::new();

    for element in &data.elements {
        let mut missing_templates = Vec::new();

        let template_image = match load_template_for_element(&element) {
            Some(img) => img,
            None => {
                missing_templates.push(element.name.clone());
                continue;
            }
        };

        let mut matcher = TemplateMatcher::new();

        matcher.match_template(
            &input_image,
            &template_image,
            MatchTemplateMethod::SumOfSquaredDifferences,
        );

        let result = matcher.wait_for_result().unwrap();

        let mut output = RgbaImage::from_pixel(
            input_image.width(),
            input_image.height(),
            Rgba([0, 0, 0, 255]),
        );

        let extremes = find_extremes(&result);

        if let location = extremes.min_value_location {
            let (x, y) = location;
            draw_rectangle(&mut output, x, y, 180, 180, element.rgb);
        }

        output
            .save("C:/Obsidian/Rust/atomas/assets/png/outputs/output.png")
            .unwrap();

        if let location = extremes.min_value_location {
            let (x, y) = location;
            found_elements.push(element);
        }
    }

    found_elements
}

fn load_template_for_element(element: &Element) -> Option<ImageBuffer<Luma<f32>, Vec<f32>>> {
    let path = format!(
        "C:/Obsidian/Rust/atomas/assets/png/{}.png",
        element.name.to_lowercase()
    );

    match image::open(&path) {
        Ok(img) => Some(img.to_luma32f()),
        Err(err) => {
            eprintln!("Unable to load image for {}: {}", element.name, err);
            None
        }
    }
}

fn find_best_match_location(result: &Image) -> Option<(u32, u32)> {
    let extremes = find_extremes(&result);
    Some(extremes.min_value_location)
}

fn draw_rectangle(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    rgb: (u8, u8, u8),
) {
    for i in 0..width {
        for j in 0..height {
            let current_x = x + i;
            let current_y = y + j;

            if current_x >= image.width() || current_y >= image.height() {
                continue;
            }

            image.put_pixel(current_x, current_y, Rgba([rgb.0, rgb.1, rgb.2, 255]));
        }
    }
}
