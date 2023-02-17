//! Parsing of [`str`] into a dataclass, [`ANSIEscapeCode`].
//!
//! Example
//! -------
//!
//! ```rust
//! use stdout_toolbox::{ANSIEscapeCode, DEFAULT_SEPARATOR};
//!
//! let parsed: ANSIEscapeCode = "\x1b[30m".try_into().unwrap();
//! assert_eq!(
//!     parsed,
//!     ANSIEscapeCode {
//!         code: Some(30),
//!         modifiers: Vec::new(),
//!         sep: DEFAULT_SEPARATOR,
//!         end_char: 'm',
//!     }
//! );
//!
//! let parsed: ANSIEscapeCode = "\x1b[20;8H".try_into().unwrap();
//! assert_eq!(
//!     parsed,
//!     ANSIEscapeCode {
//!         code: None,
//!         modifiers: vec![20,8],
//!         sep: DEFAULT_SEPARATOR,
//!         end_char: 'H',
//!     }
//! );
//!
//! let parsed: ANSIEscapeCode = "\x1b[38:5:255m".try_into().unwrap();
//! assert_eq!(
//!     parsed,
//!     ANSIEscapeCode {
//!         code: Some(38),
//!         modifiers: vec![5,255],
//!         sep: DEFAULT_SEPARATOR,
//!         end_char: 'm',
//!     }
//! );
//! ```
use std::fmt;

use lazy_static::lazy_static;

use regex::{Captures, Regex};

pub use super::super::{IntoANSIEscapeCode, ModifierError};

/// Default `sep` to use for [`ANSIEscapeCode`].
pub const DEFAULT_SEPARATOR: char = ';';

// In principle, there is one code that we won't match here, which is `\x1b[H`.
// However this "origin position" code can be easily expressed as `\x1b[0;0H`.
const BASE_CODE_PATTERN: &str = r#"\x1b\[(?P<codes>(?:\-?\d+[;:])*\-?\d+)(?P<end_char>[A-Za-z])"#;

lazy_static! {
    pub static ref ESCAPE_CODE_PATTERN: Regex = Regex::new(BASE_CODE_PATTERN).unwrap();
    pub static ref ESCAPE_CODE_START_PATTERN: Regex =
        Regex::new((String::from(r"^") + BASE_CODE_PATTERN).as_str()).unwrap();
    pub static ref SEP_PATTERN: Regex = Regex::new(r"[:;]").unwrap();
}

/// A basic dataclass of a deconstructed `\x1b[00;00;..m` structure.
///
/// This dataclass simply represents a syntax valid ANSI Escape pattern; it does not
/// necessarily guarantee that the pattern is meaningful or legal for the specific
/// command.
///
#[derive(Debug, PartialEq)]
pub struct ANSIEscapeCode {
    /// Command Code.
    ///
    /// If `end_char` is `'m'`, then this will be a [`Some<u16>`] containing the
    /// first code after `\x1b[`. For example, for the colour code of `\x1b[30m`, `code`
    /// will be [`Some(30_u16)`]. If `end_char` is not `'m'`, `code` shall be [`None`].
    pub code: Option<u16>,

    /// Modifier codes.
    ///
    /// a [`Vec`] of [`i32`] collecting all the modifier codes that follows
    /// `code` above. For commands like `\x1b[2A` to move cursor up by `2` rows,
    /// `modifiers` will be `vec![2]`; for colour commands like the above `\x1b[30m`
    pub modifiers: Vec<i32>,

    /// Separator char.
    ///
    /// Must be either `:` or `;` to be valid.
    ///
    /// This is not currently in use when parsing; any code that is parsed will use
    /// `DEFAULT_SEPARATOR` instead; however if this is set, then `to_string` will
    /// build the `String` wtih the separator.
    pub sep: char,

    /// The trailing character of the sequence.
    ///
    /// Typically `'m'` for most modern commands; but can be any `char` within
    /// `[A-Za-z]` for other codes such as cursor movement.
    ///
    /// Mandatory - without this `char`, the pattern cannot be terminated.
    pub end_char: char,
}
#[allow(dead_code)]
impl ANSIEscapeCode {
    /// Creates a new [`ANSIEscapeCode`] instance with the default separator.
    pub fn new(code: Option<u16>, modifiers: Option<Vec<i32>>, end_char: char) -> Self {
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

    /// Parse a [`str`] beginning with `\x1b` into a [`regex::Captures`] object.
    pub fn parse(text: &str) -> Result<Captures, ModifierError> {
        ESCAPE_CODE_START_PATTERN
            .captures(text)
            .ok_or(ModifierError::ValueIsNotAModifier(
                text.to_string(),
                String::from("Unmatchable pattern."),
            ))
    }
}
impl TryFrom<&str> for ANSIEscapeCode {
    type Error = ModifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let captures = Self::parse(value)?;

        // Just use TryFrom<Captures<'t>> to do the job for us.
        Self::try_from(captures)
    }
}
impl<'t> TryFrom<Captures<'t>> for ANSIEscapeCode {
    type Error = ModifierError;

    fn try_from(value: Captures) -> Result<Self, Self::Error> {
        let captures = value; // Rename value: change owner

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

        let end_char = captures
            .name("end_char")
            .ok_or(ModifierError::BadRegexPattern)?
            .as_str()
            .chars()
            .next()
            .unwrap();

        let (code, modifiers) = match end_char {
            'm' => {
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
                            Err(ModifierError::ValueIsNotAModifier(
                                captures
                                    .get(0)
                                    .map(|m| m.as_str())
                                    .unwrap_or("(unparsable match)")
                                    .to_string(),
                                String::from("Code is not a valid u16 integer."),
                            )),
                        )
                    })?;

                let modifiers = Some(Vec::from(&codes[1..]));

                (Some(code), modifiers)
            }
            _ => (None, Some(codes)),
        };

        Ok(Self::new(code, modifiers, end_char))
    }
}
impl fmt::Display for ANSIEscapeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // If there is a code, use it to start an iterator.
        // Otherwise, get an empty Vec to start it instead.
        let code = if let Some(code) = self.code {
            vec![code as i32]
        } else {
            Vec::new()
        };

        let modifier_string =
            code.iter()
                .chain(self.modifiers.iter())
                .fold(String::new(), |mut lhs, rhs| {
                    lhs.push(self.sep);
                    lhs.push_str(&rhs.to_string());
                    lhs
                });

        write!(f, "\x1b[{}{}", modifier_string, self.end_char)
    }
}
impl<U> From<&U> for ANSIEscapeCode
where
    U: IntoANSIEscapeCode,
{
    /// Global implementation for anything that has the IntoANSIEscapeCode trait
    /// to have a `From<U>` and `Into<ANSIEscapeCode>` implemented.
    ///
    /// This is necessary because we want to have the conversion code to reside within
    /// the struct U, but if we just implement `Into<ANSIEscapeCode>` there,
    /// `From<U>` won't be implemented for ANSIEscapeCode then. Hence an intermediary
    /// trait is required.
    fn from(value: &U) -> Self {
        value.into_ansi_escape_code()
    }
}
