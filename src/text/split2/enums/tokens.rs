use std::ops::Index;

use super::chars::TokenSeparator;
use crate::modifiers::*;

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
        todo!()
    }
}
