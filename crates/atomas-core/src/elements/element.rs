use serde::{Deserialize, Serialize};
use std::fmt;

use crate::elements::id::Id;
use crate::elements::types::ElementType;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Element<'a> {
    pub id: Id,
    pub element_type: ElementType,
    #[serde(borrow)]
    pub name: &'a str,
    pub rgb: (u8, u8, u8),
}

impl<'a> fmt::Display for Element<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.id {
            Id::Single(c) => write!(f, "{}", c)?,
            Id::Double(id) => write!(f, "{}", id.iter().collect::<String>())?,
            Id::Triple(id) => write!(f, "{}", id.iter().collect::<String>())?,
        }

        write!(
            f,
            " {} ({}, {}, {})",
            self.name, self.rgb.0, self.rgb.1, self.rgb.2
        )
    }
}

