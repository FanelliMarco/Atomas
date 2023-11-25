use std::cmp::Ordering;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct Node {
    level: u8,
    source_colors: Vec<Color>,
    children: Vec<Option<Box<Node>>>,
    color_index: Option<usize>,
}

impl Node {
    fn new(level: u8, source_colors: &[Color]) -> Self {
        Node {
            level,
            source_colors: source_colors.to_vec(),
            children: vec![None; 8],
            color_index: None,
        }
    }

    fn add_color(&mut self, color_index: usize) {
        if self.level == 7 {
            self.color_index = Some(color_index);
        } else {
            let index = Self::get_oct_index(&self.source_colors[color_index], self.level);

            if self.children[index].is_none() {
                self.children[index] =
                    Some(Box::new(Node::new(self.level + 1, &self.source_colors)));
            }

            self.children[index]
                .as_mut()
                .unwrap()
                .add_color(color_index);
        }
    }

    fn get_nearest_color_index(&self, color: &Color) -> (usize, u32) {
        if self.level == 7 {
            let index = self.color_index.unwrap();
            let distance = Self::get_distance(&self.source_colors[index], color);
            (index, distance)
        } else {
            let index = Self::get_oct_index(color, self.level);

            if let Some(child) = &self.children[index] {
                child.get_nearest_color_index(color)
            } else {
                let mut min_distance = u32::MAX;
                let mut min_color = 0;

                for child in &self.children {
                    if let Some(child) = child {
                        let (child_color, child_distance) = child.get_nearest_color_index(color);

                        if child_distance < min_distance {
                            min_distance = child_distance;
                            min_color = child_color;
                        }
                    }
                }

                (min_color, min_distance)
            }
        }
    }

    fn get_oct_index(color: &Color, level: u8) -> usize {
        let mask = [128, 64, 32, 16, 8, 4, 2, 1];
        let shift = 7 - level;

        ((color.r & mask[level as usize]) >> shift)
            | ((color.g & mask[level as usize]) >> (shift - 1))
            | ((color.b & mask[level as usize]) >> (shift - 2))
    }

    fn get_distance(c1: &Color, c2: &Color) -> u32 {
        let dr = (c1.r as i32 - c2.r as i32).abs() as u32;
        let dg = (c1.g as i32 - c2.g as i32).abs() as u32;
        let db = (c1.b as i32 - c2.b as i32).abs() as u32;

        dr * dr + dg * dg + db * db
    }
}

struct ColorFinder {
    root: Node,
}

impl ColorFinder {
    fn new(source_colors: &[Color]) -> Self {
        let root = Node::new(0, source_colors);

        for (i, color) in source_colors.iter().enumerate() {
            root.add_color(i);
        }

        ColorFinder { root }
    }

    fn get_nearest_color_index(&self, color: &Color) -> usize {
        let (index, _) = self.root.get_nearest_color_index(color);
        index
    }
}
