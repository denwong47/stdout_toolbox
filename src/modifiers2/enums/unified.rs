use std::ops::Add;

use super::super::StringWrapper;
use super::{Background, Colour, Intensity};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Modifier {
    Intensity(Intensity),
    Colour(Colour),
    Background(Background),

    Combo(Vec<Self>),
}
impl Add for Modifier {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut lhs_mods = match self {
            Self::Combo(mods) => mods,
            other => vec![other],
        };

        let mut rhs_mods = match rhs {
            Self::Combo(mods) => mods,
            other => vec![other],
        };

        lhs_mods.append(&mut rhs_mods);

        Self::Combo(lhs_mods)
    }
}
impl StringWrapper for Modifier {
    /// Enclose the text with the modifier.
    fn wraps(&self, text: &str) -> String {
        match self {
            Self::Combo(mods) => mods
                .iter()
                .rev()
                .fold(String::from(text), |text, modifier| modifier.wraps(&text)),
            Self::Intensity(modifier) => modifier.wraps(text),
            Self::Colour(modifier) => modifier.wraps(text),
            Self::Background(modifier) => modifier.wraps(text),
        }
    }
}
