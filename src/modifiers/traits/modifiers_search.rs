use lazy_static::lazy_static;
use regex::{Matches, Regex};

lazy_static! {
    static ref MODIFIER_PATTERN: Regex = Regex::new(r#"\x1b\[(?:\d+[;:])*\d+[A-Za-z]"#).unwrap();
}

pub struct ModifiersInText<'r, 't>(Matches<'r, 't>);
impl<'r, 't> ModifiersInText<'r, 't> {
    pub fn new(s: &'t str) -> Self {
        return Self(MODIFIER_PATTERN.find_iter(s));
    }
}
impl<'r, 't> Iterator for ModifiersInText<'r, 't> {
    type Item = &'t str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|m| m.as_str())
    }
}

pub trait FindModifiers {
    fn iter_modifier_strs(&self) -> ModifiersInText;
    fn len(&self) -> usize;
}
impl FindModifiers for &str {
    fn iter_modifier_strs(&self) -> ModifiersInText {
        ModifiersInText::new(self)
    }

    fn len(&self) -> usize {
        return str::len(self);
    }
}
impl FindModifiers for String {
    fn iter_modifier_strs(&self) -> ModifiersInText {
        ModifiersInText::new(&self)
    }

    fn len(&self) -> usize {
        return self.len();
    }
}

pub trait LengthWithoutModifiers: FindModifiers {
    fn len_without_modifiers(&self) -> usize;
}
impl<T> LengthWithoutModifiers for T
where
    T: FindModifiers,
{
    fn len_without_modifiers(&self) -> usize {
        let modifier_count = {
            self.iter_modifier_strs()
                .fold(0_usize, |count, matched| count + matched.len())
        };

        return self.len() - modifier_count;
    }
}