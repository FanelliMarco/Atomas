use crate::elements::Element;
use crate::ring::CircularList;
use std::fmt;


#[derive(Debug, Clone)]
pub struct AdjMatrix<'a> {
    size: usize,
    matrix: Vec<Vec<bool>>,
    elements: Vec<Option<Element<'a>>>,
    center_element: Option<Element<'a>>,
}

impl<'a> AdjMatrix<'a> {
    /// Creates a new adjacency matrix with a fixed maximum size
    pub fn new(max_size: usize) -> Self {
        AdjMatrix {
            size: max_size,
            matrix: vec![vec![false; max_size]; max_size],
            elements: vec![None; max_size],
            center_element: None,
        }
    }

    /// Updates the matrix from the current game state
    pub fn update_from_ring(&mut self, ring: &CircularList<Element<'a>>, center: &Element<'a>) {
        // Clear existing state
        self.matrix = vec![vec![false; self.size]; self.size];
        self.elements = vec![None; self.size];
        
        // Store elements and build adjacency relationships
        for (i, element) in ring.iter().enumerate() {
            if i >= self.size {
                break;
            }
            
            self.elements[i] = Some(element.clone());
            
            // Connect to previous element (circular)
            if i > 0 {
                self.matrix[i][i-1] = true;
                self.matrix[i-1][i] = true;
            }

            // Connect center element to all ring elements
            // Using the last index (self.size - 1) for the center element
            self.matrix[i][self.size - 1] = true;
            self.matrix[self.size - 1][i] = true;
        }
        
        // Connect first and last elements to complete the circle
        if ring.len() > 1 {
            let last_idx = ring.len().min(self.size - 1); // Adjust for center element
            self.matrix[0][last_idx - 1] = true;
            self.matrix[last_idx - 1][0] = true;
        }

        // Store center element in the last position
        self.elements[self.size - 1] = Some(center.clone());
        self.center_element = Some(center.clone());
    }

    /// Gets the adjacency value between two positions
    pub fn get_adjacency(&self, i: usize, j: usize) -> bool {
        i < self.size && j < self.size && self.matrix[i][j]
    }

    // ... rest of the implementation remains the same ...
}

impl<'a> fmt::Display for AdjMatrix<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AdjMatrix ({}x{}):", self.size, self.size)?;
        
        // Print matrix
        for i in 0..self.size {
            write!(f, "    [")?;
            for j in 0..self.size {
                write!(f, "{:>4}", if self.matrix[i][j] { "1" } else { "0" })?;
            }
            writeln!(f, " ]")?;
        }
        
        // ... rest of display implementation remains the same ...
        Ok(())
    }
}
