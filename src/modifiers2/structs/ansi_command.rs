use lazy_static::lazy_static;

use regex::{Captures, Regex};

pub use super::super::{IntoANSIEscapeCode, ModifierError};

pub const DEFAULT_SEPARATOR: char = ';';

const BASE_CODE_PATTERN: &str = r#"\x1b\[(?P<codes>(?:\d+[;:])*\d+)(?P<end_char>[A-Za-z])"#;

lazy_static! {
    pub static ref ESCAPE_CODE_PATTERN: Regex = Regex::new(BASE_CODE_PATTERN).unwrap();
    pub static ref ESCAPE_CODE_START_PATTERN: Regex =
        Regex::new((String::from(r"^") + BASE_CODE_PATTERN).as_str()).unwrap();
}

#[derive(Debug, PartialEq)]
pub struct ANSIEscapeCode {
    code: u16,
    modifiers: Vec<i32>,
    pub sep: char,
}
#[allow(dead_code)]
impl ANSIEscapeCode {
    /// Creates a new ANSIEscapeCode instance with the default separator.
    pub fn new(code: u16, modifiers: Option<Vec<i32>>) -> Self {
        return Self {
            code: code,
            modifiers: modifiers.unwrap_or(Vec::new()),
            sep: DEFAULT_SEPARATOR,
        };
    }

    /// Replace the seperator in this instance.
    fn use_sep(mut self, sep: char) -> Self {
        self.sep = sep;
        self
    }

    /// Parse a &str beginning with `\x1b` into a Captures object.
    fn parse(text: &str) -> Result<Captures, ModifierError> {
        ESCAPE_CODE_START_PATTERN
            .captures(text)
            .ok_or(ModifierError::ValueIsNotAModifier(text.to_string()))
    }
}
impl ToString for ANSIEscapeCode {
    fn to_string(&self) -> String {
        let modifier_string = self.modifiers.iter().fold(String::new(), |mut lhs, rhs| {
            lhs.push(self.sep);
            lhs.push_str(&rhs.to_string());
            lhs
        });

        format!("\x1b[{}{}m", self.code, modifier_string,)
    }
}
impl<U> From<&U> for ANSIEscapeCode
where
    U: IntoANSIEscapeCode,
{
    /// Global implementation for anything that has the IntoANSIEscapeCode trait
    /// to have a From<U> and Into<ANSIEscapeCode> implemented.
    ///
    /// This is necessary because we want to have the conversion code to reside within
    /// the struct U, but if we just implement Into<ANSIEscapeCode> there,
    /// From<U> won't be implemented for ANSIEscapeCode then. Hence an intermediary
    /// trait is required.
    fn from(value: &U) -> Self {
        value.into_ansi_escape_code()
    }
}
