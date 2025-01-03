use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Id {
    Single(char),
    Double([char; 2]),
    Triple([char; 3]),
}

impl Id {
    pub fn from_chars(chars: &[char]) -> Self {
        match chars.len() {
            1 => Id::Single(chars[0]),
            2 => Id::Double([chars[0], chars[1]]),
            3 => Id::Triple([chars[0], chars[1], chars[2]]),
            _ => panic!("Invalid id length"),
        }
    }
}