use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use enum_values::*;

type OptionalChar = Option<char>;

#[derive(ValuedEnum, EnumIter, Debug, PartialEq)]
#[value_type(OptionalChar)]
pub enum SpecialUnicodeChar {
    #[value(None)]
    Nothing,

    #[value(Some('\u{0009}'))]
    Tab,

    #[value(Some('\u{000A}'))]
    LineFeed,

    #[value(Some('\u{000D}'))]
    CarriageReturn,

    #[value(Some('\u{0020}'))]
    Space,

    #[value(Some('\u{00A0}'))]
    NoBreakSpace,

    #[value(Some('\u{1680}'))]
    OghamSpaceMark,

    #[value(Some('\u{2000}'))]
    EnQuad,

    #[value(Some('\u{2001}'))]
    EmQuad,

    #[value(Some('\u{2002}'))]
    EnSpace,

    #[value(Some('\u{2003}'))]
    EmSpace,

    #[value(Some('\u{2004}'))]
    ThreePerEmSpace,

    #[value(Some('\u{2005}'))]
    FourPerEmSpace,

    #[value(Some('\u{2006}'))]
    SixPerEmSpace,

    #[value(Some('\u{2007}'))]
    FigureSpace,

    #[value(Some('\u{2008}'))]
    PunctuationSpace,

    #[value(Some('\u{2009}'))]
    ThinSpace,

    #[value(Some('\u{200A}'))]
    HairSpace,

    #[value(Some('\u{2028}'))]
    LineSeparator,

    #[value(Some('\u{2029}'))]
    ParagraphSeparator,

    #[value(Some('\u{202F}'))]
    NarrowNoBreakSpace,

    #[value(Some('\u{205F}'))]
    MediumMathSpace,

    #[value(Some('\u{3000}'))]
    IdeographicSpace,

    #[value(Some('\u{2013}'))]
    EnDash,

    #[value(Some('\u{2014}'))]
    EmDash,

    #[value(Some('\u{2212}'))]
    Minus,

    #[value(Some('\u{002D}'))]
    Hyphen,
}
impl SpecialUnicodeChar {
    pub fn char(&self) -> Option<char> {
        self.value()
    }

    pub fn find_char(c: char) -> Option<Self> {
        Self::from_value(&Some(c))
    }

    pub fn all_non_breaking_chars() -> String {
        Self::iter()
            .filter(|m| !m.is_breaking())
            .fold(String::new(), |mut lhs, rhs| {
                match rhs.char() {
                    Some(c) => lhs.push(c),
                    None => (),
                }

                lhs
            })
    }

    pub fn is_breaking(&self) -> bool {
        match self {
            Self::NoBreakSpace | Self::NarrowNoBreakSpace | Self::FigureSpace | Self::Nothing => {
                true
            }
            _ => false,
        }
    }

    pub fn is_new_line(&self) -> bool {
        match self {
            Self::CarriageReturn | Self::LineFeed | Self::LineSeparator => true,
            _ => false,
        }
    }

    pub fn is_needed_end_of_line(&self) -> bool {
        match self {
            Self::EnDash | Self::EmDash | Self::Minus | Self::Hyphen => true,
            _ => false,
        }
    }

    pub fn append_to(&self, s: &mut String) {
        match self.char() {
            Some(c) => s.push(c),
            None => (),
        }
    }
}
