use std::marker::PhantomData;

use regex;
use regex::Regex;
use strum::IntoEnumIterator;

/// Pair of traits: HasValue and HasResetter. 
pub trait HasValue<T> {
    fn value(&self) -> T;
}
pub trait HasResetter {
    fn resetter(&self) -> Self;
}

impl<U> HasValue<String> for U
where
    U: HasValue<u8>,
{
    fn value(&self) -> String {
        format!("\x1b[{}m", self.value())
    }
}

pub trait SearchValue<T> : IntoEnumIterator+HasValue<T> {
    fn find_value(value: &T) -> Option<Self>;
}
impl<U, T> SearchValue<T> for U
where
    T: PartialEq,
    U: IntoEnumIterator+HasValue<T>
{
    // TODO If we do a proc_macro that map Enum members into values, we can also
    // implement a backward mechanism, rather than a sequential search.
    fn find_value(value: &T) -> Option<Self> {
        Self
        ::iter()
        .find(
            | m | m.value().eq(value)
        )
    }
}

// =====================================================================================

pub struct ModifierIter<'t, T>
{
    text: &'t str,
    pos: usize,
    pattern: Regex,

    _enum_type: PhantomData<T>,
}
impl<'t, T> From<&'t str> for ModifierIter<'t, T> 
where
    T: SearchValue<String>
{
    fn from(value: &'t str) -> Self {
        Self {
            text: value,
            pos: 0,
            pattern: Regex::new(
                T::iter()
                .fold(
                    String::new(),
                    | lhs, rhs | {
                        let sep = match lhs.len() { 0=>"", _=>"|"};
                        lhs + sep + &regex::escape(&rhs.value())
                    }
                )
                .as_str()
            ).unwrap(),

            _enum_type: PhantomData,
        }
    }
}
impl<'t, T> Iterator for ModifierIter<'t, T>
where
    T: SearchValue<String>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(subtext) = self.text.get(self.pos..) {
            if let Some(item) = self.pattern.find(subtext) {
                let mod_str = item.as_str().to_owned();

                self.pos += item.end();

                return T::find_value(&mod_str);
            }
        }

        None
    }
}

/// Trait for enums that Iter with String values.
/// 
/// This allows us to construct an Iterator that iterates through each modifier within
/// a String.
pub trait SearchValueInStr : SearchValue<String> {
    fn iter_member_in_str<'t>(text: &'t str) -> ModifierIter<'t, Self>;
}
impl<U> SearchValueInStr for U
where 
    U: SearchValue<String>,
    Self: SearchValue<String>,
{
    fn iter_member_in_str<'t>(text: &'t str) -> ModifierIter<'t, Self> {
        ModifierIter::from(text)
    }
}