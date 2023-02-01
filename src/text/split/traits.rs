/// NEED TO REWRITE THIS ENTIRELY
///
/// Probably have an enum along the lines of
/// 
/// pub enum WordToken {
///     Text(String),
///     Modifier(Box<ANSIModifier>),
///     ConnectingChar(char),
///     EOF,
/// }
/// 
/// and iter over that rather than (word, sep).
/// 
/// Enums should implement a From and Into their .value() type, so find_value()
/// doesn't need to iterate.

use std::mem::swap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::modifiers::*;

use super::*;

lazy_static! {
    static ref MODIFIER_PATTERN: Regex = {
        let mut breaking_chars = String::from(r#"^(?P<word>.*?)(?P<sep>["#);
        breaking_chars.push_str(&SpecialUnicodeChar::all_non_breaking_chars());
        breaking_chars.push_str(r#"])"#);

        Regex::new(&breaking_chars).unwrap()
    };
}

pub struct SplitWords<'t> {
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
                return Some((
                    item["word"].to_owned(),
                    SpecialUnicodeChar::find_char(item["sep"].chars().next().unwrap())
                        .unwrap_or(SpecialUnicodeChar::Nothing),
                ));
            } else {
                // Last word without an ending sep
                self.pos += subtext.len() + 1;
                return Some((subtext.to_owned(), SpecialUnicodeChar::Nothing));
            }
        } else {
            return None;
        }
    }
}

pub struct SplitLines<'t> {
    iter: SplitWords<'t>,
    cached: String,
    line_length: usize,
}
impl<'t> Iterator for SplitLines<'t> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter_exhausted: bool = false;
        let mut result = String::new();
        swap(&mut result, &mut self.cached);

        if result.len_without_modifiers() >= self.line_length - 1 {
            // Bug - result.get below uses len WITH modifiers
            let range_wrapper = RangeWithoutModifiers::new(&result);
            let idx_with_modifiers = range_wrapper.index_with_modifiers(self.line_length - 1);

            self.cached = result.get(idx_with_modifiers..).unwrap_or("").to_owned();

            result = result.get(..idx_with_modifiers).unwrap().to_owned();
            if result
                .chars()
                .last()
                .map(|c| !c.is_whitespace())
                .unwrap_or(false)
            {
                result += "-";
            }
        }

        let last_char = {
            result
                .chars()
                .last()
                .and_then(|c| SpecialUnicodeChar::find_char(c))
        };

        if last_char.map(|m| m.is_new_line()).unwrap_or(false) {
            // This can occur when
            // - "A long sentence that finishes with a\n" gets full after "with",
            // - then "a\n" will enter self.cache, but it shall not be appended to
            //   because the next time `next` is called, we should just return "a".
            // - that's why we need to check blank lines here.
            drop(result.pop()); // We don't need the new line char itself.
        } else {
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
                        break;
                    } else if next_sep.is_new_line() {
                        // Line not full, but new line detected
                        result.push_str(&next_word);

                        if next_sep.is_needed_end_of_line() {
                            next_sep.append_to(&mut result);
                        }
                        break;
                    } else {
                        // Line still have space, continue
                        result.push_str(&next_word);
                        next_sep.append_to(&mut result);
                    }
                } else {
                    iter_exhausted = true;
                    break;
                }
            }
        }

        if result.len() > 0 || !iter_exhausted {
            if let Some(last_char) = result.pop() {
                if let Some(m) = SpecialUnicodeChar::find_char(last_char) {
                    if m.is_needed_end_of_line() {
                        // If we need it, push it back
                        result.push(last_char)
                    }
                } else {
                    // If its not a special character, put it back
                    result.push(last_char)
                }

                // Check for the last modifiers, and if found, reset it,
                macro_rules! sub_members {
                    ($enum_name:ident) => {
                        if let Some(last_mod) = $enum_name::iter_member_in_str(&result).last() {
                            if last_mod != last_mod.resetter() {
                                result.push_str(
                                    {
                                        format!(
                                            "{:width$}",
                                            "",
                                            width =
                                                self.line_length - result.len_without_modifiers()
                                        ) + HasValue::<String>::value(&last_mod.resetter()).as_str()
                                    }
                                    .as_str(),
                                );
                                self.cached = HasValue::<String>::value(&last_mod) + &self.cached;
                            }
                        }
                    };
                }

                sub_members!(ForegroundColours);
                sub_members!(BackgroundColours);
                sub_members!(Intensity);
            }

            Some(result)
        } else {
            None
        }
    }
}

pub trait DissembleString {
    fn iter_words(&self) -> SplitWords;
    fn iter_lines(&self, line_length: usize) -> SplitLines;
}
impl DissembleString for &str {
    fn iter_words(&self) -> SplitWords {
        return SplitWords::from(*self);
    }

    fn iter_lines(&self, line_length: usize) -> SplitLines {
        return SplitLines {
            iter: self.iter_words(),
            cached: String::new(),
            line_length,
        };
    }
}
