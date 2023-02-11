use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use enum_index::*;

type OptionalChar = Option<char>;

#[derive(EnumIndex, EnumIter, Debug, PartialEq)]
#[index_type(OptionalChar)]
pub enum SpecialUnicodeChar {
    #[index(None)]
    Nothing,

    #[index(Some('\u{0009}'))]
    Tab,

    #[index(Some('\u{000A}'))]
    LineFeed,

    #[index(Some('\u{000D}'))]
    CarriageReturn,

    #[index(Some('\u{0020}'))]
    Space,

    #[index(Some('\u{00A0}'))]
    NoBreakSpace,

    #[index(Some('\u{1680}'))]
    OghamSpaceMark,

    #[index(Some('\u{2000}'))]
    EnQuad,

    #[index(Some('\u{2001}'))]
    EmQuad,

    #[index(Some('\u{2002}'))]
    EnSpace,

    #[index(Some('\u{2003}'))]
    EmSpace,

    #[index(Some('\u{2004}'))]
    ThreePerEmSpace,

    #[index(Some('\u{2005}'))]
    FourPerEmSpace,

    #[index(Some('\u{2006}'))]
    SixPerEmSpace,

    #[index(Some('\u{2007}'))]
    FigureSpace,

    #[index(Some('\u{2008}'))]
    PunctuationSpace,

    #[index(Some('\u{2009}'))]
    ThinSpace,

    #[index(Some('\u{200A}'))]
    HairSpace,

    #[index(Some('\u{2028}'))]
    LineSeparator,

    #[index(Some('\u{2029}'))]
    ParagraphSeparator,

    #[index(Some('\u{202F}'))]
    NarrowNoBreakSpace,

    #[index(Some('\u{205F}'))]
    MediumMathSpace,

    #[index(Some('\u{3000}'))]
    IdeographicSpace,

    #[index(Some('\u{2013}'))]
    EnDash,

    #[index(Some('\u{2014}'))]
    EmDash,

    #[index(Some('\u{2212}'))]
    Minus,

    #[index(Some('\u{002D}'))]
    Hyphen,
}
impl SpecialUnicodeChar {
    pub fn char(&self) -> Option<char> {
        self.index()
    }

    pub fn find_char(c: char) -> Option<Self> {
        Self::from_index(&Some(c))
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
