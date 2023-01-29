use duplicate::duplicate_item;

#[duplicate_item(
    __name__                __char__;
    [ TAB ]                 [ '\u{0009}' ];
    [ LINE_FEED ]           [ '\u{000A}' ];
    [ CARRIAGE_RETURN ]     [ '\u{000D}' ];
    [ SPACE ]               [ '\u{0020}' ];
    [ NO_BREAK_SPACE ]      [ '\u{00A0}' ]; // DO NOT BREAK THIS
    [ OGHAM_SPACE_MARK ]    [ '\u{1680}' ];
    [ EN_QUAD ]             [ '\u{2000}' ];
    [ EM_QUAD ]             [ '\u{2001}' ];
    [ EN_SPACE ]            [ '\u{2002}' ];
    [ EM_SPACE ]            [ '\u{2003}' ];
    [ THREE_PER_EM_SPACE ]  [ '\u{2004}' ];
    [ FOUR_PER_EM_SPACE ]   [ '\u{2005}' ];
    [ SIX_PER_EM_SPACE ]    [ '\u{2006}' ];
    [ FIGURE_SPACE ]        [ '\u{2007}' ]; // DO NOT BREAK THIS
    [ PUNCTUATION_SPACE ]   [ '\u{2008}' ];
    [ THIN_SPACE ]          [ '\u{2009}' ];
    [ HAIR_SPACE ]          [ '\u{200A}' ];
    [ LINE_SEPARATOR ]      [ '\u{2028}' ];
    [ PARAGRAPH_SEPARATOR ] [ '\u{2029}' ];
    [ NARROW_NO_BREAK_SPACE][ '\u{202F}' ];
    [ MEDIUM_MATH_SPACE ]   [ '\u{205F}' ];
    [ IDEOGRAPHIC_SPACE ]   [ '\u{3000}' ];
    [ EN_DASH ]             [ '\u{2013}' ];
    [ EM_DASH ]             [ '\u{2014}' ];
    [ MINUS ]               [ '\u{2212}' ];

    [ HYPHEN ]              [ '\u{002D}' ];
)]
#[allow(dead_code)]
pub static __name__:char = __char__;