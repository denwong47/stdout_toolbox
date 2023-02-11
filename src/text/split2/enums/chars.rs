use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use enum_index::*;

type OptionalChar = Option<char>;

#[derive(EnumIndex, EnumIter, Debug, PartialEq)]
#[index_type(OptionalChar)]
pub enum TokenSeparator {
    #[index(None)]
    Nothing,

    #[index(Some('\0'))]
    Null,

    #[index(Some('\u{0005}'))]
    Eof,

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
/// Allow all variants to
impl ToString for TokenSeparator {
    fn to_string(&self) -> String {
        if let Some(c) = self.index() {
            c.to_string()
        } else {
            String::new()
        }
    }
}
