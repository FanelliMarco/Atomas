use crate::circularlist::CircularList;
use crate::elements::Element;
use rand::Rng;
use std::fmt;

#[derive(Clone)]
pub struct GameState<'a> {
    pub ring: CircularList<Element<'a>>,
    pub player_atom: Element<'a>,
    pub max_value: i32,
    pub score: i32,
}

impl<'a> GameState<'a> {
    pub fn new(elements: &[Element<'a>]) -> Self {
        let mut rng = rand::thread_rng();
        let player_atom = elements[rng.gen_range(0..elements.len())].clone();

        GameState {
            ring: CircularList::new(),
            player_atom,
            max_value: 1,
            score: 0,
        }
    }
}

impl<'a> fmt::Debug for GameState<'a> {
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
