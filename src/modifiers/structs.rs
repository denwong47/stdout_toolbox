use super::*;

pub struct ANSIModifiers
{
    pub applicator: String,
    pub resetter: String,
}
impl HasValue<String> for ANSIModifiers {
    fn value(&self) -> String {
        self.applicator.clone()
    }
}
impl HasResetter for ANSIModifiers {
    fn resetter(&self) -> Self {
        Self {
            applicator: self.resetter.clone(),
            resetter: String::new(),
        }
    }
}
#[allow(dead_code)]
impl ANSIModifiers {
    fn new() -> Self {
        return Self {
            applicator: String::new(),
            resetter: String::new(),
        }
    }

    pub fn from<T>(obj: T) -> Self
    where T: HasValue<String>+HasResetter {
        return Self {
            applicator: obj.value(),
            resetter: obj.resetter().value(),
        }
    }

    pub fn join<T>(mut self, rhs: T) -> Self 
    where T: HasValue<String>+HasResetter
    {
        self.applicator += &rhs.value();
        self.resetter = rhs.resetter().value() + &self.resetter;

        self
    }

    pub fn wrapper<T>(self)
    -> impl Fn(&T) -> String
    where   T: ToString {
        move | s: &T | {
            self.wraps(s)
        }
    }
}