use lazy_static::lazy_static;

use regex::{Captures, Regex};

pub use super::super::{IntoANSIEscapeCode, ModifierError};

pub const DEFAULT_SEPARATOR: char = ';';

const BASE_CODE_PATTERN: &str = r#"\x1b\[(?P<codes>(?:\d+[;:])*\d+)(?P<end_char>[A-Za-z])"#;

lazy_static! {
    pub static ref ESCAPE_CODE_PATTERN: Regex = Regex::new(BASE_CODE_PATTERN).unwrap();
    pub static ref ESCAPE_CODE_START_PATTERN: Regex =
        Regex::new((String::from(r"^") + BASE_CODE_PATTERN).as_str()).unwrap();
    pub static ref SEP_PATTERN: Regex = Regex::new(r"[:;]").unwrap();
}

/// A basic dataclass of a deconstructed \x1b[00;00;..m structure.
#[derive(Debug, PartialEq)]
pub struct ANSIEscapeCode {
    pub code: u16,
    pub modifiers: Vec<i32>,
    pub sep: char,
    pub end_char: char,
}
#[allow(dead_code)]
impl ANSIEscapeCode {
    /// Creates a new ANSIEscapeCode instance with the default separator.
    pub fn new(code: u16, modifiers: Option<Vec<i32>>, end_char: char) -> Self {
        return Self {
            code,
            modifiers: modifiers.unwrap_or(Vec::new()),
            sep: DEFAULT_SEPARATOR,
            end_char,
        };
    }

    /// Replace the seperator in this instance.
    pub fn use_sep(mut self, sep: char) -> Self {
        self.sep = sep;
        self
    }

    /// Parse a &str beginning with `\x1b` into a Captures object.
    pub fn parse(text: &str) -> Result<Captures, ModifierError> {
        ESCAPE_CODE_START_PATTERN
            .captures(text)
            .ok_or(ModifierError::ValueIsNotAModifier(text.to_string()))
    }
}
impl TryFrom<&str> for ANSIEscapeCode {
    type Error = ModifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let captures = Self::parse(value)?;

        let codes: Vec<i32> = {
            captures
                .name("codes")
                .ok_or(ModifierError::BadRegexPattern)
                .and_then(|codes_match| {
                    Result::from_iter(SEP_PATTERN.split(codes_match.as_str()).map(|code| {
                        code.parse::<i32>().or(
                            // At least one of the code is not u8 parsable
                            Err(ModifierError::ValueNotRecognised(
                                stringify!($enum_name).to_string(),
                                code.to_string(),
                                String::from(
                                    "At least one of the provided codes are not i16 parsable.",
                                ),
                            )),
                        )
                    }))
                })
        }?;

        let code: u16 = codes
            .first()
            .ok_or(
                // Codes are empty
                ModifierError::ValueNotRecognised(
                    stringify!($enum_name).to_string(),
                    format!("{:?}", codes),
                    String::from("No codes provided."),
                ),
            )
            .and_then(|code| {
                (*code).try_into().or(
                    // ANSI Code not within u8
                    Err(ModifierError::ValueIsNotAModifier(format!("{:?}", value))),
                )
            })?;

        let modifiers = Some(Vec::from(&codes[1..]));

        let end_char = captures
            .name("end_char")
            .ok_or(ModifierError::BadRegexPattern)?
            .as_str()
            .chars()
            .next()
            .unwrap();

        Ok(Self::new(code, modifiers, end_char))
    }
}
impl ToString for ANSIEscapeCode {
    fn to_string(&self) -> String {
        let modifier_string = self.modifiers.iter().fold(String::new(), |mut lhs, rhs| {
            lhs.push(self.sep);
            lhs.push_str(&rhs.to_string());
            lhs
        });

        format!("\x1b[{}{}{}", self.code, modifier_string, self.end_char)
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
