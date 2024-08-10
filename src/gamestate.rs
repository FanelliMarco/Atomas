use crate::circularlist::CircularList;
use crate::elements::Element;
use rand::Rng;
use std::fmt;

#[derive(Clone)]
pub struct GameState {
    pub ring: CircularList<Element>,
    pub player_atom: Element,
    pub max_value: i32,
    pub score: i32,
}

impl GameState {
    pub fn new(elements: &[Element]) -> Self {
        let mut rng = rand::thread_rng();
        let player_atom = elements[rng.gen_range(0..elements.len())].clone(); // why random?

        GameState {
            ring: CircularList::new(),
            player_atom,
            max_value: 1,
            score: 0,
        }
    }

    pub fn add_atom_to_ring(&mut self, index: usize) {
        self.ring.insert(self.player_atom.clone(), index);
        self.update_max_value();
        self.generate_new_player_atom();
    }

    pub fn update_max_value(&mut self) {
        // Implement logic to update max_value based on the ring
    }

    pub fn generate_new_player_atom(&mut self) {
        // Implement logic to generate a new player atom
    }

    pub fn check_fusion(&mut self, _index: usize) -> bool {
        // Implement fusion logic
        false
    }

    // Implement other game logic methods
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GameState {{")?;
        writeln!(f, "  ring: [")?;
        for (i, element) in self.ring.iter().enumerate().take(12) {
            // Limit to first 12 elements
            writeln!(f, "    {}: {:?},", i, element)?;
        }
        if self.ring.len() > 12 {
            writeln!(f, "    ... ({} more elements)", self.ring.len() - 12)?;
        }
        writeln!(f, "  ],")?;
        writeln!(f, "  player_atom: {:?},", self.player_atom)?;
        writeln!(f, "  max_value: {},", self.max_value)?;
        writeln!(f, "  score: {}", self.score)?;
        write!(f, "}}")
    }
}
