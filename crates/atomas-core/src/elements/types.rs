use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementType {
    /// Special atoms (-10 to -1)
    Special(SpecialAtom),
    /// Periodic table elements (1 to 118)
    Periodic(u8),
    /// Custom elements (119 to 128)
    Custom(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialAtom {
    Plus = -1,
    Minus = -2,
    DarkPlus = -3,
    Neutrino = -4,
    Luxon = -5,
    // Reserve space for future special atoms (-6 to -10)
}

impl ElementType {
    pub fn to_numeric(&self) -> i16 {
        match self {
            ElementType::Special(special) => *special as i16,
            ElementType::Periodic(num) => *num as i16,
            ElementType::Custom(num) => *num as i16,
        }
    }

    pub fn from_numeric(value: i16) -> Option<Self> {
        match value {
            -10..=-1 => Some(ElementType::Special(
                // Convert negative number to SpecialAtom enum
                match value {
                    -1 => SpecialAtom::Plus,
                    -2 => SpecialAtom::Minus,
                    -3 => SpecialAtom::DarkPlus,
                    -4 => SpecialAtom::Neutrino,
                    -5 => SpecialAtom::Luxon,
                    _ => return None, // Reserved for future special atoms
                }
            )),
            1..=118 => Some(ElementType::Periodic(value as u8)),
            119..=128 => Some(ElementType::Custom(value as u8)),
            _ => None,
        }
    }

    pub fn is_special(&self) -> bool {
        matches!(self, ElementType::Special(_))
    }

    pub fn is_periodic(&self) -> bool {
        matches!(self, ElementType::Periodic(_))
    }

    pub fn is_custom(&self) -> bool {
        matches!(self, ElementType::Custom(_))
    }
}