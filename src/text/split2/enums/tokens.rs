use super::chars::TokenSeparator;
use crate::modifiers::*;

#[allow(dead_code)]
pub enum StringToken {
    Word(String),
    Separator(TokenSeparator),
    Intensity(Intensity),
    Foreground(ForegroundColours),
    Background(BackgroundColours),
    Control(MoveCursor),
}

pub struct StringTokenStream<'t> {
    text: &'t str,
    pos: usize,
}
impl StringTokenStream<'_> {
    fn segment(&self) -> Option<&str> {
        return self.text.get(self.pos..);
    }
}
impl<'t> From<&'t str> for StringTokenStream<'t> {
    fn from(value: &'t str) -> Self {
        Self {
            text: value,
            pos: 0,
        }
    }
}
impl<'t> Iterator for StringTokenStream<'t> {
    type Item = StringToken;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(seg) = self.segment() {
            let mut word: Option<String> = None;

            let mut char_stream = seg.chars();

            loop {
                if let Some(ch) = char_stream.next() {
                    if let Ok(sep) = TokenSeparator::try_from(&Some(ch)) {
                        if word.is_some() {
                            // If there are
                            break;
                        } else {
                            self.pos += 1;
                            return Some(Self::Item::Separator(sep));
                        }
                    } else if ch == '\x1b' {
                        todo!()
                    } else {
                        let word_value = word.get_or_insert(String::new());
                        word_value.push(ch);
                    }
                } else {
                    break;
                }
            }

            let word_len = word
                .as_ref()
                .and_then(|s| if s.len() > 0 { Some(s.len()) } else { None });

            if word_len.is_some() {
                self.pos += word_len.unwrap();
                Some(Self::Item::Word(word.unwrap_or_default()))
            } else {
                None
            }
        } else {
            None
        }
    }
}
