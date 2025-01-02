use crate::circularlist::CircularList;
use crate::elements::{Data, Element};
use crate::gamestate::GameState;
use image::{ImageBuffer, Luma, Rgba, RgbaImage};
use template_matching::{find_extremes, Image, MatchTemplateMethod, TemplateMatcher};
use crate::adjmatrix::AdjMatrix;

pub fn detect_game_state<'a>(input_image_path: &str, data: &Data<'a>) -> GameState<'a> {
    let input_image = image::open(input_image_path).unwrap().to_luma32f();
    let mut ring = CircularList::new();
    let mut player_atom: Option<&Element<'a>> = None;
    let mut max_value = 1;
    let mut score = 0;

    // Initialize the output image for visualization
    let mut output = RgbaImage::from_pixel(
        input_image.width(),
        input_image.height(),
        Rgba([0, 0, 0, 255]),
    );

    for element in &data.elements {
        let template_image = match load_template_for_element(element) {
            Some(img) => img,
            None => {
                eprintln!("Missing template for element: {}", element.name);
                continue;
            }
        };

        let mut matcher = TemplateMatcher::new();
        let input_image_tm = Image::new(
            input_image.as_raw().to_vec(),
            input_image.width(),
            input_image.height(),
        );
        let template_image_tm = Image::new(
            template_image.as_raw().to_vec(),
            template_image.width(),
            template_image.height(),
        );
        matcher.match_template(
            input_image_tm,
            template_image_tm,
            MatchTemplateMethod::SumOfSquaredDifferences,
        );

        let result = matcher.wait_for_result().unwrap();
        let extremes = find_extremes(&result);

        let (x, y) = extremes.min_value_location;
        draw_rectangle(&mut output, x, y, 180, 180, element.rgb);

        // Determine if this is the player atom or part of the ring
        if is_player_atom_position(x, y, input_image.width(), input_image.height()) {
            player_atom = Some(element);
        } else {
            let index = calculate_ring_index(x, y, input_image.width(), input_image.height());
            ring.insert(element.clone(), index);
        }

        // Update max_value if necessary
        max_value = max_value.max(element_to_value(element));
    }

    // Save the output image for visualization
    output
        .save("C:/Obsidian/Rust/atomas/assets/png/outputs/detected_state.png")
        .unwrap();

    // Create and return the GameState
    let mut game_state = GameState {
        ring,
        player_atom: player_atom
            .cloned()
            .unwrap_or_else(|| data.elements[0].clone()),
        max_value,
        score,
        adj_matrix: AdjMatrix::new(12),
    };

    // Update the adjacency matrix
    game_state.update_adjacency();

    game_state
}
fn load_template_for_element<'a>(
    element: &Element<'a>,
) -> Option<ImageBuffer<Luma<f32>, Vec<f32>>> {
    let path = format!("C:/Obsidian/Rust/atomas/assets/png/{}.png", element.name);

    match image::open(&path) {
        Ok(img) => Some(img.to_luma32f()),
        Err(err) => {
            eprintln!("Unable to load image for {}: {}", element.name, err);
            None
        }
    }
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

fn is_player_atom_position(x: u32, y: u32, width: u32, height: u32) -> bool {
    let center_x = width / 2;
    let center_y = height / 2;

    (x as i32 - center_x as i32).abs() < 50 && (y as i32 - center_y as i32).abs() < 50
}

fn calculate_ring_index(x: u32, y: u32, width: u32, height: u32) -> usize {
    let center_x = width / 2;
    let center_y = height / 2;
    let angle = ((y as f32 - center_y as f32).atan2(x as f32 - center_x as f32)
        + std::f32::consts::PI)
        % (2.0 * std::f32::consts::PI);
    (angle / (2.0 * std::f32::consts::PI) * 12.0).round() as usize % 12
}

fn element_to_value(element: &Element) -> i32 {
    element.name.chars().next().map(|c| c as i32).unwrap_or(0)
}
