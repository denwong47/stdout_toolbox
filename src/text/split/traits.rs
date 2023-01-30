use std::mem::swap;
use crate::modifiers::LengthWithoutModifiers;

use lazy_static::lazy_static;
use regex::Regex;

use super::*;

lazy_static! {
    static ref MODIFIER_PATTERN: Regex = {
        let mut breaking_chars = String::from(r#"^(?P<word>.*?)(?P<sep>["#);
        breaking_chars.push_str(&SpecialUnicodeChar::all_non_breaking_chars());
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
    type Item = (String, SpecialUnicodeChar);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(subtext) = self.text.get(self.pos..) {
            if let Some(item) = MODIFIER_PATTERN.captures(subtext) {
                self.pos += item["word"].len() + 1;
                return Some((item["word"].to_owned(), SpecialUnicodeChar::find_char(item["sep"].chars().next().unwrap()).unwrap_or(SpecialUnicodeChar::Nothing)))
            } else {
                // Last word without an ending sep
                self.pos += subtext.len() + 1;
                return Some((subtext.to_owned(), SpecialUnicodeChar::Nothing))
            }
        } else {
            return None
        }
    }
}

pub struct SplitLines<'t>{
    iter: SplitWords<'t>,
    cached: String,
    line_length: usize,
}
impl<'t> Iterator for SplitLines<'t> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = String::new();
        swap(&mut result, &mut self.cached);

        if result.len_without_modifiers() >= self.line_length-1 {
            self.cached = result.get(self.line_length-1..).unwrap().to_owned();
            return Some(result.get(..self.line_length-1).unwrap().to_owned() + "-")
        }
        loop {
            if let Some((next_word, next_sep)) = self.iter.next() {
                let new_len = {
                    result.len_without_modifiers()
                    + next_word.len_without_modifiers()
                    + (next_sep.is_needed_end_of_line() as usize)
                };

                if new_len > self.line_length {
                    // Line full
                    self.cached = String::from(next_word);
                    next_sep.append_to(&mut self.cached);

                    break
                } else if next_sep.is_new_line() {
                    // Line not full, but new line detected
                    result.push_str(&next_word);
                    next_sep.append_to(&mut result);
                    break
                } else {
                    // Line still have space, continue
                    result.push_str(&next_word);
                    next_sep.append_to(&mut result);
                }
            } else {
                break
            }
        }

        match result.len() {
            0 => None,
            _ => Some(result)
        }
    }
}

pub trait DissembleString{
    fn iter_words(&self) -> SplitWords;
    fn iter_lines(&self, line_length: usize) -> SplitLines;
}
impl DissembleString for &str {
    fn iter_words(&self) -> SplitWords {
        return SplitWords::from(*self)
    }

    fn iter_lines(&self, line_length: usize) -> SplitLines {
        return SplitLines {
            iter: self.iter_words(),
            cached: String::new(),
            line_length,
        }
    }
}