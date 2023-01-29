use crate::modifiers::LengthWithoutModifiers;

use lazy_static::lazy_static;
use regex::{Match, Regex, Split};

use super::*;

lazy_static! {
    static ref MODIFIER_PATTERN: Regex = {
        let mut breaking_chars = String::from(r#"^(?P<word>.*?)(?P<sep>["#);
        breaking_chars.push(TAB);
        breaking_chars.push(LINE_FEED);
        breaking_chars.push(CARRIAGE_RETURN);
        breaking_chars.push(SPACE);
        // breaking_chars.push(NO_BREAK_SPACE);
        breaking_chars.push(OGHAM_SPACE_MARK);
        breaking_chars.push(EN_QUAD);
        breaking_chars.push(EM_QUAD);
        breaking_chars.push(EN_SPACE);
        breaking_chars.push(EM_SPACE);
        breaking_chars.push(THREE_PER_EM_SPACE);
        breaking_chars.push(FOUR_PER_EM_SPACE);
        breaking_chars.push(SIX_PER_EM_SPACE);
        breaking_chars.push(FIGURE_SPACE);
        breaking_chars.push(PUNCTUATION_SPACE);
        breaking_chars.push(THIN_SPACE);
        breaking_chars.push(HAIR_SPACE);
        breaking_chars.push(LINE_SEPARATOR);
        breaking_chars.push(PARAGRAPH_SEPARATOR);
        // breaking_chars.push(NARROW_NO_BREAK_SPACE);
        breaking_chars.push(MEDIUM_MATH_SPACE);
        breaking_chars.push(IDEOGRAPHIC_SPACE);
        breaking_chars.push(EN_DASH);
        breaking_chars.push(EM_DASH);
        breaking_chars.push(MINUS);

        // Last one to avoid it being read as a Regex Modifier
        breaking_chars.push(HYPHEN);
        breaking_chars.push_str(r#"])"#);

        Regex::new(&breaking_chars).unwrap()
    };
}

pub struct SplitWords<'t>{
    text: &'t str,
    pos: usize,
}
impl<'t> From<&'t str> for SplitWords<'t> {
    fn from(value: &'t str) -> Self {
        Self {
            text: value,
            pos: 0,
        }
    }
}
impl<'t> Iterator for SplitWords<'t> {
    type Item = (String, Option<char>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(subtext) = self.text.get(self.pos..) {
            if let Some(item) = MODIFIER_PATTERN.captures(subtext) {
                self.pos += item["word"].len() + 1;
                return Some((item["word"].to_owned(), Some(item["sep"].chars().next().unwrap())))
            } else {
                // Last word without an ending sep
                return Some((subtext.to_owned(), None))
            }
        } else {
            return None
        }
    }
}

pub trait DissembleString{
    fn iter_words(&self) -> SplitWords;
}
impl DissembleString for &str {
    fn iter_words(&self) -> SplitWords {
        return SplitWords::from(*self)
    }
}