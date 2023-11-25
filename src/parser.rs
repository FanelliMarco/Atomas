/*
* takes an image of the board, and outputs the current game state in some format
*/

use image::GenericImageView;
use palette::color_difference::EuclideanDistance;
use palette::Srgb;
use palette::Srgb::D65;
use palette::{FromColor, Lab};
use std::collections::HashSet;

use crate::elements::{Data, Element};

fn rgb_to_lab(rgb: (u8, u8, u8)) -> Lab<Srgb, f32> {
    let rgb = Srgb::new(
        rgb.0 as f32 / 255.0,
        rgb.1 as f32 / 255.0,
        rgb.2 as f32 / 255.0,
    );

    let rgb = D65.into_linear().apply_gamma(rgb);

    let lab = Lab::from_color(rgb);

    lab
}

fn distance_lab(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f32 {
    let lab1 = rgb_to_lab(rgb1);
    let lab2 = rgb_to_lab(rgb2);

    lab1.distance(lab2)
}

pub fn find_elements_in_image(data: &Data) -> HashSet<&Element> {
    let path = "C:/Obsidian/Rust/atomas/assets/jpg/merge.jpg";

    let image = image::open(path).expect("Failed to open image");

    let mut elements_set = HashSet::new();

    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y);
            let pixel_rgb = (pixel[0], pixel[1], pixel[2]);

            for element in &data.elements {
                let element_rgb = element.rgb;
                let distance = distance_lab(pixel_rgb, element_rgb);

                // You can adjust the threshold value based on your needs
                if distance < 10.0 {
                    elements_set.insert(element);
                }
            }
        }
    }

    elements_set
}
