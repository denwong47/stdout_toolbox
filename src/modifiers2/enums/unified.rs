//! The primary enum, containing all the modifier enum structs as well as providing
//! a method to combine modifiers in sequence.

use std::{fmt, ops};

use super::super::{ANSIEscapeCode, ModifierError, Resetter, StringWrapper};
use super::{Background, Colour, Intensity};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Modifier {
    Intensity(Intensity),
    Colour(Colour),
    Background(Background),

    Combo(Vec<Self>),
}

/// Allow all Modifiers to have a resetter.
/// For all single types, just return its own resetter.
/// For `Combo`, returns another `Combo` with the resetters in reversed order.
impl Resetter for Modifier {
    fn resetter(&self, input: Option<&str>) -> Self {
        macro_rules! expand_variants {
            ($($variant:ident),+) => {
                match self {
                    $(Self::$variant(modifier) => Self::$variant(modifier.resetter(input)),)+
                    Self::Combo(modifiers) => {
                        // For `Combo`, sequentially format all the modifiers.
                        Self::Combo(
                            modifiers
                            .iter()
                            .rev()
                            .map(
                                | modifier | {
                                    modifier.resetter(input)
                                }
                            )
                            .collect()
                        )
                    },
                }
            };
        }

        expand_variants!(Intensity, Colour, Background)
    }
}

/// Allow two `Modifier` to be added together; so that when wrapping, the modifiers
/// will be applied in reversed sequence.
impl ops::Add for Modifier {
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

/// Allow the use Modifier enum variants directly in `println!()` or `format!()`.
impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! expand_variants {
            ($($variant:ident),+) => {
                match self {
                    $(Self::$variant(modifier) => modifier.fmt(f),)+
                    Self::Combo(modifiers) => {
                        // For `Combo`, sequentially format all the modifiers.
                        Result::from_iter(
                            modifiers.iter().map(
                                | modifier | modifier.fmt(f)
                            )
                        )
                    },
                }
            };
        }

        expand_variants!(Intensity, Colour, Background)
    }
}

/// Allows a `Modifier` to wrap a `str`, decorating it and resetting itself afterwards.
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

/// Try to parse an [`ANSIEscapeCode`] into a known [`Modifier`].
impl TryFrom<ANSIEscapeCode> for Modifier {
    type Error = ModifierError;
    fn try_from(value: ANSIEscapeCode) -> Result<Self, Self::Error> {
        macro_rules! expand_base_enums {
            ($(($variant:ident, $base_enum:ident, $code:pat, $end_char:literal)),+) => {
                match (value.code, value.end_char) {
                    $(
                        ($code, $end_char) => {
                            let modifier = $base_enum::try_from(value)?;

                            Ok(Self::$variant(modifier))
                        },
                    )+
                    (Some(code), _) => Err(
                        ModifierError::UnsupportedANSICode(code),
                    ),
                    (_, chr) => Err(
                        ModifierError::UnsupportedEndChar(chr),
                    ),
                }
            };
        }

        expand_base_enums!(
            (Intensity, Intensity, Some(1), 'm'),
            (Intensity, Intensity, Some(2), 'm'),
            (Intensity, Intensity, Some(22), 'm'),
            (Colour, Colour, Some(38), 'm'),
            (Colour, Colour, Some(39), 'm'),
            (Background, Background, Some(48), 'm'),
            (Background, Background, Some(49), 'm')
        )
    }
}
