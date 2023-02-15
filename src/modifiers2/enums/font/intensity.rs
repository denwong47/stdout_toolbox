use std::default;
use strum_macros::EnumIter;

use enum_index::*;

use super::super::super::{ANSIEscapeCode, IntoANSIEscapeCode, ModifierError, Resetter};

/// Intensity modifier
#[derive(Debug, EnumIter, EnumIndex, PartialEq)]
#[index_type(u16)]
pub enum Intensity {
    #[index(22)]
    Normal,

    #[index(1)]
    Bold,

    #[index(2)]
    Faint,
}
impl default::Default for Intensity {
    fn default() -> Self {
        Self::Normal
    }
}

impl Resetter for Intensity {
    #[allow(unused_variables)]
    fn resetter(&self, input: Option<&str>) -> Self {
        Self::default()
    }
}

impl IntoANSIEscapeCode for Intensity {
    fn into_ansi_escape_code(&self) -> ANSIEscapeCode {
        ANSIEscapeCode::new(self.index(), None, 'm')
    }
}

impl ToString for Intensity {
    /// Transform the object into ANSIEscapeCode, then use that to generate
    /// a String.
    ///
    /// This also implements Display.
    fn to_string(&self) -> String {
        let ansi: ANSIEscapeCode = self.into();

        ansi.to_string()
    }
}

impl TryFrom<ANSIEscapeCode> for Intensity {
    type Error = ModifierError;

    fn try_from(value: ANSIEscapeCode) -> Result<Self, Self::Error> {
        if value.end_char != 'm' {
            return Err(ModifierError::UnexpectedEndCharacter(
                stringify!($enum_name).to_string(),
                value.end_char.to_string(),
            ));
        }

        if value.modifiers.len() > 0 {
            return Err(ModifierError::ValueNotRecognised(
                stringify!($enum_name).to_string(),
                format!("{:?}:{:?}", value.code, value.modifiers),
                String::from("This code does not accept modifiers."),
            ));
        }

        Self::try_from(&value.code).or(Err(ModifierError::MismatchedANSICode(
            stringify!($enum_name).to_string(),
            value.code,
            1,
        )))
    }
}
