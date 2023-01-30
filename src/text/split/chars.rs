use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum SpecialUnicodeChar {
    Tab,
    LineFeed,
    CarriageReturn,
    SPACE,
    NoBreakSpace,
    OghamSpaceMark,
    EnQuad,
    EmQuad,
    EnSpace,
    EmSpace,
    ThreePerEmSpace,
    FourPerEmSpace,
    SixPerEmSpace,
    FigureSpace,
    PunctuationSpace,
    ThinSpace,
    HairSpace,
    LineSeparator,
    ParagraphSeparator,
    NarrowNoBreakSpace,
    MediumMathSpace,
    IdeographicSpace,

    EnDash,
    EmDash,
    Minus,
    Hyphen,

    Nothing,
}
impl SpecialUnicodeChar {
    pub fn char(&self) -> Option<char> {
        match self {
            Self::Nothing => None,

            Self::Tab => Some('\u{0009}'),
            Self::LineFeed => Some('\u{000A}'),
            Self::CarriageReturn => Some('\u{000D}'),
            Self::SPACE => Some('\u{0020}'),
            Self::NoBreakSpace => Some('\u{00A0}'),
            Self::OghamSpaceMark => Some('\u{1680}'),
            Self::EnQuad => Some('\u{2000}'),
            Self::EmQuad => Some('\u{2001}'),
            Self::EnSpace => Some('\u{2002}'),
            Self::EmSpace => Some('\u{2003}'),
            Self::ThreePerEmSpace => Some('\u{2004}'),
            Self::FourPerEmSpace => Some('\u{2005}'),
            Self::SixPerEmSpace => Some('\u{2006}'),
            Self::FigureSpace => Some('\u{2007}'),
            Self::PunctuationSpace => Some('\u{2008}'),
            Self::ThinSpace => Some('\u{2009}'),
            Self::HairSpace => Some('\u{200A}'),
            Self::LineSeparator => Some('\u{2028}'),
            Self::ParagraphSeparator => Some('\u{2029}'),
            Self::NarrowNoBreakSpace => Some('\u{202F}'),
            Self::MediumMathSpace => Some('\u{205F}'),
            Self::IdeographicSpace => Some('\u{3000}'),

            Self::EnDash => Some('\u{2013}'),
            Self::EmDash => Some('\u{2014}'),
            Self::Minus => Some('\u{2212}'),
            Self::Hyphen => Some('\u{002D}'),
        }
    }
    
    pub fn find_char(c: char) -> Option<Self> {
        Self::iter().find(| m | m.char().as_ref() == Some(&c))
    }

    pub fn all_non_breaking_chars() -> String {
        Self::iter()
            .filter(| m | !m.is_breaking())
            .fold(String::new(), | mut lhs, rhs | {
                match rhs.char() {
                    Some(c) => lhs.push(c),
                    None => ()
                }

                lhs
            })
    }

    pub fn is_breaking(&self) -> bool {
        match self {
            Self::NoBreakSpace
            | Self::NarrowNoBreakSpace
            | Self::FigureSpace
            | Self::Nothing => true,
            _ => false,
        }
    }

    pub fn is_new_line(&self) -> bool {
        match self {
            Self::CarriageReturn
            | Self::LineFeed
            | Self::LineSeparator => true,
            _ => false,
        }
    }

    pub fn is_needed_end_of_line(&self) -> bool {
        match self {
            Self::EnDash
            | Self::EmDash
            | Self::Minus
            | Self::Hyphen => true,
            _ => false,
        }
    }

    pub fn append_to(&self, s:&mut String) {
        match self.char() {
            Some(c) => s.push(c),
            None => (),
        }
    }
}