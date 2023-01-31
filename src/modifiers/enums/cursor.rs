use std::fmt::Display;
use strum_macros::EnumIter;

use super::super::{HasResetter, HasValue};

#[allow(dead_code)]
#[derive(EnumIter, Debug, PartialEq)]
pub enum MoveCursor {
    Up(i16),
    Down(i16),
    Right(i16),
    Left(i16),
    Origin,
    Absolute(i16, i16),
}
#[allow(dead_code)]
impl HasValue<String> for MoveCursor {
    fn value(&self) -> String {
        match self {
            // By default, shell treat commands with 0 magnitude as 1.
            // There are many reasons why 0 would mean actual 0, so we give an empty string instead.
            Self::Up(0) | Self::Down(0) | Self::Left(0) | Self::Right(0) => String::new(),
            _ => {
                let (command_char, magnitude) = match self {
                    Self::Up(magnitude) => ("A", magnitude.to_string()),
                    Self::Down(magnitude) => ("B", magnitude.to_string()),
                    Self::Right(magnitude) => ("C", magnitude.to_string()),
                    Self::Left(magnitude) => ("D", magnitude.to_string()),
                    Self::Origin => ("H", String::new()),
                    Self::Absolute(x, y) => ("H", format!("{};{}", y, x)),
                };

                format!("\x1b[{}{}", magnitude, command_char,)
            }
        }
    }
}
#[allow(dead_code)]
impl HasResetter for MoveCursor {
    fn resetter(&self) -> Self {
        match self {
            Self::Up(magnitude) => Self::Down(*magnitude),
            Self::Down(magnitude) => Self::Up(*magnitude),
            Self::Right(magnitude) => Self::Left(*magnitude),
            Self::Left(magnitude) => Self::Right(*magnitude),
            Self::Origin => Self::Origin,
            Self::Absolute(_, _) => Self::Origin,
        }
    }
}
impl Display for MoveCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
